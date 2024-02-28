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
/// NOTE 1 The MAC address component of the StreamID can, but does not
/// necessarily, have the same value as the source_address parameter
/// of any frame in the actual data Stream. For example, the Stream ID
/// can be assigned by a TSN CUC (see 46.1.3.3 of IEEE Std
/// 802.1Q-2022), using a pool of MAC addresses that the TSN CUC
/// maintains.
///
/// NOTE 2 If the MAC addresses used to construct Stream IDs are not
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

/// This YANG grouping specifies the identification of a distinct
/// point of attachment (interface) in a station (end station or
/// Bridge).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupInterfaceId {
    /// mac-address is the unique individual MAC address (IEEE Std 802) of
    /// the interface in the station (end station or Bridge). This MAC
    /// address uniquely identifies the station within the local
    /// network.
    ///
    /// mac-address shall be included in an instance of a container
    /// using group-interface-id.
    ///
    /// NOTE This MAC address can be discovered in the physical topology
    /// using protocols such as IEEE Std 802.1AB (LLDP). LLDP supports
    /// MAC address as a subtype for the stations Chassis ID and Port
    /// ID. If the station does not use MAC address for its LLDP IDs,
    /// remote management can be used to associate this mac-address to
    /// the values provided in the LLDP IDs.
    ///
    /// The string uses the hexadecimal representation specified in IEEE
    /// Std 802 (i.e. canonical format).
    pub mac_address: String,

    /// interface-name is the name of the interface that is assigned
    /// locally by the station (end station or Bridge).
    ///
    /// interface-name may be included in an instance of a container
    /// using group-interface-id.
    ///
    /// IEEE Std 802 recommends that each distinct point of attachment
    /// to an IEEE 802 network have its own EUI MAC address. If the
    /// identified station follows this IEEE 802 recommendation, the
    /// mac-address leaf uniquely identifies the interface as well as
    /// the station, and interface-name is not needed.
    ///
    /// If the mac-address applies to more than one interface (distinct
    /// point of attachment) within the station, interface-name provides
    /// a locally assigned name that can help to identify the interface.
    ///
    /// When YANG is used for management of the station, interface-name
    /// is the interface name that serves as the key for the stations
    /// interface list (RFC7223).
    ///
    /// NOTE 1 The TSN CNC is typically located in a different physical
    /// product than the station identified by this group-interface-id.
    /// Since the interface-name is assigned locally by the identified
    /// station, it is possible that the stations product will change
    /// interface-name in a manner that the TSN CNC cannot detect. For
    ///  example, RFC7223 mentions that the YANG interface name can
    /// change when a physical attachment point is inserted or removed.

    /// NOTE 2 This interface name can be discovered in the physical
    /// topology using protocols such as IEEE Std 802.1AB (LLDP). LLDP
    /// supports interface name as a subtype for its Port ID. If the
    /// station does not use interface name for its LLDP Port ID, remote
    /// management can be used to associate this interface-name to the
    /// values provided in the LLDP Port ID.
    pub interface_name: String,
}

/// This YANG grouping specifies the pair of IEEE 802 MAC addresses
/// for Stream identification.
///
/// The use of these fields for Stream identification corresponds to
/// the managed objects for Stream identification in IEEE Std 802.1CB.
/// If inconsistency arises between this specification and IEEE Std
/// 802.1CB, IEEE Std 802.1CB takes precedence.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupIeee802MacAddress {
    /// Destination MAC address.
    ///
    /// An address of all 1's specifies that the destination MAC address
    /// is ignored for purposes of Stream identification.
    ///
    /// The string uses the hexadecimal representation specified in IEEE
    /// Std 802 (i.e. canonical format).
    pub destination_mac_adress: String,

    /// Source MAC address.
    ///
    /// An address of all 1's specifies that the source MAC address is
    /// ignored for purposes of Stream identification.
    ///
    /// The string uses the hexadecimal representation specified in IEEE
    /// Std 802 (i.e. canonical format).
    pub source_mac_adress: String,
}

/// This YANG grouping specifies a customer VLAN Tag (C-TAG of clause
///    9) for Stream identification.
///
/// The Drop Eligible Indicator (DEI) field is not relevant from the
/// perspective of a TSN Talker/Listener.
///
/// The use of these fields for Stream identification corresponds to
/// the managed objects for Stream identification in IEEE Std 802.1CB.
/// If inconsistency arises between this specification and IEEE Std
/// 802.1CB, IEEE Std 802.1CB takes precedence.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupIeee802VlanTag {
    /// Priority Code Point (PCP) field.
    ///
    /// The priority-code-point is not used to identify the Stream, but
    /// it does identify a traffic class (queue) in Bridges.
    pub priority_code_point: u8,

    /// VLAN ID (VID) field.
    ///   
    /// If only the priority-code-point is known, the vlan-id is
    /// specified as 0.
    pub vlan_id: u16,
}

