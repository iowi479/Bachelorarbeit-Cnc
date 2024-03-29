use self::netconf::{establish_netconf_connection, put_configurations_in_dtree};
use self::types::NetconfConnection;
use super::types::lldp_types::RemoteSystemsData;
use super::types::scheduling::{PortConfiguration, Schedule};
use super::types::topology::{Port, SSHConfigurationParams, Topology};
use super::types::tsn_types::GroupInterfaceId;
use super::types::{FailedInterface, FailedInterfaces};
use super::Cnc;
use crate::cnc::southbound::netconf::{
    edit_config_in_candidate, extract_port_delays, extract_remote_systems_data,
    get_config_interfaces, get_interface_data, get_lldp_remote_systems_data,
};
use crate::cnc::types::scheduling::Config;
use netconf_client::errors::NetconfClientError;
use std::collections::HashMap;
use std::sync::Weak;

mod netconf;
pub mod types;

pub trait SouthboundControllerInterface {}

pub trait SouthboundAdapterInterface {
    /// configures the network.
    ///
    /// if configurations failed, they are provided in the FailedInterfaces to handle on the cnc side.
    fn configure_network(&self, topology: &Topology, schedule: &Schedule) -> FailedInterfaces;

    /// this configures a node-port on the given client
    fn configure_node(
        &self,
        netconf_connection: &mut NetconfConnection,
        config: &PortConfiguration,
    ) -> Result<(), NetconfClientError>;

    /// requests the bridge-delay parameter of a specific bridge
    fn retrieve_station_capibilities(&self, config_params: SSHConfigurationParams) -> Vec<Port>;

    /// requests the lldp parameter of a specific bridge
    fn retrieve_lldp(&self, config_params: SSHConfigurationParams) -> Vec<RemoteSystemsData>;

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
    fn configure_network(&self, topology: &Topology, schedule: &Schedule) -> FailedInterfaces {
        let mut configured_nodes: HashMap<u32, NetconfConnection> = HashMap::new();
        let mut node_configurations: HashMap<u32, Vec<Config>> = HashMap::new();

        let mut failed_interfaces = FailedInterfaces {
            interfaces: Vec::new(),
        };

        'configloop: for configuration in schedule.configs.iter() {
            // check if connection is already established
            if let Some(netconf_connection) = configured_nodes.get_mut(&configuration.node_id) {
                let config_result = self.configure_node(netconf_connection, &configuration.port);
                if config_result.is_ok() {
                    node_configurations
                        .get_mut(&configuration.node_id)
                        .unwrap()
                        .push(configuration.clone());
                    continue 'configloop;
                }
            } else {
                let node = topology.get_node_from_id(configuration.node_id);

                if let Some(node) = node {
                    let config_params = node.configuration_params.unwrap();

                    match establish_netconf_connection(&config_params) {
                        Err(e) => {
                            eprintln!("[Southbound] error while connecting via netconf {e:?}");
                        }
                        Ok(mut netconf_connection) => {
                            let config_result =
                                self.configure_node(&mut netconf_connection, &configuration.port);
                            if config_result.is_ok() {
                                configured_nodes.insert(configuration.node_id, netconf_connection);
                                node_configurations
                                    .insert(configuration.node_id, vec![configuration.clone()]);
                                continue 'configloop;
                            }
                        }
                    }
                }
            }

            // if the program gets here while not continuing, something went wrong while configuring
            failed_interfaces.interfaces.push(FailedInterface {
                node_id: configuration.node_id,
                interface: GroupInterfaceId {
                    interface_name: configuration.port.name.clone(),
                    mac_address: configuration.port.mac_address.clone(),
                },
                affected_streams: configuration
                    .affected_streams
                    .iter()
                    .map(|x| x.clone())
                    .collect(),
            });
        }

        if failed_interfaces.interfaces.len() == 0 {
            for (node_id, netconf_connection) in configured_nodes.iter_mut() {
                let commit_result = netconf_connection.netconf_client.commit();

                match commit_result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("[Southbound] error while committing: {e:?}");

                        for config in node_configurations.get(node_id).unwrap() {
                            failed_interfaces.interfaces.push(FailedInterface {
                                node_id: node_id.clone(),
                                interface: GroupInterfaceId {
                                    interface_name: config.port.name.clone(),
                                    mac_address: config.port.mac_address.clone(),
                                },
                                affected_streams: config
                                    .affected_streams
                                    .iter()
                                    .map(|x| x.clone())
                                    .collect(),
                            });
                        }
                    }
                }

                if let Err(e) = netconf_connection.netconf_client.close_session() {
                    eprintln!("[Southbound] Error while closing connection... {:?}", e);
                }
            }
        } else {
            eprintln!("[Southbound] not comitting since there where configuration failures...");
        }

        failed_interfaces
    }

    fn retrieve_station_capibilities(&self, config_params: SSHConfigurationParams) -> Vec<Port> {
        if let Ok(mut netconf_connection) = establish_netconf_connection(&config_params) {
            if let Ok(dtree) = get_interface_data(&mut netconf_connection) {
                if let Err(e) = netconf_connection.netconf_client.close_session() {
                    eprintln!("[Southbound] Error while closing netconf session: {:?}", e);
                }

                return extract_port_delays(&dtree, &netconf_connection.yang_paths);
            }
            eprintln!("[Southbound] couldnt parse datatree...");
        } else {
            eprintln!("[Southbound] couldnt connect to bridge...");
        }
        Vec::new()
    }

    fn retrieve_lldp(&self, config_params: SSHConfigurationParams) -> Vec<RemoteSystemsData> {
        if let Ok(mut netconf_connection) = establish_netconf_connection(&config_params) {
            if let Ok(tree) = get_lldp_remote_systems_data(&mut netconf_connection) {
                if let Err(e) = netconf_connection.netconf_client.close_session() {
                    eprintln!("[Southbound] Error while closing netconf session: {:?}", e);
                }

                return extract_remote_systems_data(&tree, &netconf_connection.yang_paths);
            }
            eprintln!("[Southbound] couldnt parse datatree...");
        } else {
            eprintln!("[Southbound] couldnt connect to bridge...");
        }

        Vec::new()
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }

    fn configure_node(
        &self,
        netconf_connection: &mut NetconfConnection,
        port_configuration: &PortConfiguration,
    ) -> Result<(), NetconfClientError> {
        match get_config_interfaces(netconf_connection) {
            Ok(mut netconf_configuration) => {
                put_configurations_in_dtree(
                    &mut netconf_configuration,
                    &netconf_connection.yang_paths,
                    port_configuration,
                );
                edit_config_in_candidate(netconf_connection, &netconf_configuration)
            }
            Err(e) => Err(e),
        }
    }
}
