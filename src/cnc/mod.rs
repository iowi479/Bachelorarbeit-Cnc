pub mod middleware;
pub mod northbound;
pub mod southbound;
pub mod storage;
pub mod topology;
pub mod types;

use self::middleware::SchedulerAdapterInterface;
use self::northbound::{NorthboundAdapterInterface, NorthboundControllerInterface};
use self::southbound::SouthboundAdapterInterface;
use self::storage::StorageAdapterInterface;
use self::topology::{TopologyAdapterInterface, TopologyControllerInterface};
use self::types::computation::ComputationType;
use self::types::notification_types::{self, NotificationContent};
use self::types::scheduling::Schedule;
use self::types::topology::Topology;
use self::types::tsn_types::GroupInterfaceId;
use self::types::uni_types::{self, Stream};
use std::collections::HashSet;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Weak};

pub type NorthboundRef = Arc<dyn NorthboundAdapterInterface + Send + Sync>;
pub type SouthboundRef = Arc<dyn SouthboundAdapterInterface + Send + Sync>;
pub type StorageRef = Arc<dyn StorageAdapterInterface + Send + Sync>;
pub type TopologyRef = Arc<dyn TopologyAdapterInterface + Send + Sync>;
pub type SchedulerRef = Arc<dyn SchedulerAdapterInterface + Send + Sync>;

pub static CNC_NOT_PRESENT: &str = "CNC is not present exiting...";

pub struct Cnc {
    pub id: u32,
    pub domain: String,
    schedule_computation_sender: Sender<ComputationType>,
    northbound: NorthboundRef,
    southbound: SouthboundRef,
    storage: StorageRef,
    topology: TopologyRef,
    scheduler: SchedulerRef,
}

impl Cnc {
    pub fn run(
        id: u32,
        domain: String,
        mut northbound: NorthboundRef,
        mut southbound: SouthboundRef,
        mut storage: StorageRef,
        mut topology: TopologyRef,
        mut scheduler: SchedulerRef,
    ) {
        // Channel for starting a computation
        let (schedule_computation_sender, schedule_computation_receiver): (
            Sender<ComputationType>,
            Receiver<ComputationType>,
        ) = mpsc::channel();

        let cnc: Arc<Self> = Arc::new_cyclic(|my_weak_ref: &Weak<Self>| {
            // configure all components
            Arc::get_mut(&mut northbound)
                .unwrap()
                .set_cnc_ref(my_weak_ref.clone());

            Arc::get_mut(&mut southbound)
                .unwrap()
                .set_cnc_ref(my_weak_ref.clone());

            Arc::get_mut(&mut storage)
                .unwrap()
                .set_cnc_ref(my_weak_ref.clone());

            Arc::get_mut(&mut topology)
                .unwrap()
                .set_cnc_ref(my_weak_ref.clone());

            Arc::get_mut(&mut scheduler)
                .unwrap()
                .set_cnc_ref(my_weak_ref.clone());

            Self {
                id,
                domain,
                schedule_computation_sender,
                northbound,
                southbound,
                storage,
                topology,
                scheduler,
            }
        });
        println!(
            "[CNC] id: {} - Successfully configured. Its now ready for use...",
            cnc.id
        );

        // configuration of all Components
        cnc.northbound.run();
        cnc.topology.run();
        cnc.storage.configure_storage();

        // wait for computation-requests
        for computation_type in schedule_computation_receiver {
            Cnc::execute_computation(cnc.clone(), computation_type);
        }

        println!("[CNC] stopped...");
        drop(cnc);
    }

    fn execute_computation(cnc: Arc<Cnc>, computation_type: ComputationType) {
        println!("[SCHEDULER] preparing computation...");

        let topology: Topology = cnc.topology.get_topology();
        let domains: Vec<uni_types::Domain> = match computation_type {
            ComputationType::All(request_domains) => {
                cnc.storage.get_streams_in_domains(request_domains)
            }

            ComputationType::PlannedAndModified(request_domains) => cnc
                .storage
                .get_planned_and_modified_streams_in_domains(request_domains),

            ComputationType::List(request_domains) => {
                cnc.storage.get_streams_in_domains(request_domains)
            }
        };

        dbg!(&domains);

        println!("[SCHEDULER] computing now...");

        let schedule = cnc.scheduler.compute_schedule(&topology, &domains);

        println!("[SCHEDULER] computation successfull");

        // TODO faild computations
        let mut computed_notification: NotificationContent = Vec::new();
        for domain in &domains {
            let mut d = notification_types::Domain {
                domain_id: domain.domain_id.clone(),
                cucs: Vec::new(),
            };

            for cuc in &domain.cuc {
                let mut c = notification_types::Cuc {
                    cuc_id: cuc.cuc_id.clone(),
                    streams: Vec::new(),
                };

                for stream in &cuc.stream {
                    c.streams.push(notification_types::Stream {
                        stream_id: stream.stream_id.clone(),
                        failure_code: 0,
                    })
                }

                d.cucs.push(c);
            }

            computed_notification.push(d);
        }

        cnc.northbound
            .compute_streams_completed(computed_notification);

        println!("[SCHEDULER] configuring now...");

        let failed_nodes = cnc.southbound.configure_network(&topology, &schedule);
        let failed_streams = get_failed_streams(&schedule, failed_nodes);

        println!("[SCHEDULER] configuring successfull");

        cnc.storage
            .set_streams_configured(&domains, &failed_streams);
        cnc.storage.set_configs(&schedule.configs);

        // TODO mock notification
        let mut notification: NotificationContent = Vec::new();
        for domain in domains.iter() {
            let mut d = notification_types::Domain {
                domain_id: domain.domain_id.clone(),
                cucs: Vec::new(),
            };

            for cuc in domain.cuc.iter() {
                let mut c = notification_types::Cuc {
                    cuc_id: cuc.cuc_id.clone(),
                    streams: Vec::new(),
                };

                for stream in cuc.stream.iter() {
                    let mut failure_code: u8 = 0;

                    if failed_streams.get(&stream.stream_id).is_some() {
                        failure_code = 1;
                    }

                    let s = notification_types::Stream {
                        stream_id: stream.stream_id.clone(),
                        failure_code,
                    };

                    c.streams.push(s);
                }

                d.cucs.push(c);
            }

            notification.push(d);
        }
        cnc.northbound.configure_streams_completed(notification);
    }
}