/// This YANG grouping specifies parameters to identify an IPv4
/// (RFC791) Stream.
///
/// The use of these fields for Stream identification corresponds to
/// the managed objects for Stream identification in IEEE Std 802.1CB.
/// If inconsistency arises between this specification and IEEE Std
/// 802.1CB, IEEE Std 802.1CB takes precedence.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupIpv4Tuple {
    /// Source IPv4 address.
    ///
    /// An address of all 0's specifies that the IP source address is
    /// ignored for purposes of Stream identification.
    pub source_ip_adress: std::net::Ipv4Addr,

    /// Destination IPv4 address.
    pub destination_ip_adress: std::net::Ipv4Addr,

    /// Differentiated services code point, DSCP (RFC2474).
    ///
    /// A value of 64 decimal specifies that the DSCP is ignored for
    /// purposes of Stream identification.
    pub dscp: u8,

    /// IPv4 Protocol (e.g. UDP).
    ///
    /// The special value of all 1s (FFFF hex) represents None, meaning
    /// that protocol, source-port, and destination-port are ignored for
    /// purposes of Stream identification.
    ///
    /// For any value other than all 1s, the lower octet is used to
    /// match IPv4 Protocol.
    pub protocol: u16,

    /// This matches the source port of the protocol.
    pub source_port: u16,

    /// This matches the destination port of the protocol.
    pub destination_port: u16,
}

/// This YANG grouping specifies parameters to identify an IPv6
/// (RFC8200) Stream.
///
/// The use of these fields for Stream identification corresponds to
/// the managed objects for Stream identification in IEEE Std 802.1CB.
/// If inconsistency arises between this specification and IEEE Std
/// 802.1CB, IEEE Std 802.1CB takes precedence.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupIpv6Tuple {
    /// Source IPv6 address.
    ///
    /// An address of all 0's specifies that the IP source address is
    /// ignored for purposes of Stream identification.
    pub source_ip_adress: std::net::Ipv6Addr,

    /// Destination IPv6 address.
    pub destination_ip_adress: std::net::Ipv6Addr,

    /// Differentiated services code point, DSCP (RFC2474).
    ///
    /// A value of 64 decimal specifies that the DSCP is ignored for
    /// purposes of Stream identification.
    pub dscp: u8,

    /// IPv6 Next Header (e.g. UDP).
    ///
    /// The special value of all 1s (FFFF hex) represents None, meaning
    /// that protocol, source-port, and destination-port are ignored for
    /// purposes of Stream identification.
    ///
    /// For any value other than all 1s, the lower octet is used to
    /// match IPv6 Next Header.
    pub protocol: u16,

    /// This matches the source port of the protocol.
    pub source_port: u16,

    /// This matches the destination port of the protocol.
    pub destination_port: u16,
}

/// This YANG grouping specifies specifies user requirements for the
/// Stream, such as latency and redundancy.
///
/// The network (e.g. CNC) will merge all user-to-network-requirements
/// for a Stream to ensure that all requirements are met.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupUserToNetworkRequirements {
    /// num-seamless-trees specifies the number of trees that the
    /// network will configure to deliver seamless redundancy for the
    /// Stream.
    ///
    /// The value zero is interpreted as one (i.e. no seamless
    /// redundancy).
    ///
    /// This requirement is provided from the Talker only. Listeners
    /// shall set this leaf to one.
    ///
    /// From each Talker to a single Listener, the network configures a
    /// path that relays Stream data through Bridges. If the Talker has
    /// more than one Listener, the network configures a tree of
    /// multiple paths.
    ///
    /// num-seamless-trees specifies the number of maximally disjoint
    /// trees that the network shall configure from the Talker to all
    /// Listeners. Each tree is disjoint from other trees, in that the
    /// network evaluates the physical topology to avoid sharing the
    /// same Bridge and links in each trees paths. This computation of
    /// disjoint trees is maximal, in that shared Bridges and links are
    /// avoided to the maximum extent allowed by the physical topology.
    /// For example, if a single link exists from a Bridge to a
    /// Listener, and num-seamless-trees is 3, then all 3 trees will
    /// share that link to the Listener.
    ///
    /// When num-seamless-trees is greater than one, the transfer of the
    /// Streams data frames shall use a seamless redundancy standard,
    /// such as IEEE Std 802.1CB. When a link shared by multiple trees
    /// diverges to multiple disjoint links, the seamless redundancy
    /// standard replicates (i.e. forwards a distinct copy of each data
    /// frame to the disjoint trees). When disjoint trees converge to a
    /// single link, the seamless redundancy standard eliminates the
    /// duplicate copies of each data frame. Assuming that other sources
    /// of frame loss are mitigated (e.g. congestion), failure of a link
    /// or Bridge in one disjoint tree does not result in frame loss as
    /// long as at least one remaining disjoint tree is operational.
    ///
    /// If the Talker sets this leaf to one, the network may make use of
    /// redundancy standards that are not seamless (i.e. failure of link
    /// results in lost frames), such as MSTP and IS-IS.
    ///
    /// If the Talker sets this leaf to greater than one, and seamless
    /// redundancy is not possible in the current network (no disjoint
    /// paths, or no seamless redundancy standard in Bridges),
    /// group-status-stream.status-info.failure-code is non-zero
    /// (46.2.4.1 of IEEE Std 802.1Q-2022).
    ///
    /// If group-user-to-network-requirements is not provided by the
    /// Talker or Listener, the network shall use the default value of
    /// one for this leaf.
    pub num_seemless_trees: u8,

    /// Maximum latency from Talker to Listener(s) for a single frame
    /// of the Stream.
    ///
    /// max-latency is specified as an integer number of nanoseconds.
    ///
    /// Latency shall use the definition of 3.102, with additional
    /// context as follows: The Known reference point in the frame is
    /// the message timestamp point specified in IEEE Std 802.1AS for
    /// various media (i.e. start of the frame). The first point is in
    /// the Talker, at the reference plane marking the boundary between
    /// the network media and PHY (see IEEE Std 802.1AS). The second
    /// point is in the Listener, at the reference plane marking the
    /// boundary between the network media and PHY.
    ///
    /// When this requirement is specified by the Talker, it must be
    /// satisfied for all Listeners.
    ///
    /// When this requirement is specified by the Listener, it must be
    /// satisfied for this Listener only.
    ///
    /// If group-user-to-network-requirements is not provided by the
    /// Talker or Listener, the network shall use the default value of
    /// zero for this leaf.
    ///
    /// The special value of zero represents usage of the initial value
    /// of group-status-talker-listener.accumulated-latency as the
    /// maximum latency requirement. This effectively locks-down the
    /// initial latency that the network calculates after successful
    /// configuration of the Stream, such that any subsequent increase
    /// in latency beyond that value causes the Stream to fail.
    ///
    /// The assumption for when the first point occurs in the Talker
    /// depends on the presence of the time-aware container in the
    /// Talkers traffic-specification.
    ///
    /// When time-aware is not present:
    ///
    /// The Talker is assumed to transmit at an arbitrary time (not
    /// scheduled).
    ///
    /// When time-aware is present:
    ///
    /// The first point is assumed to occur at the start of each
    /// traffic-specification.interval, as if the Talkers offsets
    /// (earliest-transmit-offset and latest-transmit-offset) are both
    /// zero. The Talkers offsets are not typically zero, but use of the
    /// start of interval for purposes of max-latency allows the
    /// Listener(s) to schedule their application independently from the
    /// Talkers offset configuration.
    ///
    /// The Listener determines max-latency based on its scheduling of a
    /// read function in the application. Nevertheless, the time from
    /// frame reception (i.e. second point) to execution of the read
    /// function is in the user scope, and therefore not included in
    /// max-latency.
    ///
    /// max-latency can be set to a value greater than the Talkers
    /// interval, in order to specify a longer latency requirement. For
    /// example, if the Talkers interval is 500 microsec, and
    /// max-latency is 700 microsec, the Listener receives the frame no
    /// later than 200 microsec into the interval that follows the
    /// Talkers interval.
    pub max_latency: u32,
}

