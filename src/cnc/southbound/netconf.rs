use super::types::{NetconfConnection, YangModule, YangPaths, YANG_MODULES};
use crate::cnc::types::lldp_types::{ManagementAddress, RemoteSystemsData};
use crate::cnc::types::scheduling::PortConfiguration;
use crate::cnc::types::topology::{Port, SSHConfigurationParams};
use crate::cnc::types::tsn_types::BridgePortDelays;
use netconf_client::errors::NetconfClientError;
use netconf_client::models::replies::HelloServer;
use netconf_client::models::requests::{Filter, FilterType};
use netconf_client::netconf_client::NetconfClient;
use std::sync::Arc;
use yang2::context::{Context, ContextFlags};
use yang2::data::{
    Data, DataFormat, DataParserFlags, DataPrinterFlags, DataTree, DataValidationFlags,
};
use yang2::schema::DataValue;

/// folder for all needed yang-models
const SEARCH_DIR: &str = "./assets/yang/";

/// Initialize context for working with the correct yang models. This is unique for each Switch
/// since the Modules  might differ.
pub fn init_yang_ctx(yang_modules: &Vec<YangModule>) -> Arc<Context> {
    let mut ctx =
        Context::new(ContextFlags::NO_YANGLIBRARY).expect("Failed to create yang-context");
    ctx.set_searchdir(SEARCH_DIR)
        .expect("failed to set search directory to find yang-models");

    // Load YANG modules.
    for module in yang_modules {
        ctx.load_module(module.name, module.revision, module.features)
            .expect("failed to load yang-module");
    }

    Arc::new(ctx)
}

/// this extracts the yang_modules form the <hello>-Message
/// TODO: this is not implemented yet. The used B&R switch doesn't provide all needed models.
pub fn extract_used_yang_modules(hello_server: &HelloServer) -> Vec<YangModule> {
    // TODO: load yangmodules based on the provided capabilities.
    // Since the used B&R switch doesnt provide all needed Models, these are hardcoded here.
    let _capabilities = &hello_server.capabilities;
    let yang_modules: Vec<YangModule> = YANG_MODULES.to_vec();
    // ----------

    yang_modules
}

/// this function establishes a connection to the netconf-server. It will load all needed yang-models
pub fn establish_netconf_connection(
    config_params: &SSHConfigurationParams,
) -> Result<NetconfConnection, NetconfClientError> {
    let mut netconf_client = NetconfClient::new(
        config_params.ip.as_str(),
        config_params.port,
        config_params.username.as_str(),
        config_params.password.as_str(),
    );

    let hello_server = netconf_client.connect()?;
    let yang_modules: Vec<YangModule> = extract_used_yang_modules(&hello_server);

    netconf_client.send_hello()?;

    let netconf_connection = NetconfConnection {
        netconf_client,
        yang_ctx: init_yang_ctx(&yang_modules),
        yang_paths: YangPaths::load_paths(&yang_modules),
    };

    Ok(netconf_connection)
}

/// this runs a <get-config> rpc on the netconf-client. This will provied all configurable
/// fields to edit and commit in the end.
pub fn get_config_interfaces(
    netconf_connection: &mut NetconfConnection,
) -> Result<DataTree, NetconfClientError> {
    let get_config_interfaces_filter = Filter {
        filter_type: FilterType::Subtree,
        data: netconf_connection
            .yang_paths
            .filters
            .gate_parameters
            .clone(),
    };

    let get_config_response = netconf_connection.netconf_client.get_config(
        netconf_client::models::requests::DatastoreType::Candidate,
        Some(get_config_interfaces_filter),
    )?;

    let response_data = get_config_response.data.expect(
        "Requested <gate-parameters> of interfaces but didn't receive any data to be parsed",
    );

    let dtree = DataTree::parse_string(
        &netconf_connection.yang_ctx,
        response_data.as_str(),
        DataFormat::XML,
        DataParserFlags::NO_VALIDATION,
        DataValidationFlags::empty(),
    )
    .expect("data for <gate-parameters> was received but it couldn't be parsed based on the provided yang-models");

    Ok(dtree)
}

