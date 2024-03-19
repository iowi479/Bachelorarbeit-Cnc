use super::types::sched_types::{
    ConfigurableGateParameterTableEntry, GateControlEntry, GateControlOperation,
};
use super::types::scheduling::{Config, PortConfiguration, Schedule};
use super::types::topology::Topology;
use super::types::tsn_types::{
    ConfigListElement, ConfigValue, DataFrameSpecificationElementType, InterfaceListElement,
};
use super::types::uni_types::Domain;
use super::types::ComputationResult;
use super::Cnc;
use std::sync::Weak;

pub trait SchedulerAdapterInterface {
    /// This function will compute the schedule for the given topology and domains.
    /// This can take some time, depending on what algorithms are used.
    fn compute_schedule(&self, topology: &Topology, domains: &Vec<Domain>) -> ComputationResult;

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

pub struct MockTSNScheduler {
    cnc: Weak<Cnc>,
}

impl MockTSNScheduler {
    pub fn new() -> Self {
        Self {
            cnc: Weak::default(),
        }
    }

    /// returnes a fake configuration. All in there is hardcoded and specifically for the topology in the TopologyComponent.
    pub fn compute_fake(&self, topology: &Topology, domains: &Vec<Domain>) -> ComputationResult {
        let acc_latency = 50000;

        let mut domains = domains.clone(); // copy since we do modifications to it

        for domain in domains.iter_mut() {
            for cuc in domain.cuc.iter_mut() {
                for stream in cuc.stream.iter_mut() {
                    // configure rest of the stream...
                    // talker object
                    stream
                        .talker
                        .group_status_talker_listener
                        .accumulated_latency = acc_latency;

                    stream
                        .talker
                        .group_status_talker_listener
                        .interface_configuration
                        .interface_list = Vec::new();

                    let interface_id = stream.talker.group_talker.end_station_interfaces[0].clone();

                    let mut interface_list_element = InterfaceListElement {
                        group_interface_id: interface_id,
                        config_list: Vec::new(),
                    };

                    // copy interfaceconfigs
                    for config in stream.talker.group_talker.data_frame_specification.iter() {
                        match &config.field {
                            DataFrameSpecificationElementType::Ieee802MacAddresses(x) => {
                                interface_list_element.config_list.push(ConfigListElement {
                                    index: config.index,
                                    config_value: ConfigValue::Ieee802MacAddresses(x.clone()),
                                })
                            }
                            DataFrameSpecificationElementType::Ieee802VlanTag(x) => {
                                interface_list_element.config_list.push(ConfigListElement {
                                    index: config.index,
                                    config_value: ConfigValue::Ieee802VlanTag(x.clone()),
                                })
                            }
                            DataFrameSpecificationElementType::Ipv4Tuple(x) => {
                                interface_list_element.config_list.push(ConfigListElement {
                                    index: config.index,
                                    config_value: ConfigValue::Ipv4Tuple(x.clone()),
                                })
                            }
                            DataFrameSpecificationElementType::Ipv6Tuple(x) => {
                                interface_list_element.config_list.push(ConfigListElement {
                                    index: config.index,
                                    config_value: ConfigValue::Ipv6Tuple(x.clone()),
                                })
                            }
                        }
                    }

                    stream
                        .talker
                        .group_status_talker_listener
                        .interface_configuration
                        .interface_list
                        .push(interface_list_element);

                    // listener objects
                    for listener in stream.listener.iter_mut() {
                        listener.group_status_talker_listener.accumulated_latency = acc_latency;
                        listener
                            .group_status_talker_listener
                            .interface_configuration
                            .interface_list = Vec::new();

                        let interface_id =
                            listener.group_listener.end_station_interfaces[0].clone();
                        let interface_list_element = InterfaceListElement {
                            group_interface_id: interface_id,
                            // apply same configuration as to talker...
                            config_list: stream
                                .talker
                                .group_status_talker_listener
                                .interface_configuration
                                .interface_list[0]
                                .config_list
                                .clone(),
                        };

                        listener
                            .group_status_talker_listener
                            .interface_configuration
                            .interface_list
                            .push(interface_list_element);
                    }
                }
            }
        }

        let mut bridges: Vec<u32> = Vec::new();
        for ele in topology.nodes.iter() {
            if ele.id < 10 {
                // is a bridge
                bridges.push(ele.id);
            }
        }

        let result = ComputationResult {
            schedule: self.parse_to_schedule(bridges, topology),
            domains,                    // modified domains
            failed_streams: Vec::new(), // no failed streams
        };

        return result;
    }

