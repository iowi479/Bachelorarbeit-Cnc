use super::Cnc;
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Weak,
    thread,
    time::Duration,
};

pub trait TopologyControllerInterface {
    fn notify_topology_changed(&self);
}

pub trait TopologyAdapterInterface {
    fn get_topology(&self) -> &Topology;
    fn get_node_information(&self, id: u32) -> Option<&NodeInformation>;

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
    /// self.cnc = Some(cnc);
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

pub enum NodeType {
    Bridge,
    EndStation,
}

pub type Connection = (u32, u32);

pub struct NodeInformation {
    pub id: u32,
    pub ip: IpAddr,
    pub endstation: NodeType,
}

pub struct Path {
    pub node_a_id: u32,
    pub node_b_id: u32,
    pub hops: Vec<u32>,
}

pub struct Topology {
    pub nodes: Vec<NodeInformation>,
    pub connections: Vec<Connection>,
    pub paths: Option<Vec<Path>>,
}

pub struct MockTopology {
    topology: Topology,
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
               (2)   (3)
              /     /   \
            [10]  [11] [12]
        -----------------------
        */

        nodes.push(NodeInformation {
            id: 1,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            endstation: NodeType::Bridge,
        });
        nodes.push(NodeInformation {
            id: 2,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)),
            endstation: NodeType::Bridge,
        });
        nodes.push(NodeInformation {
            id: 3,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 3)),
            endstation: NodeType::Bridge,
        });

        nodes.push(NodeInformation {
            id: 10,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 10)),
            endstation: NodeType::EndStation,
        });
        nodes.push(NodeInformation {
            id: 11,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 11)),
            endstation: NodeType::EndStation,
        });
        nodes.push(NodeInformation {
            id: 12,
            ip: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 12)),
            endstation: NodeType::EndStation,
        });

        connections.push((1, 2));
        connections.push((1, 3));
        connections.push((2, 10));
        connections.push((3, 11));
        connections.push((3, 12));

        paths.push(Path {
            node_a_id: 10,
            node_b_id: 11,
            hops: vec![10, 2, 1, 3, 11],
        });

        paths.push(Path {
            node_a_id: 10,
            node_b_id: 12,
            hops: vec![10, 2, 1, 3, 12],
        });

        paths.push(Path {
            node_a_id: 11,
            node_b_id: 12,
            hops: vec![11, 3, 12],
        });

        let topology: Topology = Topology {
            nodes,
            connections,
            paths: Some(paths),
        };

        Self {
            topology,
            cnc: Weak::default(),
        }
    }
}

impl TopologyAdapterInterface for MockTopology {
    fn get_node_information(&self, id: u32) -> Option<&NodeInformation> {
        for node in self.topology.nodes.iter() {
            if node.id == id {
                return Some(node);
            }
        }
        return None;
    }

    fn get_topology(&self) -> &Topology {
        &self.topology
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }

    fn run(&self) {
        let cnc = self.cnc.upgrade().unwrap().clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));
            println!("[Topology] Topology Changed");
            cnc.notify_topology_changed();
        });
    }
}