/// the provided configurations will be loaded into the given dtree. If the nodes dont already exist,
/// they will be created. If they exist with different values, they will be overriden.
pub fn put_configurations_in_dtree(
    dtree: &mut DataTree,
    yang_paths: &YangPaths,
    port_configuration: &PortConfiguration,
) {
    // path-example: /ietf-interfaces:interfaces/interface[name='eth0']/ieee802-dot1q-sched:gate-parameters
    let mut port_xpath: String = String::from("/");
    port_xpath.push_str(&yang_paths.params.interfaces_by_name);
    port_xpath = port_xpath.replace("{}", &port_configuration.name);
    port_xpath.push_str("/");
    port_xpath.push_str(&yang_paths.params.gate_parameters);

    let config = &port_configuration.config;

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.gate_enabled,
        config.gate_enable.to_string().as_str(),
    );

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.admin_gate_states,
        config.admin_gate_states.to_string().as_str(),
    );

    // admin-control-list
    for (i, gce) in config.admin_control_list.iter().enumerate() {
        let operation_name = match gce.operation_name {
            crate::cnc::types::sched_types::GateControlOperation::SetGateStates => {
                "set-gate-states"
            }
            _ => panic!("not supported"),
        };

        let path_prefix = yang_paths
            .params
            .admin_control_list_by_index
            .replace("{}", &i.to_string());

        put_gate_parameters_in_dtree(
            dtree,
            port_xpath.clone(),
            &(path_prefix.clone() + &yang_paths.params.operation_name),
            operation_name,
        );
        put_gate_parameters_in_dtree(
            dtree,
            port_xpath.clone(),
            &(path_prefix.clone() + &yang_paths.params.sgs_params_gate_states_value),
            &gce.gate_state_value.to_string(),
        );
        put_gate_parameters_in_dtree(
            dtree,
            port_xpath.clone(),
            &(path_prefix.clone() + &yang_paths.params.sgs_params_time_interval_value),
            &gce.time_interval_value.to_string(),
        );
    }

    if config.admin_control_list.len() == 0 {
        // this should empty the list but not sure... test
        // TODO does this work?
        if let Err(e) = dtree
            .remove((port_xpath.clone() + &yang_paths.params.admin_control_list_length).as_str())
        {
            eprintln!("[Southbound] couldnt remove admin-control-list: {:?}", e);
        }
    }

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.admin_control_list_length,
        &config.admin_control_list.len().to_string(),
    );
    // ---

    // admin-cycle-time
    let cycle_time = config.admin_cycle_time;
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.admin_cycle_time_numerator,
        &cycle_time.0.to_string(),
    );
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.admin_cycle_time_denominator,
        &cycle_time.1.to_string(),
    );
    // ---

    // admin-base-time
    let basetime = config.admin_base_time;
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.admin_base_time_seconds,
        &basetime.0.to_string(),
    );
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.admin_base_time_fractional_seconds,
        &basetime.1.to_string(),
    );
    // ---

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.admin_cycle_time_extension,
        &config.admin_cycle_time_extension.to_string(),
    );

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        &yang_paths.params.config_change,
        &config.config_change.to_string(),
    );
}

/// puts the in path specified node at xpath into the dtree. The value to insert can be provided as well.
/// If the path doesnt exist, it gets created. Also nodes before which dont exist will be created.
fn put_gate_parameters_in_dtree(dtree: &mut DataTree, port_xpath: String, path: &str, value: &str) {
    let config_path = port_xpath + "/" + path;
    let config_path = config_path.as_str();

    dtree
        .new_path(config_path, Some(value), false)
        .expect(format!("[Southbound] couldnt configure node {} in dtree...", path).as_str());
}

/// this is for debugging.
/// prints dtree in XML format to stdout
#[allow(unused)]
pub fn print_whole_datatree(dtree: &DataTree) {
    dtree
        .print_file(
            std::io::stdout(),
            DataFormat::XML,
            DataPrinterFlags::WD_ALL | DataPrinterFlags::WITH_SIBLINGS,
        )
        .expect("Failed to log dtree to stdout");
}

