use std::{net::IpAddr, sync::Weak};

use crate::cnc::{topology::Port, types::tsn_types::BridgePortDelays};

use super::{middleware::Schedule, topology::Topology, Cnc};

pub trait SouthboundControllerInterface {}
pub trait SouthboundAdapterInterface {
    fn configure_network(&self, topology: &Topology, schedule: &Schedule);
    fn retrieve_station_capibilities(&self, ip: String) -> Vec<Port>;

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

pub struct NetconfAdapter {
    cnc: Weak<Cnc>,
}

impl NetconfAdapter {
    pub fn new() -> Self {
        Self {
            cnc: Weak::default(),
        }
    }
}

impl SouthboundAdapterInterface for NetconfAdapter {
    fn configure_network(&self, topology: &Topology, schedule: &Schedule) {
        let mut configured_nodes: Vec<IpAddr> = Vec::new();
        for config in schedule.configs.iter() {
            if let Some(node) = topology.get_node(config.node_id) {
                println!("[Southbound] <edit-config> on {}", node.ip);
                for port_config in config.ports.iter() {
                    // TODO impl netconf
                    println!("\t port: {} ", port_config.name);

                    if true {
                        configured_nodes.push(node.ip);
                    }
                }
            }
        }

        if configured_nodes.len() == schedule.configs.len() {
            for ip in configured_nodes.iter() {
                // TODO impl netconf
                println!("[Southbound] <commit> on {}", ip.to_string());
            }
        }
    }

    fn retrieve_station_capibilities(&self, ip: String) -> Vec<Port> {
        // TODO impl retrieve capabilites
        // may not be possible atm using netconf

        println!("[Southbound] <get> bridge-port-delays of {ip}");

        let mut ports: Vec<Port> = Vec::new();

        ports.push(Port {
            name: String::from("sn0p2"),
            delays: vec![
                BridgePortDelays {
                    port_speed: 100,
                    dependent_rx_delay_min: 80000,
                    dependent_rx_delay_max: 80000,
                    independent_rx_delay_min: 374,
                    independent_rx_delay_max: 384,
                    independent_rly_delay_min: 610,
                    independent_rly_delay_max: 1350,
                    independent_tx_delay_min: 2210,
                    independent_tx_delay_max: 3882,
                },
                BridgePortDelays {
                    port_speed: 1000,
                    dependent_rx_delay_min: 80000,
                    dependent_rx_delay_max: 80000,
                    independent_rx_delay_min: 326,
                    independent_rx_delay_max: 336,
                    independent_rly_delay_min: 486,
                    independent_rly_delay_max: 1056,
                    independent_tx_delay_min: 994,
                    independent_tx_delay_max: 2658,
                },
            ],
        });
        ports.push(Port {
            name: String::from("sn0p3"),
            delays: vec![
                BridgePortDelays {
                    port_speed: 100,
                    dependent_rx_delay_min: 80000,
                    dependent_rx_delay_max: 80000,
                    independent_rx_delay_min: 374,
                    independent_rx_delay_max: 384,
                    independent_rly_delay_min: 610,
                    independent_rly_delay_max: 1350,
                    independent_tx_delay_min: 2210,
                    independent_tx_delay_max: 3882,
                },
                BridgePortDelays {
                    port_speed: 1000,
                    dependent_rx_delay_min: 80000,
                    dependent_rx_delay_max: 80000,
                    independent_rx_delay_min: 326,
                    independent_rx_delay_max: 336,
                    independent_rly_delay_min: 486,
                    independent_rly_delay_max: 1056,
                    independent_tx_delay_min: 994,
                    independent_tx_delay_max: 2658,
                },
            ],
        });

        return ports;
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }
}
