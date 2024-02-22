use super::types::{NetconfConnection, YangModule};
use crate::cnc::types::lldp_types::{ManagementAddress, RemoteSystemsData};
use crate::cnc::types::scheduling::PortConfiguration;
use crate::cnc::types::topology::{Port, SSHConfigurationParams};
use crate::cnc::types::tsn_types::BridgePortDelays;
use netconf_client::errors::NetconfClientError;
use netconf_client::models::replies::HelloServer;
use netconf_client::models::requests::{Filter, FilterType};
use netconf_client::netconf_client::NetconfClient;
use std::collections::HashMap;
use std::sync::Arc;
use yang2::context::{Context, ContextFlags};
use yang2::data::{
    Data, DataFormat, DataParserFlags, DataPrinterFlags, DataTree, DataValidationFlags,
};
use yang2::schema::DataValue;

/// folder for all needed yang-models
const SEARCH_DIR: &str = "./assets/yang/";

/// all yang-models to load have to be included here.
/// TODO: move this to another file to bundle up all b&r specific stuff
pub const YANG_MODULES: &'static [YangModule] = &[
    YangModule::new_with_features("ietf-interfaces", "2018-02-20", &["if-mib"]),
    YangModule::new("ietf-yang-types", "2013-07-15"),
    YangModule::new("iana-if-type", "2017-01-19"),
    YangModule::new("ieee802-types", "2020-10-23"),
    YangModule::new("ieee802-dot1q-bridge", "2020-11-07"),
    YangModule::new("ieee802-dot1q-types", "2020-10-24"),
    YangModule::new("ieee802-dot1q-bridge-delays", "2021-11-23"),
    YangModule::new("ieee802-dot1q-preemption", "2018-09-10"),
    YangModule::new_with_features("ieee802-dot1q-sched", "2018-09-11", &["scheduled-traffic"]),
    YangModule::new("ietf-routing", "2018-03-13"),
    YangModule::new("ieee802-dot1ab-types", "2018-10-03"),
    YangModule::new("ieee802-dot1ab-lldp", "2018-11-13"),
];

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

/// TODO:
pub fn init_xpath_dict(_yang_modules: &Vec<YangModule>) -> HashMap<String, String> {
    let mut dict: HashMap<String, String> = HashMap::new();

    // TODO: is this really a good idea?
    // This will bloat the extraction of data quite a lot and doesn't really give a benefit right
    // now...
    dict.insert(
        String::from("interface-byname"),
        String::from(
            "/ietf-interfaces:interfaces/interface[name='{}']/ieee802-dot1q-sched:gate-parameters",
        ),
    );

    dict
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
        xpath_dict: init_xpath_dict(&yang_modules),
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
        data: "<interfaces xmlns=\"urn:ietf:params:xml:ns:yang:ietf-interfaces\">
                    <interface>
                        <gate-parameters xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-sched\">
                        </gate-parameters>
                    </interface>
                </interfaces>"
            .to_string(),
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
pub fn put_configurations_in_dtree(dtree: &mut DataTree, port_configuration: &PortConfiguration) {
    let port_xpath = format!(
        "/ietf-interfaces:interfaces/interface[name='{}']/ieee802-dot1q-sched:gate-parameters",
        port_configuration.name
    );
    let config = &port_configuration.config;

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/gate-enabled",
        config.gate_enable.to_string().as_str(),
    );

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/admin-gate-states",
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

        let path_prefix = format!("/admin-control-list[index={}]", i);

        put_gate_parameters_in_dtree(
            dtree,
            port_xpath.clone(),
            (path_prefix.clone() + "/operation-name").as_str(),
            operation_name,
        );
        put_gate_parameters_in_dtree(
            dtree,
            port_xpath.clone(),
            (path_prefix.clone() + "/sgs-params/gate-states-value").as_str(),
            gce.gate_state_value.to_string().as_str(),
        );
        put_gate_parameters_in_dtree(
            dtree,
            port_xpath.clone(),
            (path_prefix.clone() + "/sgs-params/time-interval-value").as_str(),
            gce.time_interval_value.to_string().as_str(),
        );
    }

    if config.admin_control_list.len() == 0 {
        // this should empty the list but not sure... test
        // TODO does this work?
        if let Err(e) = dtree.remove((port_xpath.clone() + "/admin-control-list").as_str()) {
            eprintln!("[Southbound] couldnt remove admin-control-list: {:?}", e);
        }
    }

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/admin-control-list-length",
        config.admin_control_list.len().to_string().as_str(),
    );
    // ---

    // admin-cycle-time
    let cycle_time = config.admin_cycle_time;
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/admin-cycle-time/numerator",
        cycle_time.0.to_string().as_str(),
    );
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/admin-cycle-time/denominator",
        cycle_time.1.to_string().as_str(),
    );
    // ---

    // admin-base-time
    let basetime = config.admin_base_time;
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/admin-base-time/seconds",
        basetime.0.to_string().as_str(),
    );
    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/admin-base-time/fractional-seconds",
        basetime.1.to_string().as_str(),
    );
    // ---

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/admin-cycle-time-extension",
        config.admin_cycle_time_extension.to_string().as_str(),
    );

    put_gate_parameters_in_dtree(
        dtree,
        port_xpath.clone(),
        "/config-change",
        config.config_change.to_string().as_str(),
    );
}

