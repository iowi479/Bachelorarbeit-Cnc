use super::types::YangModule;
use crate::cnc::types::lldp_types::{ManagementAddress, RemoteSystemsData};
use crate::cnc::types::scheduling::PortConfiguration;
use crate::cnc::types::topology::{Port, SSHConfigurationParams};
use crate::cnc::types::tsn_types::BridgePortDelays;
use netconf_client::errors::NetconfClientError;
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

/// all yang-models to load have to be included here.
const YANG_MODULES: &'static [YangModule] = &[
    YangModule::new("ietf-interfaces"), // downloaded because the switch didnt return it...
    YangModule::new("ietf-yang-types"), // rest is downloaded from the b&r switch
    YangModule::new("iana-if-type"),
    YangModule::new("ieee802-types"),
    YangModule::new("ieee802-dot1q-bridge"),
    YangModule::new("ieee802-dot1q-types"),
    YangModule::new("ieee802-dot1q-bridge-delays"),
    YangModule::new_with_features("ieee802-dot1q-sched", &["scheduled-traffic"]),
    YangModule::new("ietf-routing"),
    YangModule::new("ieee802-dot1ab-types"),
    YangModule::new("ieee802-dot1ab-lldp"),
];

pub fn get_netconf_connection(
    config_params: &SSHConfigurationParams,
) -> Result<NetconfClient, NetconfClientError> {
    let mut client = NetconfClient::new(
        config_params.ip.as_str(),
        config_params.port,
        config_params.username.as_str(),
        config_params.password.as_str(),
    );

    client.connect()?;
    client.send_hello()?;

    return Ok(client);
}

/// this runs a <get-config> rpc on the netconf-client. This will provied all configurable
/// fields to edit and commit in the end.
pub fn get_config_interfaces(
    client: &mut NetconfClient,
    ctx: &Arc<Context>,
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

    let response = client.get_config(
        netconf_client::models::requests::DatastoreType::Candidate,
        Some(get_config_interfaces_filter),
    )?;

    let data = response.data.expect("no data in dtree");

    let dtree = DataTree::parse_string(
        ctx,
        data.as_str(),
        DataFormat::XML,
        DataParserFlags::NO_VALIDATION,
        DataValidationFlags::empty(),
    )
    .expect("couldnt parse data");

    return Ok(dtree);
}

/// the provided configurations will be loaded into the given dtree. If the nodes dont already exist,
/// they will be created. If they exist with different values, they will be overriden.
pub fn put_config_in_dtree(dtree: &mut DataTree, port_configuration: &PortConfiguration) {
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

/// this is for debugging. Can be unused...
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
    client: &mut NetconfClient,
    dtree: &DataTree,
) -> Result<(), NetconfClientError> {
    let data = dtree
        .print_string(
            DataFormat::XML,
            DataPrinterFlags::WD_ALL | DataPrinterFlags::WITH_SIBLINGS,
        )
        .expect("couldnt parse datatree")
        .expect("no data");

    let _res = client.edit_config(
        netconf_client::models::requests::DatastoreType::Candidate,
        data,
        Some(netconf_client::models::requests::DefaultOperationType::Merge),
        Some(netconf_client::models::requests::TestOptionType::TestThenSet),
        Some(netconf_client::models::requests::ErrorOptionType::RollbackOnError),
    )?;

    Ok(())
}

pub fn get_lldp_data(
    client: &mut NetconfClient,
    ctx: &Arc<Context>,
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

    let response = client.get(Some(get_lldp_filter))?;

    let data = response.data.expect("no data in dtree");

    println!("{}", data);

    let dtree = DataTree::parse_string(
        ctx,
        data.as_str(),
        DataFormat::XML,
        DataParserFlags::NO_VALIDATION,
        DataValidationFlags::empty(),
    )
    .expect("couldnt parse data");

    return Ok(dtree);
}

pub fn get_interface_data(
    client: &mut NetconfClient,
    ctx: &Arc<Context>,
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

    let response = client.get(Some(get_interfaces_filter))?;

    let data = response.data.expect("no data in dtree");

    let dtree = DataTree::parse_string(
        ctx,
        data.as_str(),
        DataFormat::XML,
        DataParserFlags::NO_VALIDATION,
        DataValidationFlags::empty(),
    )
    .expect("couldnt parse data");

    return Ok(dtree);
}

