pub mod middleware;
pub mod northbound;
pub mod southbound;
pub mod storage;
pub mod topology;
pub mod types;

use self::middleware::types::FailedStream;
use self::middleware::SchedulerAdapterInterface;
use self::northbound::{NorthboundAdapterInterface, NorthboundControllerInterface};
use self::southbound::SouthboundAdapterInterface;
use self::storage::StorageAdapterInterface;
use self::topology::{TopologyAdapterInterface, TopologyControllerInterface};
use self::types::computation::ComputationType;
use self::types::notification_types::{self, NotificationContent};
use self::types::tsn_types::StreamIdTypeUpper;
use self::types::uni_types::{self, Stream};
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
    /// This runs the CNC continuosly unless Components stop running.
    /// This function call is blocking until the CNC stops operation.
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
        println!("[Scheduler] preparing computation...");

        let topology = cnc.topology.get_topology();
        let domains = cnc.get_domains_to_compute(computation_type);

        println!("[Scheduler] computing schedule now...");

        let computation_result = cnc.scheduler.compute_schedule(&topology, &domains);

        println!(
            "[Scheduler] computation finished - with {} streams failed",
            computation_result.failed_streams.len()
        );

        // TODO handle failed interfaces...
        // sets interface configurations of talker/listeners to storage
        cnc.storage.modify_streams(&computation_result.domains);
        let computed_notification: NotificationContent =
            create_computation_notification(&domains, &computation_result.failed_streams);
        cnc.northbound
            .compute_streams_completed(computed_notification);

        println!("[Scheduler] configuring now...");

        let failed_interfaces = cnc
            .southbound
            .configure_network(&topology, &computation_result.schedule);

        println!(
            "[Scheduler] configuring finished - with {} failed interfaces",
            failed_interfaces.interfaces.len()
        );

        // TODO rework these...
        // let failed_streams = get_failed_streams(&computation_result, &failed_nodes);
        // cnc.storage
        //     .set_streams_configured(&domains, &failed_streams);
        // cnc.storage.set_configs(&computation_result.configs);

        // let notification: NotificationContent = create_configuration_notification(&domains, &failed_streams);
        // set_failed_streams_in_storage(&domains, &failed_nodes, &topology, &failed_streams);
        // cnc.northbound.configure_streams_completed(notification);
    }

    fn get_domains_to_compute(&self, computation: ComputationType) -> Vec<uni_types::Domain> {
        let domains: Vec<uni_types::Domain> = match computation {
            ComputationType::All(request_domains) => {
                self.storage.get_streams_in_domains(request_domains)
            }

            ComputationType::PlannedAndModified(request_domains) => self
                .storage
                .get_planned_and_modified_streams_in_domains(request_domains),

            ComputationType::List(request_domains) => {
                self.storage.get_streams_in_domains(request_domains)
            }
        };

        return domains;
    }
}

fn create_computation_notification(
    domains: &Vec<uni_types::Domain>,
    failed_streams: &Vec<FailedStream>,
) -> NotificationContent {
    let mut notification: NotificationContent = Vec::new();

    for domain in domains.iter() {
        let mut notification_domain = notification_types::Domain {
            domain_id: domain.domain_id.clone(),
            cucs: Vec::new(),
        };

        for cuc in domain.cuc.iter() {
            let mut notification_cuc = notification_types::Cuc {
                cuc_id: cuc.cuc_id.clone(),
                streams: Vec::new(),
            };

            for stream in cuc.stream.iter() {
                let mut failure_code: u8 = 0;

                let failed_in_computation = failed_streams.iter().find(|x| {
                    x.stream_id == stream.stream_id
                        && x.cuc_id == cuc.cuc_id
                        && x.domain_id == domain.domain_id
                });

                if failed_in_computation.is_some() {
                    failure_code = 1;
                }

                let notification_stream = notification_types::Stream {
                    stream_id: stream.stream_id.clone(),
                    failure_code,
                };

                notification_cuc.streams.push(notification_stream);
            }

            notification_domain.cucs.push(notification_cuc);
        }

        notification.push(notification_domain);
    }
    notification
}

fn create_configuration_notification(
    domains: &Vec<uni_types::Domain>,
    failed_interfaces: &Vec<southbound::types::FailedInterface>,
) -> NotificationContent {
    let mut notification: NotificationContent = Vec::new();

    for domain in domains.iter() {
        let mut notification_domain = notification_types::Domain {
            domain_id: domain.domain_id.clone(),
            cucs: Vec::new(),
        };

        for cuc in domain.cuc.iter() {
            let mut notification_cuc = notification_types::Cuc {
                cuc_id: cuc.cuc_id.clone(),
                streams: Vec::new(),
            };

            for stream in cuc.stream.iter() {
                let mut failure_code: u8 = 0;

                let failed_with_some_interface = failed_interfaces
                    .iter()
                    .find(|x| x.affected_streams.contains(&stream.stream_id));

                if failed_with_some_interface.is_some() {
                    failure_code = 1;
                }

                let notification_stream = notification_types::Stream {
                    stream_id: stream.stream_id.clone(),
                    failure_code,
                };

                notification_cuc.streams.push(notification_stream);
            }

            notification_domain.cucs.push(notification_cuc);
        }

        notification.push(notification_domain);
    }
    notification
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
            StreamIdTypeUpper,
            types::tsn_types::GroupTalker,
            Vec<types::tsn_types::GroupListener>,
        )>,
    ) {
        // TODO parse request and add to storage
        // TODO status groups and normal groups. When does everthig get created
        let mut streams: Vec<Stream> = Vec::new();

        for requested_stream in request {
            let s = Stream {
                stream_id: requested_stream.0,
                stream_status: types::uni_types::StreamStatus::Planned,
                talker: types::uni_types::Talker {
                    group_talker: requested_stream.1.clone(),
                    group_status_talker_listener: types::tsn_types::GroupStatusTalkerListener {
                        accumulated_latency: 0,
                        interface_configuration: types::tsn_types::GroupInterfaceConfiguration {
                            interface_list: Vec::new(),
                        },
                    },
                },
                listener: vec![types::uni_types::Listener {
                    index: 0,
                    group_listener: requested_stream.2[0].clone(),
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
            streams.push(s);
        }
        self.storage.set_streams(cuc_id, &streams);
    }
}

impl TopologyControllerInterface for Cnc {
    fn notify_topology_changed(&self) {
        // println!("[CNC] TODO: got notified about TopologyChange. But doing nothing about it...");
    }
}
