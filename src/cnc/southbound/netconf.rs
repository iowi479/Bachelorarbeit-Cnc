use netconf_client::{
    errors::NetconfClientError,
    models::requests::{Filter, FilterType},
    netconf_client::NetconfClient,
};
use std::sync::Arc;
use yang2::{
    context::{Context, ContextFlags},
    data::{Data, DataFormat, DataParserFlags, DataTree, DataValidationFlags},
    schema::DataValue,
};

use crate::cnc::types::{topology::Port, tsn_types::BridgePortDelays};

use super::types::YangModule;

const SEARCH_DIR: &str = "./assets/yang/";
const YANG_MODULES: &'static [YangModule] = &[
    YangModule::new("ietf-interfaces"), // downloaded because the switch didnt return it...
    YangModule::new("ietf-yang-types"),
    YangModule::new("iana-if-type"),
    YangModule::new("ieee802-types"),
    YangModule::new("ieee802-dot1q-bridge"),
    YangModule::new("ieee802-dot1q-types"),
    YangModule::new("ieee802-dot1q-bridge-delays"),
    YangModule::new_with_features("ieee802-dot1q-sched", &["scheduled-traffic"]),
];

pub fn get_netconf_connection(
    ip: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<NetconfClient, NetconfClientError> {
    let mut client = NetconfClient::new(ip, port, username, password);

    client.connect()?;
    client.send_hello()?;

    Ok(client)
}

pub fn get_interface_data(
    client: &mut NetconfClient,
    ctx: &Arc<Context>,
) -> Result<DataTree, Box<dyn std::error::Error>> {
    let get_interfaces_filter = Filter {
        filter_type: FilterType::Subtree,
        data: "<interfaces xmlns=\"urn:ietf:params:xml:ns:yang:ietf-interfaces\">
                    <interface>
                
                        <gate-parameters xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-sched\">
                        </gate-parameters>

                        <bridge-port xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-bridge\">
                        </bridge-port>
                    </interface>
                </interfaces>"
            .to_string(),
    };

    let response = client.get(Some(get_interfaces_filter))?;

    let dtree = DataTree::parse_string(
        ctx,
        response.data.expect("no data").as_str(),
        DataFormat::XML,
        DataParserFlags::NO_VALIDATION,
        DataValidationFlags::empty(),
    )?;

    Ok(dtree)
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

    Arc::new(ctx)
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
            delays: Vec::new(),
        };

        for bpdnode in dnode
            .find_xpath((path + "/bridge-port/bridge-port-delays").as_str())
            .expect("no bpd nodes found")
        {
            let mut delays = BridgePortDelays::new();

            for child_node in bpdnode.children() {
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

    ports
}