/// This YANG grouping specifies the network capabilities of all
/// interfaces (Ports) contained in end-station-interfaces.
///
/// The network may provide configuration of these capabilities in
/// group-status-talker-listener.interface-configuration.
///
/// NOTE If an end station contains multiple interfaces with different
/// network capabilities, each interface should be specified as a
/// distinct Talker or Listener (i.e. one entry in
/// end-station-interfaces). Use of multiple entries in
/// end-station-interfaces is intended for network capabilities that
/// span multiple interfaces (e.g. seamless redundancy).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupInterfaceCapabilities {
    /// When vlan-tag-capable is true, the interface supports the
    /// ability to tag/untag frames using a Customer VLAN Tag (C-TAG of
    /// clause 9) provided by the network.
    ///
    /// For a Talker, the networks tag replaces the tag specified by the
    /// data-frame-specification. If the data-frame-specification is
    /// untagged (no group-ieee802-vlan-tag), the networks tag is
    /// inserted in the frame as it passes through the interface.
    ///
    /// For a Listener, the users tag from the data-frame-specification
    /// replaces the networks tag as the frame passes through the
    /// interface. If the data-frame-specification is untagged (no
    /// group-ieee802-vlan-tag), the networks tag is removed from the
    /// frame as it passes through the interface.
    ///
    /// If the end station supports more than one interface (i.e. more
    /// than one entry in end-station-interfaces), vlan-tag-capable of
    /// true means that a distinct VLAN tag can be applied to each
    /// interface. The list of VLAN tag (one for each interface) can be
    /// provided by the network in
    /// interface-configuration.interface-list (ieee802-vlan-tag
    /// choice).
    ///
    /// When vlan-tag-capable is false, the interface does not support
    /// the capability to tag/untag frames using a Customer VLAN Tag
    /// (C-TAG of clause 9) provided by the network.
    ///
    /// If interface-capabilities is not provided by the Talker or
    /// Listener, the network shall use the default value of false for
    /// this leaf.
    pub vlan_tag_capable: bool,

    /// cb-stream-iden-type-list provides a list of the supported
    /// Stream Identification types as specified in IEEE Std 802.1CB.
    ///  
    /// Each Stream Identification type is provided as a 32-bit unsigned
    /// integer. The upper three octets contain the OUI/CID, and the
    /// lowest octet contains the type number.
    ///  
    /// NOTE If the Talker/Listener end system supports IEEE Std 802.1CB,
    /// Null Stream identification is required, and that Stream
    /// Identification type is included in this list. If the
    /// Talker/Listener end system does not support IEEE Std 802.1CB,
    /// this list is empty.
    ///  
    /// If the end station supports more than one interface (i.e. more
    /// than one interface-id in end-station-interfaces, an empty
    /// cb-stream-iden-type-list means that the end station is capable
    /// of transferring the Stream on any one of its interfaces (not
    /// all). When this is specified, the network shall decide which
    /// interface is best used for TSN purposes, and communicate that
    /// decision by returning a single interface in
    /// interface-configuration.interface-list. The Talker/Listener uses
    /// this interface alone for the Stream.
    ///  
    /// If interface-capabilities is not provided within group-talker or
    /// group-listener, the network shall use an empty list as the
    /// default value for this element.
    pub cb_stream_iden_type_list: Vec<u32>,

    /// cb-sequence-type-list provides a list of the supported Sequence
    /// Encode/Decode types as specified in IEEE Std 802.1CB.
    ///
    /// Each sequence type is provided as a 32-bit unsigned integer. The
    /// upper three octets contain the OUI/CID, and the lowest octet
    /// contains the type number.
    ///
    /// If interface-capabilities is not provided within group-talker or
    /// group-listener, the network shall use an empty list as the
    /// default value for this element.
    pub cb_sequence_type_list: Vec<u32>,
}

