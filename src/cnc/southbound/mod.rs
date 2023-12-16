use crate::cnc::southbound::netconf::{
    get_interface_data, get_netconf_connection, get_port_delays,
};

use self::netconf::create_yang_context;

use super::cnc::Cnc;
use super::types::scheduling::{Config, Schedule};
use super::types::topology::{NodeInformation, Port, Topology};
use netconf_client::errors::NetconfClientError;
use std::sync::Arc;
use std::{net::IpAddr, sync::Weak};
use yang2::context::Context;

mod netconf;
mod types;

pub trait SouthboundControllerInterface {}
pub trait SouthboundAdapterInterface {
    fn configure_network(&self, topology: &Topology, schedule: &Schedule);
    fn retrieve_station_capibilities(
        &self,
        ip: &str,
        ssh_port: u16,
        ssh_username: &str,
        ssh_password: &str,
    ) -> Vec<Port>;

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

pub struct NetconfAdapter {
    cnc: Weak<Cnc>,
    yang_ctx: Arc<Context>,
}

impl NetconfAdapter {
    pub fn new() -> Self {
        Self {
            cnc: Weak::default(),
            yang_ctx: create_yang_context(),
        }
    }

    fn configure_node(
        &self,
        node: &NodeInformation,
        config: &Config,
    ) -> Result<(), NetconfClientError> {
        println!("[Soutgbound] <edit-config> on {}", node.id);

        for port_config in &config.ports {
            println!("\tport: {}", port_config.name);
        }

        Ok(())
    }
}

impl SouthboundAdapterInterface for NetconfAdapter {
    fn configure_network(&self, topology: &Topology, schedule: &Schedule) {
        let mut configured_nodes: Vec<IpAddr> = Vec::new();
        for config in schedule.configs.iter() {
            if let Some(node) = topology.get_node(config.node_id) {
                let config_result = self.configure_node(&node, config);

                if config_result.is_ok() {
                    configured_nodes.push(node.ip);
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

    fn retrieve_station_capibilities(
        &self,
        ip: &str,
        ssh_port: u16,
        ssh_username: &str,
        ssh_password: &str,
    ) -> Vec<Port> {
        if let Ok(mut client) = get_netconf_connection(ip, ssh_port, ssh_username, ssh_password) {
            if let Ok(dtree) = get_interface_data(&mut client, &self.yang_ctx) {
                return get_port_delays(&dtree);
            } else {
                eprintln!("couldnt parse datatree...");
            }
        } else {
            eprintln!("couldnt connect to bridge...");
        }
        return Vec::new();
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }
}
