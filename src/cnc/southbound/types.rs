use netconf_client::netconf_client::NetconfClient;
use std::sync::Arc;

/// this is used to specify the yang-models that have to be loaded later.
#[derive(Debug, Clone, Copy)]
pub struct YangModule {
    pub name: &'static str,
    pub revision: Option<&'static str>,
    pub features: &'static [&'static str],
}

impl YangModule {
    /// this returnes a YangModule with the name and revision specified.
    pub const fn new(name: &'static str, revision: &'static str) -> Self {
        Self {
            name,
            revision: Some(revision),
            features: &[],
        }
    }

    /// this returnes a YangModule with all info specified.
    pub const fn new_with_features(
        name: &'static str,
        revision: &'static str,
        features: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            revision: Some(revision),
            features,
        }
    }
}

// is used to hold a established netconf_connection as well as the yang_context for parsing the
// exchanged data.
//
// the xpath_dict is used as a lookuptable to find the specified field.
pub struct NetconfConnection {
    pub netconf_client: NetconfClient,
    pub yang_ctx: Arc<yang2::context::Context>,
    pub yang_paths: YangPaths,
}


/// This struct is used to hold all paths and filters that are used to search for specific
/// information in the yang-models.
///
/// This struct acts as a lookup table.
pub struct YangPaths {
    pub filters: SearchFilters,
    pub params: SwitchParameters,
}


impl YangPaths {
    /// this function is used to load the paths and filters for the specified yang_modules.
    pub fn load_paths(_yang_modules: &Vec<YangModule>) -> Self {
        // TODO: make this function dynamically load the paths and filters based on the yang_modules
        Self {
            filters: SearchFilters::load_br_filters(),
            params: SwitchParameters::load_br_paths(),
        }
    }
}

/// This struct contains all paths and attribute names that are used to search for specific
/// information in the yang-models.
///
/// For different Yang-models, there might be different paths and attribute names that are used for
/// the specific information. This struct is used as a lookup table to find the correct path and
/// attribute name.
pub struct SwitchParameters {
    // ietf-interfaces
    pub interfaces: String,
    pub interfaces_by_name: String,

    // ieee802-dot1q-sched
    pub gate_parameters: String,
    pub gate_enabled: String,
    pub admin_gate_states: String,
    pub admin_control_list: String,
    pub admin_control_list_by_index: String,
    pub operation_name: String,
    pub sgs_params_gate_states_value: String,
    pub sgs_params_time_interval_value: String,
    pub admin_control_list_length: String,
    pub admin_cycle_time_numerator: String,
    pub admin_cycle_time_denominator: String,
    pub admin_base_time_seconds: String,
    pub admin_base_time_fractional_seconds: String,
    pub admin_cycle_time_extension: String,
    pub config_change: String,
    pub tick_granularity: String,

    // ieee802-dot1ab-lldp
    pub remote_systems_data: String,
    pub chassis_id_subtype: String,
    pub chassis_id: String,
    pub port_id_subtype: String,
    pub port_id: String,
    pub port_desc: String,
    pub system_name: String,
    pub system_description: String,
    pub system_capabilities_supported: String,
    pub system_capabilities_enabled: String,
    pub time_mark: String,
    pub remote_index: String,
    pub management_address: String,
    pub attrib_name_address_subtype: String,
    pub attrib_name_address: String,

    // ieee802-dot1q-bridge
    pub bridge_port: String,
    pub bridge_port_address: String,

    // ieee802-dot1q-bridge-port-delays
    pub bridge_port_delays: String,
    pub port_speed: String,
    pub dependent_rx_delay_min: String,
    pub dependent_rx_delay_max: String,
    pub independent_rx_delay_min: String,
    pub independent_rx_delay_max: String,
    pub independent_rly_delay_min: String,
    pub independent_rly_delay_max: String,
    pub independent_tx_delay_min: String,
    pub independent_tx_delay_max: String,
}