pub fn edit_config_in_candidate(
    netconf_connection: &mut NetconfConnection,
    dtree: &DataTree,
) -> Result<(), NetconfClientError> {
    let data = dtree
        .print_string(
            DataFormat::XML,
            DataPrinterFlags::WD_ALL | DataPrinterFlags::WITH_SIBLINGS,
        )
        .expect("couldnt parse datatree")
        .expect("no data");

    let res = netconf_connection.netconf_client.edit_config(
        netconf_client::models::requests::DatastoreType::Candidate,
        data,
        Some(netconf_client::models::requests::DefaultOperationType::Merge),
        Some(netconf_client::models::requests::TestOptionType::TestThenSet),
        Some(netconf_client::models::requests::ErrorOptionType::RollbackOnError),
    )?;

    // TODO: check if the response is ok
    // or if something can be extracted
    dbg!(res);

    Ok(())
}

pub fn get_lldp_remote_systems_data(
    netconf_connection: &mut NetconfConnection,
) -> Result<DataTree, NetconfClientError> {
    let get_lldp_filter = Filter {
        filter_type: FilterType::Subtree,
        data: netconf_connection
            .yang_paths
            .filters
            .remote_systems_data
            .clone(),
    };

    let response = netconf_connection
        .netconf_client
        .get(Some(get_lldp_filter))?;
    let data = response.data.expect("no data in dtree");
    let dtree = DataTree::parse_string(
        &netconf_connection.yang_ctx,
        data.as_str(),
        DataFormat::XML,
        DataParserFlags::NO_VALIDATION,
        DataValidationFlags::empty(),
    )
    .expect("got lldp data but couldn't parse data");

    Ok(dtree)
}

pub fn get_interface_data(
    netconf_connection: &mut NetconfConnection,
) -> Result<DataTree, NetconfClientError> {
    let get_interfaces_filter = Filter {
        filter_type: FilterType::Subtree,
        data: netconf_connection
            .yang_paths
            .filters
            .gate_parameters_and_bridge_ports
            .clone(),
    };

    let response = netconf_connection
        .netconf_client
        .get(Some(get_interfaces_filter))?;

    let data = response.data.expect("no data in dtree");

    let dtree = DataTree::parse_string(
        &netconf_connection.yang_ctx,
        data.as_str(),
        DataFormat::XML,
        DataParserFlags::NO_VALIDATION,
        DataValidationFlags::empty(),
    )
    .expect("couldnt parse data");

    Ok(dtree)
}

/// helper function to extract the interface name from an xpath
///
/// this only works, if a single attribute is provided in the xpath and this one is named 'name'
///
/// # Example
///
/// "/ietf-interfaces:interfaces/interface[name='eth0']/ieee802-dot1q-sched:gate-parameters" -> "eth0"
fn extract_interface_name_from_xpath(xpath: &str) -> String {
    let name_plus_rest = xpath
        .split("interface[name='")
        .last()
        .expect("provided xpath for interface name is not valid. Failed on first name split");

    let only_name = name_plus_rest
        .split("']")
        .next()
        .expect("provided xpath for interface name is not valid. Failed on second name split");

    String::from(only_name)
}

/// helper function to extract the last node name from an xpath
///
/// # Example
///
/// "/ieee802-dot1ab-lldp:lldp/port/remote-systems-data" -> "remote-systems-data"
fn extract_last_node_name_from_xpath(xpath: &String) -> &str {
    let parts = xpath.split("/");
    let last_node = parts.last().expect("no last node found in xpath");
    last_node
}