fn get_failed_streams(schedule: &Schedule, failed_nodes: HashSet<u32>) -> HashSet<String> {
    let mut failed_streams: HashSet<String> = HashSet::new();

    for config in &schedule.configs {
        if failed_nodes.get(&config.node_id).is_some() {
            for stream_id in &config.for_streams {
                failed_streams.insert(stream_id.clone());
            }
        }
    }

    failed_streams
}

impl NorthboundControllerInterface for Cnc {
    fn compute_streams(
        &self,
        computation: ComputationType,
    ) -> types::uni_types::compute_streams::Output {
        match self.schedule_computation_sender.send(computation) {
            Ok(_) => String::from("Success"),
            Err(e) => e.to_string(),
        }
    }

    fn remove_streams(
        &self,
        cuc_id: &String,
        input: types::uni_types::remove_streams::Input,
    ) -> types::uni_types::remove_streams::Output {
        for stream_id in input.iter() {
            self.storage.remove_stream(cuc_id, stream_id.clone());
        }
        // TODO what gets returned?? -> Success?
        String::from("Success")
    }

    fn request_domain_id(
        &self,
        input: types::uni_types::request_domain_id::Input,
    ) -> types::uni_types::request_domain_id::Output {
        return match self.storage.get_domain_id_of_cuc(input) {
            None => String::from("Failure"),
            Some(domain_id) => domain_id,
        };
    }

    fn request_free_stream_id(
        &self,
        input: types::uni_types::request_free_stream_id::Input,
    ) -> types::uni_types::request_free_stream_id::Output {
        match self
            .storage
            .get_free_stream_id(input.domain_id, input.cuc_id)
        {
            Some(id) => id,
            None => String::from("no id"),
        }
    }

    // TODO tuple als type definieren
    fn set_streams(
        &self,
        cuc_id: &String,
        request: Vec<(
            types::tsn_types::GroupTalker,
            Vec<types::tsn_types::GroupListener>,
        )>,
    ) {
        // TODO parse request and add to storage
        // TODO status groups and normal groups. When does everthig get created

        let s: Stream = Stream {
            stream_id: String::from("00-00-00-00-00-00:00-01"),
            stream_status: types::uni_types::StreamStatus::Planned,
            talker: types::uni_types::Talker {
                group_talker: request[0].0.clone(),
                group_status_talker_listener: types::tsn_types::GroupStatusTalkerListener {
                    accumulated_latency: 0,
                    interface_configuration: types::tsn_types::GroupInterfaceConfiguration {
                        interface_list: vec![types::tsn_types::InterfaceListElement {
                            config_list: vec![types::tsn_types::ConfigListElement {
                                index: 0,
                                config_value: types::tsn_types::ConfigValue::Ieee802MacAddresses(
                                    types::tsn_types::GroupIeee802MacAddress {
                                        destination_mac_adress: String::from("00-00-00-0F-00-00"),
                                        source_mac_adress: String::from("00-00-00-00-00-01"),
                                    },
                                ),
                            }],
                            group_interface_id: GroupInterfaceId {
                                interface_name: String::new(),
                                mac_address: String::new(),
                            },
                        }],
                    },
                },
            },
            listener: vec![types::uni_types::Listener {
                index: 0,
                group_listener: request[0].1[0].clone(),
                group_status_talker_listener: types::tsn_types::GroupStatusTalkerListener {
                    accumulated_latency: 0,
                    interface_configuration: types::tsn_types::GroupInterfaceConfiguration {
                        interface_list: Vec::new(),
                    },
                },
            }],
            group_status_stream: types::tsn_types::GroupStatusStream {
                status_info: types::tsn_types::StatusInfoContainer {
                    talker_status: types::tsn_types::TalkerStatus::None,
                    listener_status: types::tsn_types::ListenerStatus::None,
                    failure_code: 0,
                },
                failed_interfaces: Vec::new(),
            },
        };

        self.storage.set_stream(cuc_id, &s);
    }
}

impl TopologyControllerInterface for Cnc {
    fn notify_topology_changed(&self) {
        // println!("[CNC] TODO: got notified about TopologyChange. But doing nothing about it...");
    }
}