/// One of the following choices is provided for each
/// configuration value. Each container name acts as the case
/// name for the choice.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConfigValue {
    /// Source and destination MAC addresses that apply to the
    /// network side of the user/network boundary.
    ///
    /// NOTE 1 On the userside, the MAC addresses correspond to the
    /// ieee802-mac-addresses of data-frame-specification.
    ///
    /// NOTE 2 The source MAC address of the network is typically
    /// the same as the user. The destination MAC address can be
    /// different. For example, the user can use an individual
    /// address, but the network can use a group (multicast)
    /// address.
    ///
    /// This configuration value is not provided unless IEEE Std
    /// 802.1CB is supported, and a value for Active Destination
    /// MAC and VLAN Stream identification is provided in
    /// cb-stream-iden-type-list of interface-capabilities.
    Ieee802MacAddresses(GroupIeee802MacAddress),

    /// Customer VLAN Tag (C-TAG of clause 9) that applies to the
    /// network side of the user/network boundary.
    ///
    /// NOTE On the user side, the VLAN tag corresponds to the
    /// ieee802-vlan-tag of data-frame-specification (including
    /// untagged if this field is not provided).
    ///
    /// If the user provides a VLAN ID in the ieee802-vlan-tag of
    /// data-frame-specification, the Streams data frames are
    /// assumed to be limited to the active topology for that VLAN
    /// ID. Therefore, if the network uses a different VLAN ID in
    /// this config-value, the network shall ensure that the
    /// replacement VLAN ID is limited to the equivalent active
    /// topology.
    ///
    /// This configuration value is not provided unless
    /// vlan-tag-capable of interface-capabilities is true.
    Ieee802VlanTag(GroupIeee802VlanTag),

    /// IPv4 identification that applies to the network side of
    /// the user/network boundary.
    ///
    /// This configuration value is not provided unless IEEE Std
    /// 802.1CB is supported, and a value for IP Stream
    /// identification is provided in cb-stream-iden-type-list of
    /// interface-capabilities.
    Ipv4Tuple(GroupIpv4Tuple),

    /// IPv6 identification that applies to the network side of
    /// the user/network boundary.
    ///
    /// This configuration value is not provided unless IEEE Std
    /// 802.1CB is supported, and a value for IP Stream
    /// identification is provided in cb-stream-iden-type-list of
    /// interface-capabilities.
    Ipv6Tuple(GroupIpv6Tuple),

    /// If the time-aware container is present in the
    /// traffic-specification of the Talker, this config-value
    /// shall be provided by the network to the Talker.
    ///
    /// If the time-aware container is not present in the
    /// traffic-specification of the Talker, this config-value
    /// shall not be provided by the network.
    ///
    /// This config-value shall not be provided to Listeners, as
    /// it is not applicable.
    ///
    /// time-aware-offset specifies the offset that the Talker
    /// shall use for transmit. The network returns a value
    /// between earliest-transmit-offset and
    /// latest-transmit-offset of the Talkers
    /// traffic-specification. The value is expressed as
    /// nanoseconds after the start of the Talkers interval.
    TimeAwareOffset(u32),
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigListElement {
    /// This index is provided in order to provide a unique key per
    /// list entry. The value of index for each entry shall be
    /// unique (but not necessarily contiguous).
    pub index: u8,

    /// One of the following choices is provided for each
    /// configuration value. Each container name acts as the case
    /// name for the choice.
    pub config_value: ConfigValue,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InterfaceListElement {
    /// List of configuration values for the interface.
    pub config_list: Vec<ConfigListElement>,
    pub group_interface_id: GroupInterfaceId,
}

/// This YANG grouping provides configuration of interfaces in the
/// Talker/Listener. This configuration assists the network in meeting
/// the Streams requirements. The interface-configuration meets the
/// capabilities of the interface as provided in
/// interface-capabilities.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupInterfaceConfiguration {
    /// A distinct configuration is provided for each interface in the
    /// Talker/Listener (even if multiple interfaces use the same
    /// configuration). Each entry in this interface-list consists of an
    /// interface identification (group-interface-id), followed by a
    /// list of configuration values for that interface (config-list).
    ///
    /// If interface-configuration is not provided within
    /// group-status-talker-listener, the network shall assume zero
    /// entries as the default (no interface configuration).
    ///
    /// Since the interface-name leaf is optional, empty string can be
    /// used for its key value.
    pub interface_list: Vec<InterfaceListElement>,
}