    /// this generates a fake-hardcoded configuration for the hardcoded topology...
    /// This doesnt work anymore if the topology changes.
    pub fn parse_to_schedule(&self, bridges: Vec<u32>, _topology: &Topology) -> Schedule {
        let mut configs: Vec<Config> = Vec::new();

        // configure node (1)
        if bridges.iter().filter(|x| x == &&1u32).count() > 0 {
            configs.push(Config {
                node_id: 1,
                port: PortConfiguration {
                    name: String::from("sw0p2"),
                    mac_address: String::from("00-60-65-82-c9-5b"),

                    config: ConfigurableGateParameterTableEntry {
                        gate_enable: true,
                        admin_gate_states: 255,
                        admin_control_list: vec![GateControlEntry {
                            operation_name: GateControlOperation::SetGateStates,
                            time_interval_value: 320000,
                            gate_state_value: 255,
                        }],
                        admin_cycle_time: (100 * 3200, 1000000000),
                        admin_cycle_time_extension: 0,
                        admin_base_time: (0, 0),
                        config_change: true,
                    },
                },
                affected_streams: vec![
                    String::from("00-00-00-00-00-01:00-01"),
                    String::from("00-00-00-00-00-01:00-02"),
                ],
            });

            configs.push(Config {
                node_id: 1,

                port: PortConfiguration {
                    name: String::from("sw0p3"),
                    mac_address: String::from("00-60-65-82-c9-5c"),
                    config: ConfigurableGateParameterTableEntry {
                        gate_enable: true,
                        admin_gate_states: 255,
                        admin_control_list: vec![GateControlEntry {
                            operation_name: GateControlOperation::SetGateStates,
                            time_interval_value: 320000,
                            gate_state_value: 255,
                        }],
                        admin_cycle_time: (100 * 3200, 1000000000),
                        admin_cycle_time_extension: 0,
                        admin_base_time: (0, 0),
                        config_change: true,
                    },
                },
                affected_streams: vec![
                    String::from("00-00-00-00-00-01:00-01"),
                    String::from("00-00-00-00-00-02:00-03"),
                ],
            });

            configs.push(Config {
                node_id: 1,
                port: PortConfiguration {
                    name: String::from("sw0p4"),
                    mac_address: String::from("00-60-65-82-c9-5d"),
                    config: ConfigurableGateParameterTableEntry {
                        gate_enable: true,
                        admin_gate_states: 255,
                        admin_control_list: vec![GateControlEntry {
                            operation_name: GateControlOperation::SetGateStates,
                            time_interval_value: 640000,
                            gate_state_value: 255,
                        }],
                        admin_cycle_time: (100 * 6400, 1000000000),
                        admin_cycle_time_extension: 0,
                        admin_base_time: (0, 0),
                        config_change: true,
                    },
                },
                affected_streams: vec![
                    String::from("00-00-00-00-00-01:00-02"),
                    String::from("00-00-00-00-00-02:00-03"),
                ],
            });
        }

        //configure node (2)
        if bridges.iter().filter(|x| x == &&2u32).count() > 0 {
            configs.push(Config {
                node_id: 2,
                port: PortConfiguration {
                    name: String::from("sw0p2"),
                    mac_address: String::from("00-10-02-00-02-02"),
                    config: ConfigurableGateParameterTableEntry {
                        gate_enable: true,
                        admin_gate_states: 255,
                        admin_control_list: vec![GateControlEntry {
                            operation_name: GateControlOperation::SetGateStates,
                            time_interval_value: 320000,
                            gate_state_value: 255,
                        }],
                        admin_cycle_time: (100 * 3200, 1000000000),
                        admin_cycle_time_extension: 0,
                        admin_base_time: (0, 0),
                        config_change: true,
                    },
                },
                affected_streams: vec![
                    String::from("00-00-00-00-00-01:00-02"),
                    String::from("00-00-00-00-00-02:00-03"),
                ],
            });

            configs.push(Config {
                node_id: 2,
                port: PortConfiguration {
                    name: String::from("sw0p3"),
                    mac_address: String::from("00-10-02-00-02-03"),
                    config: ConfigurableGateParameterTableEntry {
                        gate_enable: true,
                        admin_gate_states: 255,
                        admin_control_list: vec![GateControlEntry {
                            operation_name: GateControlOperation::SetGateStates,
                            time_interval_value: 320000,
                            gate_state_value: 255,
                        }],
                        admin_cycle_time: (100 * 3200, 1000000000),
                        admin_cycle_time_extension: 0,
                        admin_base_time: (0, 0),
                        config_change: true,
                    },
                },
                affected_streams: vec![
                    String::from("00-00-00-00-00-01:00-02"),
                    String::from("00-00-00-00-00-02:00-03"),
                ],
            });
        }

        return Schedule { configs };
    }
}

impl SchedulerAdapterInterface for MockTSNScheduler {
    fn compute_schedule(&self, topology: &Topology, domains: &Vec<Domain>) -> ComputationResult {
        let result = self.compute_fake(topology, domains);
        return result;
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }
}
