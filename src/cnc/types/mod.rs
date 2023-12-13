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
        pub ip: std::net::IpAddr,
        pub endstation: NodeType,
        pub configuration_params: Option<(u32, String)>,
        pub ports: Vec<Port>,
    }

    #[derive(Clone)]
    pub struct Port {
        pub name: String,
        pub delays: Vec<super::tsn_types::BridgePortDelays>,
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

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Config {
        pub node_id: u32,
        pub ports: Vec<PortConfiguration>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PortConfiguration {
        pub name: String,
        pub config: super::sched_types::ConfigurableGateParameterTableEntry,
    }

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