/// Rank of this Stream's configuration relative to other Streams
/// in the network. This rank is used to determine success/failure
/// of Stream resource configuration, and it is unrelated to the
/// Streams data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StreamRankContainer {
    /// The Rank is used by the network to decide which Streams can
    /// and cannot exist when TSN resources reach their limit. If a
    /// Bridges Port becomes oversubscribed (e.g. network
    /// reconfiguration, IEEE 802.11 bandwidth reduction), the Rank is
    /// used to help determine which Streams can be dropped (i.e.
    /// removed from Bridge configuration).
    ///
    /// The only valid values for Rank shall be zero and one. The
    /// configuration of a Stream with Rank zero is more important
    /// than the configuration of a Stream with Rank one. The Rank
    /// value of zero is intended for emergency traffic, and the Rank
    /// value of one is intended for non-emergency traffic.
    ///
    /// NOTE It is expected that higher layer applications and
    /// protocols can use the Rank to indicate the relative importance
    /// of Streams based on user preferences. Those user preferences
    /// are expressed by means beyond the scope of this standard. When
    /// multiple applications exist in a network (e.g. audio/video
    /// along with industrial control), it can be challenging for the
    /// varied applications and vendors to agree on multiple Rank
    /// values. To mitigate such challenges, this Rank uses a simple
    /// concept of emergency (zero) and non-emergency (one) that can
    /// be applied over all applications. For example, in a network
    /// that carries audio Streams for fire safety announcements, all
    /// applications are likely to agree that those Streams use Rank
    /// of zero.
    pub rank: u8,
}

/// One of the following choices is provided for each field that
/// the user knows. Each container name acts as the case name for
/// the choice.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataFrameSpecificationElementType {
    /// IEEE 802 MAC addresses.
    Ieee802MacAddresses(GroupIeee802MacAddress),

    /// IEEE 802.1 CTAG
    Ieee802VlanTag(GroupIeee802VlanTag),

    /// IPv4 packet identification
    Ipv4Tuple(GroupIpv4Tuple),

    /// IPv6 packet identification
    Ipv6Tuple(GroupIpv6Tuple),
}

/// data-frame-specification specifies the frame that carries the
/// Talkers Stream data. The network uses the specification to
/// identify this Streams frames as TSN, in order to apply the
/// required TSN configuration.
///
/// The specification is based on the users knowledge of the frame,
/// without any network specifics. In other words, this specifies
/// the frame that the Talker would use in the absence of TSN.
///
/// The specification is provided as a list of fields that the user
/// knows. The list is ordered from start of frame to end of header.
/// For example, if the Talker uses a VLAN-tagged Ethernet frame
/// (not IP), the list consists of ieee802-mac-addresses followed by
/// ieee802-vlan-tag. For example, if the Talker uses a UDP/IPv4
/// packet without knowledge of the Ethernet header, the list
/// consists of ipv4-tuple.
///         
/// This list is optional, and its absence indicates that Stream
/// transformation is performed in the Talker and Listeners of this
/// Stream (46.2.2 of IEEE Std 802.1Q-2022).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataFrameSpecificationElement {
    /// This index is provided in order to provide a unique key per
    /// list entry. The value of index for each entry shall be unique
    /// (but not necessarily contiguous).
    pub index: u8,
    pub field: DataFrameSpecificationElementType,
}

/// This interval specifies the period of time in which the
/// traffic specification cannot be exceeded. The traffic
/// specification is specified by max-frames-per-interval and
/// max-frame-size.
///
/// The interval is a rational number of seconds, defined by an
/// integer numerator and an integer denominator.
///
/// If the time-aware container is not present, the interval
/// specifies a sliding window of time. The Talkers transmission
/// is not synchronized to a time on the network, and therefore
/// the traffic specification cannot be exceeded during any
/// interval in time.
///
/// If the time-aware container is present, the interval specifies
/// a window of time that is aligned with the time epoch that is
/// synchronized on the network. For example, if IEEE Std
/// 802.1AS-2011 is used with the PTP timescale, the first
/// interval begins at 1 January 00:00:00 TAI. If CurrentTime
/// represents the current time, then the start of the next
/// interval (StartOfNextInterval) is: StartOfNextInterval = N *
/// interval where N is the smallest integer for which the
/// relation StartOfNextInterval >= CurrentTime would be TRUE.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficSpecificationInterval {
    /// intervals numerator.
    pub numerator: u32,

    /// intervals denominator.
    pub denominator: u32,
}

/// The time-aware container provides leafs to specify the
/// Talkers time-aware transmit to the network.
///
/// The Talker and Listeners of a Stream are assumed to coordinate
/// using user (application) mechanisms, such that each Listener
/// is aware that its Talker transmits in a time-aware manner.
///
/// If max-frames-per-interval is greater than one, the Talker
/// shall transmit multiple frames as a burst within the interval,
/// with the minimum inter-frame gap allowed by the media.
///
/// NOTE: Although scheduled traffic (8.6.8.4 of IEEE Std
/// 802.1Q-2022) specifies a valid implementation of a time-aware
/// Talker, the time-aware container is intended to support
/// alternate implementations of scheduling.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimeAwareContainer {
    /// earliest-transmit-offset specifies the earliest offset
    /// within each interval at which the Talker is capable of
    /// starting transmit of its frames. As part of
    /// group-status-talker-listener.interface-configuration, the
    /// network will return a specific time-aware-offset to the
    /// Talker (within the earliest/latest range), which the Talker
    /// uses to schedule its transmit.
    ///
    /// earliest-transmit-offset is specified as an integer number
    /// of nanoseconds.
    ///
    /// The Talkers transmit offsets include
    /// earliest-transmit-offset, latest-transmit-offset, and the
    /// time-aware-offset returned to the Talker. Each of the
    /// Talkers offsets is specified at the point when the message
    /// timestamp point of the first frame of the Stream passes the
    /// reference plane marking the boundary between the network
    /// media and PHY. The message timestamp point is specified by
    /// IEEE Std 802.1AS for various media.
    pub earliest_transmit_offset: u32,

    /// latest-transmit-offset specifies the latest offset within
    /// the interval at which the Talker is capable of starting
    /// transmit ofits frames. As part of
    /// group-status-talker-listener.interface-configuration, the
    /// network will return a specific time-aware-offset to the
    /// Talker within the earliest/latest range), which the Talker
    /// uses to schedule its transmit.
    ///
    /// latest-transmit-offset is specified as an integer number of
    /// nanoseconds.
    pub latest_transmit_offset: u32,

    /// The jitter leaf specifies the maximum difference in time
    /// between the Talkers transmit offsets, and the ideal
    /// synchronized network time (e.g. IEEE 802.1AS time). Jitter
    /// is specified as an unsigned integer number of nanoseconds.
    ///
    /// The maximum difference means sooner or later than the ideal
    /// (e.g. transmit +/- 500 nanoseconds relative to IEEE 802.1AS
    /// time results in jitter of 500).
    ///
    /// The ideal synchronized network time refers to time at the
    /// source (e.g. IEEE 802.1AS grandmaster). The jitter does not
    /// include inaccuracies as time is propagated from the time
    /// source to the Talker, because those inaccuracies are assumed
    /// to be known by the network, and time synchronization is a
    /// network technology. The jitter leaf is intended to specify
    /// inaccuracies in the Talkers implementation. For example, if
    /// the Talkers IEEE 802.1AS time is +/- 812 nanoseconds
    /// relative to the grandmaster, and the Talker schedules using
    /// a 100 microsecond timer tick driven by IEEE 802.1AS time,
    /// Jitter is 50000 (not 50812).
    ///
    /// The Talkers transmit offsets include
    /// earliest-transmit-offset, latest-transmit-offset, and the
    /// time-aware-offset returned to the Talker in
    /// group-status-talker-listener.interface-configuration.
    pub jitter: u32,
}

