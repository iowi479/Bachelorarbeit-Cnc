// ieee802-dot1q-tsn-types.yang -- as rust types
pub mod tsn_types {
    use serde::{Deserialize, Serialize};

    /// StreamId
    /// # Pattern as of the YANG-Models
    ///     "[0-9A-F]{2}"+
    ///     "(-[0-9A-F]{2}){5}"+
    ///     ":"+
    ///     "[0-9A-F]{2}"+
    ///     "-"+
    ///     "[0-9A-F]{2}"
    /// # Example
    /// stream_id: 00-00-00-00-00-00:7A-6E
    ///
    /// stream_id: 00-00-00-00-00-00:11-22
    pub type StreamIdTypeUpper = String;

    // TODO go through all types again, if something is missing
    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupInterfaceId {
        pub mac_address: String,
        pub interface_name: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupIeee802MacAddress {
        pub destination_mac_adress: String,
        pub source_mac_adress: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupIeee802VlanTag {
        pub priority_code_point: u8,
        pub vlan_id: u16,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupIpv4Tuple {
        pub source_ip_adress: std::net::Ipv4Addr,
        pub destination_ip_adress: std::net::Ipv4Addr,
        pub dscp: u8,
        pub protocol: u16,
        pub source_port: u16,
        pub destination_port: u16,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupIpv6Tuple {
        pub source_ip_adress: std::net::Ipv6Addr,
        pub destination_ip_adress: std::net::Ipv6Addr,
        pub dscp: u8,
        pub protocol: u16,
        pub source_port: u16,
        pub destination_port: u16,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupUserToNetworkRequirements {
        pub num_seemless_trees: u8,
        pub max_latency: u32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupInterfaceCapabilities {
        pub vlan_tag_capable: bool,
        pub cb_stream_iden_type_list: Vec<u32>,
        pub cb_sequence_type_list: Vec<u32>,
    }

    pub enum ConfigValue {
        Ieee802MacAddresses(GroupIeee802MacAddress),
        Ieee802VlanTag(GroupIeee802VlanTag),
        Ipv4Tuple(GroupIpv4Tuple),
        Ipv6Tuple(GroupIpv6Tuple),
        TimeAwareOffset(u32),
    }
    pub struct ConfigListElement {
        pub index: u8,
        pub config_value: ConfigValue,
    }
    pub struct InterfaceListElement {
        pub config_list: Vec<ConfigListElement>,
        // uses GroupInterfaceId
        // oder: (aber nicht genau nach standard)
        // group_interface_id: GroupInterfaceId
        pub mac_address: String,
        pub interface_name: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupInterfaceConfiguration {
        // interface_list: Vec,
        // TODO is needed?
    }

    // Need for fully centralized model
    #[derive(Serialize, Deserialize, Clone)]
    pub struct StreamRankContainer {
        pub rank: u8,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum DataFrameSpecificationElementType {
        Ieee802MacAddresses(GroupIeee802MacAddress),
        Ieee802VlanTag(GroupIeee802VlanTag),
        Ipv4Tuple(GroupIpv4Tuple),
        Ipv6Tuple(GroupIpv6Tuple),
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct DataFrameSpecificationElement {
        pub index: u8,
        pub field: DataFrameSpecificationElementType,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct TrafficSpecificationInterval {
        pub numerator: u32,
        pub denominator: u32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct TimeAwareContainer {
        pub earliest_transmit_offset: u32,
        pub latest_transmit_offset: u32,
        pub jitter: u32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct TrafficSpecificationContainer {
        pub interval: TrafficSpecificationInterval,
        pub max_frames_per_interval: u16,
        pub max_frame_size: u16,
        pub transmission_selection: u8,
        pub time_aware: TimeAwareContainer,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupTalker {
        // TODO stream_id
        pub stream_rank: StreamRankContainer,
        pub end_station_interfaces: Vec<EndStationInterface>,
        pub data_frame_specification: Vec<DataFrameSpecificationElement>,
        pub traffic_specification: TrafficSpecificationContainer,
        pub user_to_network_requirements: GroupUserToNetworkRequirements,
        pub interface_capabilities: GroupInterfaceCapabilities,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct EndStationInterface {
        pub index: u32,
        pub interface_id: GroupInterfaceId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupListener {
        pub index: u32,
        pub end_station_interfaces: Vec<EndStationInterface>,
        pub user_to_network_requirements: GroupUserToNetworkRequirements,
        pub interface_capabilities: GroupInterfaceCapabilities,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum TalkerStatus {
        None = 0,
        Ready = 1,
        Failed = 2,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum ListenerStatus {
        None = 0,
        Ready = 1,
        PartialFailed = 2,
        Failed = 3,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct StatusInfoContainer {
        pub talker_status: TalkerStatus,
        pub listener_status: ListenerStatus,
        pub failure_code: i32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupStatusStream {
        pub status_info: StatusInfoContainer,
        pub failed_interfaces: Vec<GroupInterfaceId>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct GroupStatusTalkerListener {
        pub accumulated_latency: u32,
        pub interface_configuration: GroupInterfaceConfiguration,
    }
}

// ieee802-dot1q-tsn-config-uni.yang -- as rust types
pub mod uni_types {
    use super::tsn_types;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Domain {
        pub domain_id: String,
        pub cnc_enabled: bool,
        pub cuc: Vec<Cuc>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Cuc {
        pub cuc_id: String,
        pub stream: Vec<Stream>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum StreamStatus {
        Planned = 0,
        Configured = 1,
        Modified = 2,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Stream {
        pub stream_id: String,
        pub stream_status: StreamStatus,
        pub talker: Talker,
        pub listener: Vec<Listener>,
        pub group_status_stream: tsn_types::GroupStatusStream,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Talker {
        // evtl nicht als struct und fields...
        pub group_talker: tsn_types::GroupTalker,
        pub group_status_talker_listener: tsn_types::GroupStatusTalkerListener,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Listener {
        // evtl nicht als struct und fields...
        pub index: u32,
        pub group_listener: tsn_types::GroupListener,
        pub group_status_talker_listener: tsn_types::GroupStatusTalkerListener,
    }

    pub mod compute_streams {
        // rpc compute_streams
        pub type Input = Vec<Domain>;

        pub struct Domain {
            pub domain_id: String,
            pub cuc: Vec<CucElement>,
        }

        pub struct CucElement {
            pub cuc_id: String,
            pub stream_list: Vec<crate::cnc::tsntypes::tsn_types::StreamIdTypeUpper>,
        }

        pub type Output = String;
    }

    pub mod compute_planned_and_modified_streams {
        // rpc compute_planned_and_modified_streams
        pub type Input = Vec<Domain>;
        pub struct Domain {
            pub domain_id: String,
            pub cuc: Vec<String>,
        }

        pub type Output = String;
    }

    pub mod compute_all_streams {
        // rpc compute_all_streams
        pub type Input = Vec<Domain>;

        pub struct Domain {
            pub domain_id: String,
            pub cuc: Vec<String>,
        }

        pub type Output = String;
    }

    pub mod request_domain_id {
        // rpc request_domain_id
        pub type Input = String;
        pub type Output = String;
    }

    pub mod request_free_stream_id {
        // rpc request_free_stream_id
        pub struct Input {
            pub domain_id: String,
            pub cuc_id: String,
        }

        pub type Output = String;
    }

    pub mod remove_streams {
        use crate::cnc::tsntypes::tsn_types::StreamIdTypeUpper;
        pub type Input = Vec<StreamIdTypeUpper>;
        pub type Output = String;
    }
}

// ieee802-dot1q-sched.yang -- as rust types
pub mod shed_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct GateControlEntry {
        operation_name: Box<GateControlEntry>,
        time_interval_value: u32,
        gate_state_value: u8,
    }

    #[derive(Debug)]
    pub enum GateControlOperation {
        SetGateStates,
        SetAndHoldMAC,
        SetAndReleaseMAC,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct QueueMaxSduEntry {
        traffic_class: u8, // TODO u8 type? supported traffic classes up to 8
        queue_max_sdu: u32,
        transmission_overrun: u64,
    }

    pub type PtpTimeScale = u32;
    pub type RationalGrouping = (i32, i32);

    pub type SchedParameters = Vec<GateParameterTableEntry>;

    #[derive(Debug)]
    pub struct GateParameterTableEntry {
        queue_max_sdu_table: Vec<QueueMaxSduEntry>,
        gate_enable: bool,
        admin_gate_states: u8, // all 8 gates coded into bit representation
        oper_gate_states: u8,  // all 8 gates coded into bit representation
        admin_control_list: Vec<GateControlEntry>,
        oper_control_list: Vec<GateControlEntry>,
        admin_cycle_time: RationalGrouping,
        oper_cycle_time: RationalGrouping,
        admin_cycle_time_extension: u32,
        oper_cycle_time_extension: u32,
        admin_base_time: PtpTimeScale,
        oper_base_time: PtpTimeScale,
        config_change: bool,
        config_time_change_time: PtpTimeScale,
        tick_granularity: u32,
        current_time: PtpTimeScale,
        config_pending: bool,
        config_change_error: u64,
        supported_list_max: u32,
        supported_cycle_max: RationalGrouping,
        supported_interval_max: u32,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ConfigurableGateParameterTableEntry {
        // YANG -> The value must be retained across reinitializations of the management system.
        queue_max_sdu_table: Vec<QueueMaxSduEntry>,

        gate_enable: bool,
        admin_gate_states: u8, // all 8 gates coded into bit representation
        admin_control_list: Vec<GateControlEntry>,
        admin_cycle_time: RationalGrouping,
        admin_cycle_time_extension: u32,
        admin_base_time: PtpTimeScale,

        // must not be retained... This applies the config?
        config_change: bool,

        // config false aber The value must be retained across reinitializations of the management system.
        tick_granularity: u32,

        // following -> Maybe
        supported_list_max: u32,
        supported_cycle_max: RationalGrouping,
        supported_interval_max: u32,
    }
}

pub mod notification_types {
    use super::tsn_types::StreamIdTypeUpper;

    pub type NotificationContent = Vec<Domain>;

    #[derive(Debug)]
    pub struct Domain {
        domain_id: String,
        cucs: Vec<Cuc>,
    }

    #[derive(Debug)]
    pub struct Cuc {
        cuc_id: String,
        streams: Vec<Stream>,
    }

    #[derive(Debug)]
    pub struct Stream {
        stream_id: StreamIdTypeUpper,
        /// # Values
        /// successful (0) or unsuccessful (1)
        failure_code: u8,
    }
}
