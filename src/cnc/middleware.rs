use super::types::sched_types::{
    ConfigurableGateParameterTableEntry, GateControlEntry, GateControlOperation,
};
use super::types::scheduling::{Config, PortConfiguration, Schedule};
use super::types::topology::Topology;
use super::types::uni_types::Domain;
use super::Cnc;
use std::{sync::Weak, thread, time::Duration};

pub trait SchedulerAdapterInterface {
    fn compute_schedule(&self, topology: &Topology, domains: &Vec<Domain>) -> Schedule;

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

pub struct IPVSDsyncTSNScheduling {
    // TODO impl sched-algo
    cnc: Weak<Cnc>,
}

impl IPVSDsyncTSNScheduling {
    pub fn new() -> Self {
        Self {
            cnc: Weak::default(),
        }
    }

    pub fn compute(&self, _topology: &Topology, _domains: &Vec<Domain>) -> Vec<(u32, u32)> {
        // TODO call Algorithm
        thread::sleep(Duration::from_secs(10));

        return vec![(1, 1000)];
    }

    pub fn parse_to_schedule(&self, starts: Vec<(u32, u32)>) -> Schedule {
        let mut configs: Vec<Config> = Vec::new();
        let mut ports: Vec<PortConfiguration> = Vec::new();

        ports.push(PortConfiguration {
            name: String::from("sw0p2"),
            config: ConfigurableGateParameterTableEntry {
                gate_enable: true,
                admin_gate_states: 255,
                admin_control_list: vec![GateControlEntry {
                    operation_name: GateControlOperation::SetGateStates,
                    time_interval_value: 320000,
                    gate_state_value: 255,
                }],
                admin_cycle_time: (320000, 1000000000),
                admin_cycle_time_extension: 0,
                admin_base_time: (starts[0].1 as u64, 0),
                config_change: true,
            },
        });

        ports.push(PortConfiguration {
            name: String::from("sw0p3"),
            config: ConfigurableGateParameterTableEntry {
                gate_enable: true,
                admin_gate_states: 255,
                admin_control_list: vec![GateControlEntry {
                    operation_name: GateControlOperation::SetGateStates,
                    time_interval_value: 320000,
                    gate_state_value: 255,
                }],
                admin_cycle_time: (320000, 1000000000),
                admin_cycle_time_extension: 0,
                admin_base_time: (starts[0].1 as u64, 0),
                config_change: true,
            },
        });

        configs.push(Config { node_id: 1, ports });

        return Schedule { configs };
    }
}

impl SchedulerAdapterInterface for IPVSDsyncTSNScheduling {
    fn compute_schedule(&self, topology: &Topology, domains: &Vec<Domain>) -> Schedule {
        // TODO call sched-algo
        // todo!("compute schedule");

        let starts = self.compute(topology, domains);

        return self.parse_to_schedule(starts);
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }
}