/// This traffic-specification specifies how the Talker transmits
/// frames for the Stream. This is effectively the Talkers promise
/// to the network. The network uses this traffic spec to allocate
/// resources and adjust queue parameters in Bridges.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrafficSpecificationContainer {
    /// This interval specifies the period of time in which the
    /// traffic specification cannot be exceeded. The traffic
    /// specification is specified by max-frames-per-interval and
    /// max-frame-size.
    ///
    /// The interval is a rational number of seconds, defined by an
    /// integer numerator and an integer denominator.
    ///
    /// If the time-aware container is not present, the interval
    /// specifies a sliding window of time. The Talkers transmission
    /// is not synchronized to a time on the network, and therefore
    /// the traffic specification cannot be exceeded during any
    /// interval in time.
    ///
    /// If the time-aware container is present, the interval specifies
    /// a window of time that is aligned with the time epoch that is
    /// synchronized on the network. For example, if IEEE Std
    /// 802.1AS-2011 is used with the PTP timescale, the first
    /// interval begins at 1 January 00:00:00 TAI. If CurrentTime
    /// represents the current time, then the start of the next
    /// interval (StartOfNextInterval) is: StartOfNextInterval = N *
    /// interval where N is the smallest integer for which the
    /// relation StartOfNextInterval >= CurrentTime would be TRUE.
    pub interval: TrafficSpecificationInterval,

    /// max-frames-per-interval specifies the maximum number of
    /// frames that the Talker can transmit in one interval.
    pub max_frames_per_interval: u16,

    /// max-frame-size specifies maximum frame size that the Talker
    /// will transmit, excluding any overhead for media-specific
    /// framing (e.g., preamble, IEEE 802.3 header, Priority/VID tag,
    /// CRC, interframe gap). As the Talker or Bridge determines the
    /// amount of bandwidth to reserve on the egress Port (interface),
    /// it will calculate the media-specific framing overhead on that
    /// Port and add it to the number specified in the max-frame-size
    /// leaf.
    pub max_frame_size: u16,

    /// transmission-selection specifies the algorithm that the
    /// Talker uses to transmit this Streams traffic class. This
    /// algorithm is often referred to as the shaper for the traffic
    /// class.
    ///
    /// The value for this leaf uses Table 8-5 (Transmission selection
    /// algorithm identifiers) of 8.6.8 of IEEE Std 802.1Q-2022. If no
    /// algorithm is known, the value zero (strict priority) can be
    /// used.
    ///
    /// The Talkers shaping and scheduling of the Stream is considered
    /// to be on the user side of the user/network boundary, and this
    /// leaf specifies the Talkers behavior to the network.
    pub transmission_selection: u8,

    /// The time-aware container provides leafs to specify the
    /// Talkers time-aware transmit to the network.
    ///
    /// The Talker and Listeners of a Stream are assumed to coordinate
    /// using user (application) mechanisms, such that each Listener
    /// is aware that its Talker transmits in a time-aware manner.
    ///
    /// If max-frames-per-interval is greater than one, the Talker
    /// shall transmit multiple frames as a burst within the interval,
    /// with the minimum inter-frame gap allowed by the media.
    ///
    /// NOTE: Although scheduled traffic (8.6.8.4 of IEEE Std
    /// 802.1Q-2022) specifies a valid implementation of a time-aware
    /// Talker, the time-aware container is intended to support
    /// alternate implementations of scheduling.
    pub time_aware: TimeAwareContainer,
}