// TODO: if other configurations are needed, you can add them here.
impl SwitchParameters {
    fn load_br_paths() -> Self {
        Self {
            // ietf-interfaces
            interfaces: "ietf-interfaces:interfaces/interface".to_string(),
            interfaces_by_name: "ietf-interfaces:interfaces/interface[name='{}']".to_string(),

            // ieee802-dot1q-sched
            gate_parameters: "ieee802-dot1q-sched:gate-parameters".to_string(),
            gate_enabled: "gate-enabled".to_string(),
            admin_gate_states: "admin-gate-states".to_string(),
            admin_control_list: "admin-control-list".to_string(),
            admin_control_list_by_index: "admin-control-list[index={}]".to_string(),
            operation_name: "operation-name".to_string(),
            sgs_params_gate_states_value: "sgs-params/gate-states-value".to_string(),
            sgs_params_time_interval_value: "sgs-params/time-interval-value".to_string(),
            admin_control_list_length: "admin-control-list-length".to_string(),
            admin_cycle_time_numerator: "admin-cycle-time/numerator".to_string(),
            admin_cycle_time_denominator: "admin-cycle-time/denominator".to_string(),
            admin_base_time_seconds: "admin-base-time/seconds".to_string(),
            admin_base_time_fractional_seconds: "admin-base-time/fractional-seconds".to_string(),
            admin_cycle_time_extension: "admin-cycle-time-extension".to_string(),
            config_change: "config-change".to_string(),
            tick_granularity: "tick-granularity".to_string(),

            // ieee802-dot1ab-lldp
            remote_systems_data: "ieee802-dot1ab-lldp:lldp/port/remote-systems-data".to_string(),
            chassis_id_subtype: "chassis-id-subtype".to_string(),
            chassis_id: "chassis-id".to_string(),
            port_id_subtype: "port-id-subtype".to_string(),
            port_id: "port-id".to_string(),
            port_desc: "port-desc".to_string(),
            system_name: "system-name".to_string(),
            system_description: "system-description".to_string(),
            system_capabilities_supported: "system-capabilities-supported".to_string(),
            system_capabilities_enabled: "system-capabilities-enabled".to_string(),
            time_mark: "time-mark".to_string(),
            remote_index: "remote-index".to_string(),
            management_address: "management-address".to_string(),
            attrib_name_address_subtype: "address-subtype".to_string(),
            attrib_name_address: "address".to_string(),

            // ieee802-dot1q-bridge
            bridge_port: "ieee802-dot1q-bridge:bridge/bridge-ports".to_string(),
            bridge_port_address: "bridge-port/address".to_string(),

            // ieee802-dot1q-bridge-port-delays
            bridge_port_delays: "bridge-port/bridge-port-delays".to_string(),
            port_speed: "port-speed".to_string(),
            dependent_rx_delay_min: "dependentRxDelayMin".to_string(),
            dependent_rx_delay_max: "dependentRxDelayMax".to_string(),
            independent_rx_delay_min: "independentRxDelayMin".to_string(),
            independent_rx_delay_max: "independentRxDelayMax".to_string(),
            independent_rly_delay_min: "independentRlyDelayMin".to_string(),
            independent_rly_delay_max: "independentRlyDelayMax".to_string(),
            independent_tx_delay_min: "independentTxDelayMin".to_string(),
            independent_tx_delay_max: "independentTxDelayMax".to_string(),
        }
    }
}

/// This struct is used to hold all filters that are used to search for specific information in the
/// yang-models.
pub struct SearchFilters {
    pub gate_parameters: String,
    pub gate_parameters_and_bridge_ports: String,
    pub remote_systems_data: String,
}

// TODO: if other filters are needed, you can add them here.
impl SearchFilters {
    fn load_br_filters() -> Self {
        Self {
            gate_parameters: 
                "<interfaces xmlns=\"urn:ietf:params:xml:ns:yang:ietf-interfaces\">
                    <interface>
                        <gate-parameters xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-sched\">
                        </gate-parameters>
                    </interface>
                </interfaces>"
            .to_string(),
            
            gate_parameters_and_bridge_ports: 
                "<interfaces xmlns=\"urn:ietf:params:xml:ns:yang:ietf-interfaces\">
                    <interface>
                        <gate-parameters xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-sched\"></gate-parameters>
                        <bridge-port xmlns=\"urn:ieee:std:802.1Q:yang:ieee802-dot1q-bridge\"></bridge-port>
                    </interface>
                </interfaces>"
            .to_string(),

            remote_systems_data:
                "<lldp xmlns=\"urn:ieee:std:802.1AB:yang:ieee802-dot1ab-lldp\">
                    <port>
                        <name></name>
                        <remote-systems-data></remote-systems-data>
                    </port>
                </lldp>"
            .to_string(),
        }
    }
}


/// all yang-models to load for the B&R Switch have to be included here.
/// TODO: if this should be dynamic, you can add a function to load the modules from a file or
/// something else
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
