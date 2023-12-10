use super::{types::tsn_types::BridgePortDelays, Cnc};
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::{RwLock, Weak},
    thread,
    time::Duration,
};

pub trait TopologyControllerInterface {
    fn notify_topology_changed(&self) {}
}

pub trait TopologyAdapterInterface {
    fn get_topology(&self) -> Topology;
    fn get_node_information(&self, id: u32) -> Option<NodeInformation>;

    /// running this component continously
    ///
    /// possibly in a new Thread
    ///
    /// # Important
    /// This has to be non-blocking!
    fn run(&self);

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

#[derive(Clone)]
pub enum NodeType {
    Bridge,
    EndStation,
}

#[derive(Clone)]
pub struct ConnectionInterface {
    node_id: u32,
    port_name: String,
}

#[derive(Clone)]
pub struct Connection {
    id: u32,
    a: ConnectionInterface,
    b: ConnectionInterface,
}

#[derive(Clone)]
pub struct NodeInformation {
    pub id: u32,
    pub ip: IpAddr,
    pub endstation: NodeType,
    pub ports: Vec<Port>,
}

#[derive(Clone)]
pub struct Port {
    pub name: String,
    pub delays: Vec<BridgePortDelays>,
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

impl Topology {
    pub fn get_node(&self, node_id: u32) -> Option<NodeInformation> {
        for node in self.nodes.iter() {
            if node.id == node_id {
                return Some(node.clone());
            }
        }
        return None;
    }
}

pub struct MockTopology {
    topology: RwLock<Topology>,
    cnc: Weak<Cnc>,
}

impl MockTopology {
    pub fn new() -> Self {
        let mut nodes: Vec<NodeInformation> = Vec::new();
        let mut connections: Vec<Connection> = Vec::new();
        let mut paths: Vec<Path> = Vec::new();

        /*
        --- Mock Topology -----
                  (1)
                 /   \
               [10]  11]
        -----------------------
        */

        nodes.push(NodeInformation {
            id: 1,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            endstation: NodeType::Bridge,
            ports: Vec::new(),
        });

        nodes.push(NodeInformation {
            id: 10,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 10)),
            endstation: NodeType::EndStation,
            ports: Vec::new(),
        });

        nodes.push(NodeInformation {
            id: 11,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 11)),
            endstation: NodeType::EndStation,
            ports: Vec::new(),
        });

        connections.push(Connection {
            id: 0,
            a: ConnectionInterface {
                node_id: 10,
                port_name: String::from("eth0"),
            },
            b: ConnectionInterface {
                node_id: 1,
                port_name: String::from("sw0p2"),
            },
        });

        connections.push(Connection {
            id: 1,
            a: ConnectionInterface {
                node_id: 11,
                port_name: String::from("eth0"),
            },
            b: ConnectionInterface {
                node_id: 1,
                port_name: String::from("sw0p3"),
            },
        });

        paths.push(Path {
            node_a_id: 10,
            node_b_id: 11,
            hops: vec![01],
        });

        let topology: Topology = Topology {
            nodes,
            connections,
            paths: Some(paths),
        };

        Self {
            topology: RwLock::new(topology),
            cnc: Weak::default(),
        }
    }
}

impl TopologyAdapterInterface for MockTopology {
    fn get_node_information(&self, id: u32) -> Option<NodeInformation> {
        for node in self.topology.read().unwrap().nodes.iter() {
            if node.id == id {
                return Some(node.clone());
            }
        }
        return None;
    }

    fn get_topology(&self) -> Topology {
        return self.topology.read().unwrap().clone();
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }

    fn run(&self) {
        let cnc = self.cnc.upgrade().unwrap().clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));
            thread::sleep(Duration::from_secs(10));
            cnc.notify_topology_changed();
            // println!("[Topology] Topology Changed");
        });
    }
}
