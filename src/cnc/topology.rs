use super::types::topology::{
    Connection, ConnectionInterface, NodeInformation, NodeType, Path, SSHConfigurationParams,
    Topology,
};
use super::{Cnc, CNC_NOT_PRESENT};
use std::sync::{RwLock, Weak};
use std::thread;
use std::time::Duration;

pub trait TopologyControllerInterface {
    /// on a detected topology change, this gets called.
    /// Any actions the cnc should take have to be called in here...
    fn notify_topology_changed(&self);
}

pub trait TopologyAdapterInterface {
    /// returnes to currently available Topology-Information
    fn get_topology(&self) -> Topology;

    /// returns information about a specific node
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

impl Topology {
    pub fn get_node_from_id(&self, node_id: u32) -> Option<NodeInformation> {
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
    pub fn new_failing() -> Self {
        let mut nodes: Vec<NodeInformation> = Vec::new();
        let mut connections: Vec<Connection> = Vec::new();
        let mut paths: Vec<Path> = Vec::new();

        /*
        For this Topology node (2) doesnt exist and thus will fail to be configured.
        (Switches) [EndStations]

        ------- Mock Topology -----
                  (1) --- (2)
                 /   \       \
               [10] [11]    [12]
        ---------------------------
        */

        nodes.push(NodeInformation {
            id: 1,
            mac_addresses_interfaces: vec![
                String::from("00-60-65-82-c9-5b"),
                String::from("00-60-65-82-c9-5c"),
                String::from("00-60-65-82-c9-5d"),
            ],
            endstation: NodeType::Bridge,
            ports: Vec::new(),
            configuration_params: Some(SSHConfigurationParams {
                ip: String::from("10.2.0.1"),
                port: 830,
                username: String::from("admin"),
                password: String::from("admin"),
            }),
        });

        nodes.push(NodeInformation {
            id: 2,
            mac_addresses_interfaces: vec![
                String::from("00-10-02-00-02-02"),
                String::from("00-10-02-00-02-03"),
            ],
            endstation: NodeType::Bridge,
            ports: Vec::new(),
            configuration_params: Some(SSHConfigurationParams {
                ip: String::from("10.2.0.2"),
                port: 830,
                username: String::from(""),
                password: String::from(""),
            }),
        });

        nodes.push(NodeInformation {
            id: 10,
            mac_addresses_interfaces: vec![String::from("00-00-00-00-00-01")],
            endstation: NodeType::EndStation,
            ports: Vec::new(),
            configuration_params: None,
        });

        nodes.push(NodeInformation {
            id: 11,
            mac_addresses_interfaces: vec![String::from("00-00-00-00-00-02")],
            endstation: NodeType::EndStation,
            ports: Vec::new(),
            configuration_params: None,
        });
        nodes.push(NodeInformation {
            id: 12,
            mac_addresses_interfaces: vec![String::from("00-00-00-00-00-03")],
            endstation: NodeType::EndStation,
            ports: Vec::new(),
            configuration_params: None,
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
                port_name: String::from("eth1"),
            },
            b: ConnectionInterface {
                node_id: 1,
                port_name: String::from("sw0p3"),
            },
        });

        connections.push(Connection {
            id: 2,
            a: ConnectionInterface {
                node_id: 1,
                port_name: String::from("sw0p4"),
            },
            b: ConnectionInterface {
                node_id: 2,
                port_name: String::from("sw0p2"),
            },
        });

        connections.push(Connection {
            id: 3,
            a: ConnectionInterface {
                node_id: 2,
                port_name: String::from("sw0p3"),
            },
            b: ConnectionInterface {
                node_id: 12,
                port_name: String::from("eth0"),
            },
        });

        paths.push(Path {
            node_a_id: 10,
            node_b_id: 11,
            hops: vec![1],
        });
        paths.push(Path {
            node_a_id: 10,
            node_b_id: 12,
            hops: vec![1, 2],
        });
        paths.push(Path {
            node_a_id: 11,
            node_b_id: 12,
            hops: vec![1, 2],
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

    pub fn new_functioning() -> Self {
        let mut nodes: Vec<NodeInformation> = Vec::new();
        let mut connections: Vec<Connection> = Vec::new();
        let mut paths: Vec<Path> = Vec::new();

        /*
        --- Mock Topology -----
                  (1)
                 /   \
               [10] [11]
        -----------------------
        */

        nodes.push(NodeInformation {
            id: 1,
            mac_addresses_interfaces: vec![
                String::from("00-60-65-82-c9-5b"),
                String::from("00-60-65-82-c9-5c"),
                String::from("00-60-65-82-c9-5d"),
            ],
            endstation: NodeType::Bridge,
            ports: Vec::new(),
            configuration_params: Some(SSHConfigurationParams {
                ip: String::from("10.2.0.1"),
                port: 830,
                username: String::from("admin"),
                password: String::from("admin"),
            }),
        });

        nodes.push(NodeInformation {
            id: 10,
            mac_addresses_interfaces: vec![String::from("00-00-00-00-00-01")],
            endstation: NodeType::EndStation,
            ports: Vec::new(),
            configuration_params: None,
        });

        nodes.push(NodeInformation {
            id: 11,
            mac_addresses_interfaces: vec![String::from("00-00-00-00-00-02")],
            endstation: NodeType::EndStation,
            ports: Vec::new(),
            configuration_params: None,
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
                port_name: String::from("eth1"),
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
        let nodes = &self.topology.read().unwrap().nodes;
        return nodes.iter().find(|x| x.id == id).cloned();
    }

    fn get_topology(&self) -> Topology {
        return self.topology.read().unwrap().clone();
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }

    fn run(&self) {
        let cnc = self.cnc.upgrade().expect(CNC_NOT_PRESENT).clone();

        // this simulates a topology change every 15 seconds
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(15));
            cnc.notify_topology_changed();
        });
    }
}