/// Initialize context for working with the yang models. This only gets called on startup.
pub fn create_yang_context() -> Arc<Context> {
    let mut ctx =
        Context::new(ContextFlags::NO_YANGLIBRARY).expect("Failed to create yang-context");
    ctx.set_searchdir(SEARCH_DIR)
        .expect("Failed to set YANG search directory");

    // Load YANG modules.
    for module in YANG_MODULES {
        ctx.load_module(module.name, module.revision, module.features)
            .expect("Failed to load module");
    }

    return Arc::new(ctx);
}

fn interface_name_from_xpath(xpath: &str) -> String {
    let parts = xpath
        .split("[name='")
        .last()
        .expect("failed on first name split");

    let part = parts
        .split("']")
        .next()
        .expect("failed on second name split");

    return part.to_string();
}

fn last_node_name_from_xpath(xpath: &String) -> &str {
    xpath.split("/").last().unwrap().trim()
}

pub fn get_remote_systems_data(dtree: &DataTree) -> Vec<RemoteSystemsData> {
    let mut remote_systems: Vec<RemoteSystemsData> = Vec::new();

    for dnode in dtree
        .find_xpath("/ieee802-dot1ab-lldp:lldp/port/remote-systems-data")
        .expect("no remote-systems-data found")
    {
        let mut system = RemoteSystemsData::new();

        for child_node in dnode.children() {
            let path = child_node.path();
            let node_name = last_node_name_from_xpath(&path);
            println!("node_name: {}", node_name);

            let value = child_node.value();

            if let Some(value) = value {
                match value {
                    DataValue::Other(v) => match node_name {
                        "chassis-id-subtype" => system.chassis_id_subtype = v,
                        "chassis-id" => system.chassis_id = v,
                        "port-id-subtype" => system.port_id_subtype = v,
                        "port-id" => system.port_id = v,
                        "port-description" => system.port_desc = v,
                        "system-name" => system.system_name = v,
                        "system-description" => system.system_description = v,
                        "system-capabilities-supported" => system.system_capabilities_supported = v,
                        "system-capabilities-enabled" => system.system_capabilities_enabled = v,
                        _ => eprintln!("unknown node found in dtree..."),
                    },
                    DataValue::Uint32(v) => match node_name {
                        "time-mark" => system.time_mark = v,
                        "remote-index" => system.remote_index = v,
                        _ => eprintln!("unknown node found in dtree..."),
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
                            "address-type" => address.address_subtype = value.to_string(),
                            "address" => address.address = value.to_string(),
                            _ => eprintln!("unknown key found in management-address..."),
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

pub fn get_port_delays(dtree: &DataTree) -> Vec<Port> {
    let mut ports: Vec<Port> = Vec::new();

    for dnode in dtree
        .find_xpath("/ietf-interfaces:interfaces/interface")
        .expect("no iterfaces found")
    {
        let path = dnode.path();
        let name = interface_name_from_xpath(path.as_str());

        let mut port = Port {
            name,
            mac_address: String::new(),
            delays: Vec::new(),
            tick_granularity: 0,
        };

        for address_node in dnode
            .find_xpath((path.clone() + "/bridge-port/address").as_str())
            .expect("no address field found")
        {
            if let Some(value) = address_node.value() {
                match value {
                    DataValue::Other(v) => port.mac_address = v,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        }

        for tick_node in dnode
            .find_xpath((path.clone() + "/gate-parameters/tick-granularity").as_str())
            .expect("no tick-granularity field found")
        {
            if let Some(value) = tick_node.value() {
                match value {
                    DataValue::Uint32(tick) => port.tick_granularity = tick,
                    _ => eprintln!("found an unexpected node in dtree"),
                }
            }
        }

        for bpd_node in dnode
            .find_xpath((path + "/bridge-port/bridge-port-delays").as_str())
            .expect("no bpd nodes found")
        {
            let mut delays = BridgePortDelays::new();

            for child_node in bpd_node.children() {
                let path = child_node.path();
                let node_name = last_node_name_from_xpath(&path);

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