/// This YANG grouping specifies: - Talkers behavior for Stream
/// (how/when transmitted) - Talkers requirements from the network -
/// TSN capabilities of the Talkers interface(s)
///
/// In the fully centralized model of TSN configuration, this grouping
/// originates from the CUC, and is delivered to the CNC.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupTalker {
    /// Rank of this Stream's configuration relative to other Streams
    /// in the network. This rank is used to determine success/failure
    /// of Stream resource configuration, and it is unrelated to the
    /// Streams data.
    pub stream_rank: StreamRankContainer,

    /// List of identifiers, one for each physical interface (distinct
    /// point of attachment) in the end station acting as a Talker.
    ///
    /// Although many end stations contain a single interface, this list
    /// allows for multiple interfaces. Some TSN features allow a single
    /// Stream to span multiple interfaces (e.g. seamless redundancy).
    ///
    /// Each entry of end-station-interfaces is used by the CNC to
    /// locate the Talker in the topology.
    ///
    /// Since the interface-name leaf is optional, empty string can be
    /// used for its key value.
    pub end_station_interfaces: Vec<GroupInterfaceId>,

    /// data-frame-specification specifies the frame that carries the
    /// Talkers Stream data. The network uses the specification to
    /// identify this Streams frames as TSN, in order to apply the
    /// required TSN configuration.
    ///
    /// The specification is based on the users knowledge of the frame,
    /// without any network specifics. In other words, this specifies
    /// the frame that the Talker would use in the absence of TSN.
    ///
    /// The specification is provided as a list of fields that the user
    /// knows. The list is ordered from start of frame to end of header.
    /// For example, if the Talker uses a VLAN-tagged Ethernet frame
    /// (not IP), the list consists of ieee802-mac-addresses followed by
    /// ieee802-vlan-tag. For example, if the Talker uses a UDP/IPv4
    /// packet without knowledge of the Ethernet header, the list
    /// consists of ipv4-tuple.
    ///
    /// This list is optional, and its absence indicates that Stream
    /// transformation is performed in the Talker and Listeners of this
    /// Stream (46.2.2 of IEEE Std 802.1Q-2022).
    pub data_frame_specification: Vec<DataFrameSpecificationElement>,

    /// This traffic-specification specifies how the Talker transmits
    /// frames for the Stream. This is effectively the Talkers promise
    /// to the network. The network uses this traffic spec to allocate
    /// resources and adjust queue parameters in Bridges.
    pub traffic_specification: TrafficSpecificationContainer,

    /// user-to-network-requirements specifies user requirements for
    /// the Stream, such as latency and redundancy. The network (CNC)
    /// will merge all user-to-network-requirements for a Stream to
    /// ensure that all requirements are met.
    pub user_to_network_requirements: GroupUserToNetworkRequirements,

    /// interface-capabilities specifies the network capabilities of
    /// all interfaces (Ports) contained in end-station-interfaces.
    ///
    /// The network may provide configuration of these capabilities in
    /// group-status-talker-listener.interface-configuration.
    ///
    /// NOTE If an end station contains multiple interfaces with
    /// different network capabilities, each interface should be
    /// specified as a distinct Talker or Listener (i.e. one entry in
    /// end-station-interfaces). Use of multiple entries in
    /// end-station-interfaces is intended for network capabilities that
    /// span multiple interfaces (e.g. seamless redundancy).
    pub interface_capabilities: GroupInterfaceCapabilities,
}

/// This YANG grouping specifies: - Listeners requirements from the
/// network - TSN capabilities of the Listeners interface(s)
///
/// In the fully centralized model of TSN configuration, this grouping
/// originates from the CUC, and is delivered to the CNC.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupListener {
    // TODO index needed... not in yang model
    pub index: u32,

    /// List of identifiers, one for each physical interface (distinct
    /// point of attachment) in the end station acting as a Listener.
    ///
    /// Although many end stations contain a single interface, this list
    /// allows for multiple interfaces. Some TSN features allow a single
    /// Stream to span multiple interfaces (e.g. seamless redundancy).
    ///
    /// Each entry of end-station-interfaces is used by the CNC to
    /// locate the Listener in the topology.
    ///
    /// Since the interface-name leaf is optional, empty string can be
    /// used for its key value.
    pub end_station_interfaces: Vec<GroupInterfaceId>,

    /// user-to-network-requirements specifies user requirements for
    /// the Stream, such as latency and redundancy. The network (CNC)
    /// will merge all user-to-network-requirements for a Stream to
    /// ensure that all requirements are met.
    pub user_to_network_requirements: GroupUserToNetworkRequirements,

    /// interface-capabilities specifies the network capabilities of
    /// all interfaces (Ports) contained in end-station-interfaces.
    ///
    /// The network may provide configuration of these capabilities in
    /// group-status-talker-listener.interface-configuration.
    ///
    /// NOTE If an end station contains multiple interfaces with
    /// different network capabilities, each interface should be
    /// specified as a distinct Talker or Listener (i.e. one entry in
    /// end-station-interfaces). Use of multiple entries in
    /// end-station-interfaces is intended for network capabilities that
    /// span multiple interfaces (e.g. seamless redundancy).
    pub interface_capabilities: GroupInterfaceCapabilities,
}
/// This is an enumeration for the status of the Streams Talker.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TalkerStatus {
    /// No Talker detected.
    None = 0,

    /// Talker ready (configured).
    Ready = 1,

    /// Talker failed.
    Failed = 2,
}

/// This is an enumeration for the status of the Streams
/// Listener(s).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ListenerStatus {
    /// No Listener detected.
    None = 0,

    /// All Listeners ready (configured).
    Ready = 1,

    /// One or more Listeners ready, and one or more Listeners
    /// failed. If Talker is ready, Stream can be used.
    PartialFailed = 2,

    /// All Listeners failed
    Failed = 3,
}