pub fn extract_remote_systems_data(
    dtree: &DataTree,
    yang_paths: &YangPaths,
) -> Vec<RemoteSystemsData> {
    let mut remote_systems: Vec<RemoteSystemsData> = Vec::new();
    let remote_systems_path: String = String::from("/") + (&yang_paths.params.remote_systems_data);

    for dnode in dtree
        .find_xpath(&remote_systems_path)
        .expect("no remote-systems-data found")
    {
        let mut system = RemoteSystemsData::new();

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.chassis_id_subtype) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.chassis_id_subtype = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no chassis-id-subtype found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.chassis_id) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.chassis_id = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no chassis-id found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.port_id_subtype) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.port_id_subtype = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no port-id-subtype found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.port_id) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.port_id = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no port-id found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.port_desc) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.port_desc = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no port-desc found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.system_name) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.system_name = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no system-name found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.system_description) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.system_description = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no system-description found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.system_capabilities_supported) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.system_capabilities_supported = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no system-capabilities-supported found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.system_capabilities_enabled) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => system.system_capabilities_enabled = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no system-capabilities-enabled found in dtree")
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.time_mark) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Uint32(v) => system.time_mark = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no time-mark found in dtree");
        };

        if let Ok(child_node) = dnode.find_path(&yang_paths.params.remote_index) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Uint32(v) => system.remote_index = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no remote-index found in dtree");
        };

        if let Ok(management_nodes) = dnode.find_xpath(&yang_paths.params.management_address) {
            for child_node in management_nodes {
                let child_node_path = child_node.path();
                let child_node_name = extract_last_node_name_from_xpath(&child_node_path);
                let params = child_node_name.replace(&yang_paths.params.management_address, "");
                let attribs = params
                    .split("][")
                    .map(|x| {
                        let x = x.replace("[", "");
                        let x = x.replace("]", "");
                        x.replace("'", "")
                    })
                    .collect::<Vec<String>>();

                let mut address: ManagementAddress = ManagementAddress::new();
                for attrib in attribs {
                    let parts = attrib.split("=").collect::<Vec<&str>>();
                    let key = parts[0];
                    let value = parts[1];

                    match key {
                        _ if key == &yang_paths.params.attrib_name_address_subtype => {
                            address.address_subtype = value.to_string()
                        }
                        _ if key == &yang_paths.params.attrib_name_address => {
                            address.address = value.to_string()
                        }
                        _ => eprintln!(
                            "unknown node found in dtree... value: {:?} on {}",
                            value, child_node_name
                        ),
                    }
                }
                system.management_address.push(address);
            }
        }

        remote_systems.push(system);
    }

    remote_systems
}

pub fn extract_port_delays(dtree: &DataTree, yang_paths: &YangPaths) -> Vec<Port> {
    let mut ports: Vec<Port> = Vec::new();
    let interfaces_path: String = String::from("/") + &yang_paths.params.interfaces;

    for interface_dnode in dtree
        .find_xpath(&interfaces_path)
        .expect("no iterfaces found")
    {
        let path = interface_dnode.path();
        let name = extract_interface_name_from_xpath(path.as_str());

        let mut port = Port {
            name,
            mac_address: String::new(),
            delays: Vec::new(),
            tick_granularity: 0,
        };

        if let Ok(child_node) = interface_dnode.find_path(&yang_paths.params.bridge_port_address) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Other(v) => port.mac_address = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no bridge-port-address found in dtree");
        };

        if let Ok(child_node) = interface_dnode.find_path(&yang_paths.params.tick_granularity) {
            if let Some(value) = child_node.value() {
                match value {
                    DataValue::Uint32(v) => port.tick_granularity = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        } else {
            eprintln!("no tick-granularity found in dtree");
        };

        for bridge_port_delays_dnode in interface_dnode
            .find_xpath((path + "/bridge-port/bridge-port-delays").as_str())
            .expect("no bpd nodes found")
        {
            let mut delays = BridgePortDelays::new();

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.port_speed)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint32(v) => delays.port_speed = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no port-speed found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.dependent_rx_delay_min)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.dependent_rx_delay_min = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no dependent-rx-delay-min found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.dependent_rx_delay_max)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.dependent_rx_delay_max = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no dependent-rx-delay-max found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.independent_rx_delay_min)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.independent_rx_delay_min = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no independent-rx-delay-min found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.independent_rx_delay_max)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.independent_rx_delay_max = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no independent-rx-delay-max found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.independent_rly_delay_min)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.independent_rly_delay_min = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no independent-rly-delay-min found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.independent_rly_delay_max)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.independent_rly_delay_max = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no independent-rly-delay-max found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.independent_tx_delay_min)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.independent_tx_delay_min = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no independent-tx-delay-min found in dtree");
            }

            if let Ok(child_node) =
                bridge_port_delays_dnode.find_path(&yang_paths.params.independent_tx_delay_max)
            {
                if let Some(value) = child_node.value() {
                    match value {
                        DataValue::Uint64(v) => delays.independent_tx_delay_max = v,
                        _ => eprintln!("found an unexpected node in dtree"),
                    }
                }
            } else {
                eprintln!("no independent-tx-delay-max found in dtree");
            }

            port.delays.push(delays);
        }

        ports.push(port);
    }

    ports
}
