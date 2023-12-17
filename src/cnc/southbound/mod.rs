use self::netconf::{create_yang_context, get_config_interfaces, put_config_in_dtree};
use super::types::scheduling::Schedule;
use super::types::topology::{Port, SSHConfigurationParams, Topology};
use super::Cnc;
use crate::cnc::southbound::netconf::{
    edit_config_in_candidate, get_interface_data, get_netconf_connection, get_port_delays,
};
use netconf_client::netconf_client::NetconfClient;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Weak;
use yang2::context::Context;

mod netconf;
mod types;

pub trait SouthboundControllerInterface {}
pub trait SouthboundAdapterInterface {
    /// configures the network.
    ///
    /// returns a list of node_ids. Each of the ids represents, that the configuration for this node failed.
    fn configure_network(&self, topology: &Topology, schedule: &Schedule) -> HashSet<u32>;

    /// requests the bridge-delay parameter of a specific bridge
    fn retrieve_station_capibilities(&self, config_params: SSHConfigurationParams) -> Vec<Port>;

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
}

impl SouthboundAdapterInterface for NetconfAdapter {
    fn configure_network(&self, topology: &Topology, schedule: &Schedule) -> HashSet<u32> {
        let mut configured_nodes: Vec<(u32, NetconfClient)> = Vec::new();
        let mut failed_nodes: HashSet<u32> = HashSet::new();

        for config in schedule.configs.iter() {
            if let Some(node) = topology.get_node(config.node_id) {
                let config_params = node.configuration_params.clone().unwrap();

                println!(
                    "[Southbound] Connecting to {} via Netconf",
                    &config_params.ip
                );

                let connection_result = get_netconf_connection(config_params);

                match connection_result {
                    Err(e) => eprintln!("[Southbound] error while connecting via netconf {e:?}"),
                    Ok(mut client) => {
                        println!("[Southbound] successfully connected to switch");
                        if let Ok(mut netconf_configuration) =
                            get_config_interfaces(&mut client, &self.yang_ctx)
                        {
                            // print_whole_datatree(&netconf_configuration);
                            put_config_in_dtree(&mut netconf_configuration, &config.ports);
                            // print_whole_datatree(&netconf_configuration);
                            let config_result =
                                edit_config_in_candidate(&mut client, &netconf_configuration);

                            match config_result {
                                Err(e) => {
                                    failed_nodes.insert(config.node_id);
                                    eprintln!("[Southbound] failed while configuring {:?}", e)
                                }
                                Ok(_) => configured_nodes.push((config.node_id, client)),
                            }
                        }
                    }
                }
            }
        }

        if configured_nodes.len() == schedule.configs.len() {
            for (node_id, client) in configured_nodes.iter_mut() {
                let commit_result = client.commit();

                match commit_result {
                    Ok(_) => {
                        println!("[Southbound] commit successful")
                    }
                    Err(e) => {
                        failed_nodes.insert(node_id.clone());
                        eprintln!("[Southbound] error while committing: {e:?}");
                    }
                }

                if let Err(e) = client.close_session() {
                    eprintln!("[Southbound] Error while closing connection... {:?}", e);
                }
            }
        }

        failed_nodes.iter().map(|x| x.clone()).collect()
    }

    fn retrieve_station_capibilities(&self, config_params: SSHConfigurationParams) -> Vec<Port> {
        if let Ok(mut client) = get_netconf_connection(config_params) {
            if let Ok(dtree) = get_interface_data(&mut client, &self.yang_ctx) {
                if let Err(e) = client.close_session() {
                    eprintln!("[Southbound] Error while closing netconf session: {:?}", e);
                }

                return get_port_delays(&dtree);
            } else {
                eprintln!("[Southbound] couldnt parse datatree...");
            }
        } else {
            eprintln!("[Southbound] couldnt connect to bridge...");
        }
        return Vec::new();
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }
}
