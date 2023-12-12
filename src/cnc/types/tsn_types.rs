use serde::{Deserialize, Serialize};

/// This typedef specifies a Stream ID, a unique identifier of the
/// Stream's configuration, used by protocols in the network to
/// associate the user's Stream with TSN resources.
///
/// The Stream ID is a string that represents two fields:
///
/// # MAC Address:
///
/// A 48-bit IEEE 802 MAC address associated with the Talker sourcing
/// the Stream to the bridged network. The entire range of MAC
/// addresses are acceptable.
///
/// NOTE 1The MAC address component of the StreamID can, but does not
/// necessarily, have the same value as the source_address parameter
/// of any frame in the actual data Stream. For example, the Stream ID
/// can be assigned by a TSN CUC (see 46.1.3.3 of IEEE Std
/// 802.1Q-2022), using a pool of MAC addresses that the TSN CUC
/// maintains.
///
/// NOTE 2If the MAC addresses used to construct Stream IDs are not
/// unique within the network, duplicate Stream IDs can be generated,
/// with unpredictable results.
///
/// # Unique ID:
///
/// A 16-bit unique ID that is used to distinguish between multiple
/// Streams within the station identified by MAC Address.
///
/// The string specifies eight octets, with each octet represented as
/// two hexadecimal characters. The first six octets specify the MAC
/// Address, using the canonical format of IEEE Std 802, with a dash
/// separating each octet. The last two octets specify the Unique ID,
/// with the high-order octet, a dash, and then the low-order octet.
/// The MAC Address and Unique ID are separated by colon. Only upper
/// case characters are allowed to be used for the hexadecimal
/// characters.
///
/// stream-id-type is intended for use by other modules as the type
/// for a key to a list of Stream configurations (using group-talker
/// and group-listener) and a list of Stream status (using
/// group-status-stream and group-status-talker-listener).
///
/// # Pattern
///     "[0-9A-F]{2}"+
///     "(-[0-9A-F]{2}){5}"+
///     ":"+
///     "[0-9A-F]{2}"+
///     "-"+
///     "[0-9A-F]{2}"
///
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

#[derive(Serialize, Deserialize, Clone)]
pub enum ConfigValue {
    Ieee802MacAddresses(GroupIeee802MacAddress),
    Ieee802VlanTag(GroupIeee802VlanTag),
    Ipv4Tuple(GroupIpv4Tuple),
    Ipv6Tuple(GroupIpv6Tuple),
    TimeAwareOffset(u32),
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigListElement {
    pub index: u8,
    pub config_value: ConfigValue,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct InterfaceListElement {
    pub config_list: Vec<ConfigListElement>,
    pub mac_address: String,
    pub interface_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GroupInterfaceConfiguration {
    pub interface_list: Vec<InterfaceListElement>,
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

/// This packet is only viable for the specific b&r switch used in this paper. Since this Object is not (yet) in the official IEEE Standard.
#[derive(Serialize, Deserialize, Clone)]
pub struct BridgePortDelays {
    pub port_speed: u32,
    pub dependent_rx_delay_min: u32,
    pub dependent_rx_delay_max: u32,
    pub independent_rx_delay_min: u32,
    pub independent_rx_delay_max: u32,
    pub independent_rly_delay_min: u32,
    pub independent_rly_delay_max: u32,
    pub independent_tx_delay_min: u32,
    pub independent_tx_delay_max: u32,
}
