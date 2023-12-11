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
pub mod sched_types;

/// Rust-Types for YANG-Models
///
/// https://github.com/YangModels/yang/blob/main/standard/ieee/draft/802.1/Qdj/ieee802-dot1q-tsn-types.yang
pub mod tsn_types;

/// Rust-Types for YANG-Models
///
/// https://github.com/YangModels/yang/blob/main/standard/ieee/draft/802.1/Qdj/ieee802-dot1q-tsn-config-uni.yang
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

    pub struct Schedule {
        pub configs: Vec<crate::cnc::storage::Config>,
        // TODO impl computed Schedule
    }
}

pub mod computation {
    pub enum ComputationType {
        All(super::uni_types::stream_request::Input),
        PlannedAndModified(super::uni_types::stream_request::Input),
        List(super::uni_types::stream_request::Input),
    }
}