/// puts the in path specified node at xpath into the dtree. The value to insert can be provided as well.
/// If the path doesnt exist, it gets created. Also nodes before which dont exist will be created.
fn put_gate_parameters_in_dtree(dtree: &mut DataTree, port_xpath: String, path: &str, value: &str) {
    let config_path = port_xpath + path;
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
        .expect("Failed to print dtree");
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
        data: "
        <lldp xmlns=\"urn:ieee:std:802.1AB:yang:ieee802-dot1ab-lldp\">
            <port>
                <name></name>
                <remote-systems-data></remote-systems-data>
            </port>
        </lldp>"
            .to_string(),
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
        data: "<interfaces xmlns=\"urn:ietf:params:xml:ns:yang:ietf-interfaces\">
                    <interface>
                        <gate-parameters xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-sched\"></gate-parameters>
                        <bridge-port xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-bridge\"></bridge-port>
                    </interface>
                </interfaces>"
            .to_string(),
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
    xpath.split("/").last().unwrap().trim()
}

pub fn extract_remote_systems_data(dtree: &DataTree) -> Vec<RemoteSystemsData> {
    let mut remote_systems: Vec<RemoteSystemsData> = Vec::new();

    for dnode in dtree
        .find_xpath("/ieee802-dot1ab-lldp:lldp/port/remote-systems-data")
        .expect("no remote-systems-data found")
    {
        let mut system = RemoteSystemsData::new();

        for child_node in dnode.children() {
            let path = child_node.path();
            let node_name = extract_last_node_name_from_xpath(&path);

            let value = child_node.value();

            if let Some(value) = value {
                match value {
                    DataValue::Other(v) => match node_name {
                        "chassis-id-subtype" => system.chassis_id_subtype = v,
                        "chassis-id" => system.chassis_id = v,
                        "port-id-subtype" => system.port_id_subtype = v,
                        "port-id" => system.port_id = v,
                        "port-desc" => system.port_desc = v,
                        "system-name" => system.system_name = v,
                        "system-description" => system.system_description = v,
                        "system-capabilities-supported" => system.system_capabilities_supported = v,
                        "system-capabilities-enabled" => system.system_capabilities_enabled = v,
                        _ => eprintln!(
                            "unknown node found in dtree... value: {:?} on {}",
                            v, node_name
                        ),
                    },
                    DataValue::Uint32(v) => match node_name {
                        "time-mark" => system.time_mark = v,
                        "remote-index" => system.remote_index = v,
                        _ => eprintln!(
                            "unknown node found in dtree... value: {:?} on {}",
                            v, node_name
                        ),
                    },
                    _ => eprintln!(
                        "found an unexpected node in dtree {:?} {}",
                        value, node_name
                    ),
                }
            } else {
                // management-address has no value since the parameters are in the xpath
                if node_name.starts_with("management-address") {
                    let params = node_name.replace("management-address", "");
                    let attribs = params
                        .split("][")
                        .map(|x| {
                            let x = x.replace("[", "");
                            let x = x.replace("]", "");
                            return x.replace("'", "");
                        })
                        .collect::<Vec<String>>();

                    let mut address: ManagementAddress = ManagementAddress::new();
                    for attrib in attribs {
                        let parts = attrib.split("=").collect::<Vec<&str>>();
                        let key = parts[0];
                        let value = parts[1];

                        match key {
                            "address-subtype" => address.address_subtype = value.to_string(),
                            "address" => address.address = value.to_string(),
                            _ => eprintln!(
                                "unknown node found in dtree... value: {:?} on {}",
                                value, node_name
                            ),
                        }
                    }
                    system.management_address.push(address);
                } else {
                    println!("parsing not implemented for {}", node_name);
                }
            }
        }

        remote_systems.push(system);
    }

    return remote_systems;
}

pub fn extract_port_delays(dtree: &DataTree) -> Vec<Port> {
    let mut ports: Vec<Port> = Vec::new();

    for interface_dnode in dtree
        .find_xpath("/ietf-interfaces:interfaces/interface")
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

        for address_dnode in interface_dnode
            .find_xpath((path.clone() + "/bridge-port/address").as_str())
            .expect("no address field found")
        {
            if let Some(value) = address_dnode.value() {
                match value {
                    DataValue::Other(v) => port.mac_address = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        }

        for tick_dnode in interface_dnode
            .find_xpath((path.clone() + "/gate-parameters/tick-granularity").as_str())
            .expect("no tick-granularity field found")
        {
            if let Some(value) = tick_dnode.value() {
                match value {
                    DataValue::Uint32(tick) => port.tick_granularity = tick,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        }

        for bridge_port_delays_dnode in interface_dnode
            .find_xpath((path + "/bridge-port/bridge-port-delays").as_str())
            .expect("no bpd nodes found")
        {
            let mut delays = BridgePortDelays::new();

            for child_node in bridge_port_delays_dnode.children() {
                let path = child_node.path();
                let node_name = extract_last_node_name_from_xpath(&path);

                let value: u32 = match child_node
                    .value()
                    .expect("no value in tree is not possible")
                {
                    DataValue::Uint64(v) => v as u32,
                    DataValue::Uint32(v) => v as u32,
                    _ => 0,
                };

                match node_name {
                    "port-speed" => delays.port_speed = value,
                    "dependentRxDelayMin" => delays.dependent_rx_delay_min = value,
                    "dependentRxDelayMax" => delays.dependent_rx_delay_max = value,
                    "independentRxDelayMin" => delays.independent_rx_delay_min = value,
                    "independentRxDelayMax" => delays.independent_rx_delay_max = value,
                    "independentRlyDelayMin" => delays.independent_rly_delay_min = value,
                    "independentRlyDelayMax" => delays.independent_rly_delay_max = value,
                    "independentTxDelayMin" => delays.independent_tx_delay_min = value,
                    "independentTxDelayMax" => delays.independent_tx_delay_max = value,
                    _ => eprintln!("unknown node found in dtree..."),
                }
            }

            port.delays.push(delays);
        }

        ports.push(port);
    }

    return ports;
}