/// status-info provides information regarding the status of a
/// Streams configuration in the network.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusInfoContainer {
    /// This is an enumeration for the status of the Streams Talker.
    pub talker_status: TalkerStatus,

    /// This is an enumeration for the status of the Streams
    /// Listener(s).
    pub listener_status: ListenerStatus,

    /// If the Stream encounters a failure (talker-status is failed,
    /// or listener-status is failed, or listener-status is
    /// partial-failed), failure-code provides a non-zero code that
    /// specifies the problem. Table 46-15 of IEEE Std 802.1Q-2022
    /// describes each code.)
    pub failure_code: i32,
}

/// This YANG grouping provides the status of a Streams configuration
/// from the network to each user. The status in this grouping applies
/// to the entire Stream (Talker and all Listeners).
///
/// In the fully centralized model of TSN configuration, this grouping
/// originates from the CNC, and is delivered to the CUC.
///
/// The group-status-stream and group-status-talker-listener groupings
/// are intended to be used by other modules within a list of status
/// (state) for each Stream, with each list entry using: - leaf of
/// type stream-id-type, used as key to the list - container using
/// group-status-stream - container for Talker, using
/// group-status-talker-listener - list for Listeners, using
/// group-status-talker-listener
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupStatusStream {
    /// status-info provides information regarding the status of a
    /// Streams configuration in the network.
    pub status_info: StatusInfoContainer,

    /// When a failure occurs in network configuration (i.e. non-zero
    /// failure-code in status-info), failed-interfaces provides a list
    /// of one or more physical interfaces (distinct points of
    /// attachement) in the failed end station or Bridge. Each
    /// identifier is sufficient to locate the interface in the physical
    /// topology.
    ///
    /// The failed-interfaces list is optional.
    ///
    /// Since the interface-name leaf is optional, empty string can be
    /// used for its key value.
    pub failed_interfaces: Vec<GroupInterfaceId>,
}

/// This YANG grouping provides the status for a specific Talker or
/// Listener.
///
/// In the fully centralized model of TSN configuration, this grouping
/// originates from the CNC, and is delivered to the CUC.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupStatusTalkerListener {
    /// accumulated-latency provides the worst-case maximum latency
    /// that a single frame of the Stream can encounter along its
    /// current path(s).
    ///
    /// When provided to a Listener, accumulated-latency is the
    /// worst-case maximum latency for that Listener only.
    ///
    /// When provided to a Talker, accumulated-latency is the worst-case
    /// maximum latency for all Listeners (worst path).
    ///
    /// accumulated-latency is specified as an integer number of
    /// nanoseconds.
    ///
    /// accumulated-latency uses the same definition for latency as
    /// user-to-network-requirements.max-latency.
    ///
    /// For successful status-info, the network returns a value less
    /// than or equal to user-to-network-requirements.max-latency.
    ///
    /// If the time-aware container is present in the
    /// traffic-specification of the Talker, the value is expressed as
    /// nanoseconds after the start of the Talkers
    /// traffic-specification.interval.
    ///
    /// If the time-aware container is not present in the
    /// traffic-specification of the Talker, the value is expressed as
    /// nanoseconds after the Talkers transmit of any frame in the
    /// Stream, at any arbitrary time.
    ///
    /// If user-to-network-requirements.num-seamless-trees is one,
    /// accumulated-latency shall provide the worst-case maximum latency
    /// for the current path from Talker to each Listener. If the path
    /// is changed (e.g. by a spanning tree protocol),
    /// accumulated-latency changes accordingly.
    ///
    /// If user-to-network-requirements.num-seamless-trees is greater
    /// than one, accumulated-latency shall provide the worst-case
    /// maximum latency for all paths configured from the Talker to each
    /// Listener.
    pub accumulated_latency: u32,

    /// interface-configuration provides configuration of interfaces in
    /// the Talker/Listener. This configuration assists the network in
    /// meeting the Streams requirements. The interface-configuration
    /// meets the capabilities of the interface as provided in
    /// interface-capabilities.
    pub interface_configuration: GroupInterfaceConfiguration,
}

/// This packet is only viable for the specific b&r switch used in this paper. Since this Object is not (yet) present in the official IEEE Standard for TSN.
///
/// A list containing a set of the bridge port delays for every available port speed.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BridgePortDelays {
    /// Bridge port speed in Megabits per second (Mb/s).
    pub port_speed: u32,

    /// Dependent RX delay minimum.
    pub dependent_rx_delay_min: u64,

    /// Dependent RX delay maximum.
    pub dependent_rx_delay_max: u64,

    /// Independent RX delay minimum.
    pub independent_rx_delay_min: u64,

    /// Independent RX delay maximum.
    pub independent_rx_delay_max: u64,

    /// Independent relay delay minimum.
    pub independent_rly_delay_min: u64,

    /// Independent relay delay maximum.
    pub independent_rly_delay_max: u64,

    /// Independent TX delay minimum.
    pub independent_tx_delay_min: u64,

    /// Independent TX delay maximum.
    pub independent_tx_delay_max: u64,
}

impl BridgePortDelays {
    /// This creates an "empty" object. All fields are initialized with 0.
    pub fn new() -> Self {
        Self {
            port_speed: 0,
            dependent_rx_delay_max: 0,
            dependent_rx_delay_min: 0,
            independent_rly_delay_max: 0,
            independent_rly_delay_min: 0,
            independent_rx_delay_max: 0,
            independent_rx_delay_min: 0,
            independent_tx_delay_max: 0,
            independent_tx_delay_min: 0,
        }
    }
}
