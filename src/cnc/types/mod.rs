use self::scheduling::Schedule;
use self::tsn_types::{GroupInterfaceId, StreamIdTypeUpper};
use self::uni_types::Domain;
use std::collections::HashSet;

/// Rust-Types for YANG-Models
///
/// Specific Types for the specified notifications.
///
/// Since they dont include big parts of the modells, these are defined as extra types.
///
/// https://github.com/YangModels/yang/blob/main/standard/ieee/draft/802.1/Qdj/ieee802-dot1q-tsn-types.yang
pub mod notification_types;

/// Rust-Types for YANG-Models
///
/// https://github.com/YangModels/yang/blob/main/standard/ieee/draft/802.1/Qcw/ieee802-dot1q-sched.yang
/// also the b&r model: ieee802-dot1q-bridge-delays
///
/// # Description
/// This module provides for management of IEEE Std 802.1Q Bridges that
/// support Scheduled Traffic Enhancements.
///
/// Copyright (C) IEEE (2023).
///
/// This version of this YANG module is part of IEEE Std 802.1Q; see the
/// standard itself for full legal notices.
pub mod sched_types;

/// Rust-Types for YANG-Models
///
/// https://github.com/YangModels/yang/blob/main/standard/ieee/draft/802.1/Qdj/ieee802-dot1q-tsn-types.yang
///
/// # Description
/// Common typedefs and groupings for TSN user/network configuration in
/// IEEE Std 802.1Q.
///
/// Copyright (C) IEEE (2022).
///
/// This version of this YANG module is part of IEEE Std 802.1Q; see the
/// standard itself for full legal notices.
pub mod tsn_types;

/// Rust-Types for YANG-Models
///
/// https://github.com/YangModels/yang/blob/main/standard/ieee/draft/802.1/Qdj/ieee802-dot1q-tsn-config-uni.yang
///
/// # Description
/// Time-Sensitive Networking (TSN) User/Network Interface (UNI) for the
/// exchange of information between CUC and CNC that are required to
/// configure TSN Streams in a TSN network.
pub mod uni_types;

pub mod lldp_types;

pub mod topology {
    #[derive(Clone)]
    pub enum NodeType {
        Bridge,
        EndStation,
    }

    #[derive(Clone)]
    pub struct ConnectionInterface {
        pub node_id: u32,
        pub port_name: String,
    }

    #[derive(Clone)]
    pub struct Connection {
        pub id: u32,
        pub a: ConnectionInterface,
        pub b: ConnectionInterface,
    }

    #[derive(Clone)]
    pub struct NodeInformation {
        pub id: u32,
        pub mac_addresses_interfaces: Vec<String>,
        pub endstation: NodeType,
        pub configuration_params: Option<SSHConfigurationParams>,
        pub ports: Vec<Port>,
    }

    #[derive(Debug, Clone)]
    pub struct SSHConfigurationParams {
        pub ip: String,
        pub port: u16,
        pub username: String,
        pub password: String,
    }

    #[derive(Clone, Debug)]
    pub struct Port {
        pub name: String,
        pub mac_address: String,
        pub delays: Vec<super::tsn_types::BridgePortDelays>,
        pub tick_granularity: u32,
    }

    #[derive(Clone)]
    pub struct Path {
        pub node_a_id: u32,
        pub node_b_id: u32,
        pub hops: Vec<u32>,
    }

    #[derive(Clone)]
    pub struct Topology {
        pub nodes: Vec<NodeInformation>,
        pub connections: Vec<Connection>,
        pub paths: Option<Vec<Path>>,
    }
}

pub mod scheduling {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Config {
        pub port: PortConfiguration,
        pub node_id: u32,
        pub affected_streams: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct PortConfiguration {
        pub name: String,
        pub mac_address: String,
        pub config: super::sched_types::ConfigurableGateParameterTableEntry,
    }

    #[derive(Clone, Debug)]
    pub struct Schedule {
        pub configs: Vec<Config>,
    }
}

pub mod computation {
    pub enum ComputationType {
        All(super::uni_types::compute_streams::Input),
        PlannedAndModified(super::uni_types::compute_streams::Input),
        List(super::uni_types::compute_streams::Input),
    }
}

/// This struct provides information about failed configurations
pub struct FailedInterfaces {
    pub interfaces: Vec<FailedInterface>,
}

/// This struct is provided for each interface that failed configuration.
/// This Information is essential for the CNC to further configure streams.
pub struct FailedInterface {
    pub interface: GroupInterfaceId,
    pub node_id: u32,
    pub affected_streams: HashSet<StreamIdTypeUpper>,
}

pub struct ComputationResult {
    pub schedule: Schedule,
    pub domains: Vec<Domain>,
    pub failed_streams: Vec<FailedStream>,
}

pub struct FailedStream {
    pub stream_id: StreamIdTypeUpper,
    pub cuc_id: String,
    pub domain_id: String,
    pub failure_code: u32,
}

pub struct StreamRequest {
    pub stream_id: StreamIdTypeUpper,
    pub talker: tsn_types::GroupTalker,
    pub listeners: Vec<tsn_types::GroupListener>,
}
