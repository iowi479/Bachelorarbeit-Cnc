use super::middleware::SchedulerAdapterInterface;
use super::northbound::{NorthboundAdapterInterface, NorthboundControllerInterface};
use super::southbound::SouthboundAdapterInterface;
use super::storage::StorageAdapterInterface;
use super::topology::{TopologyAdapterInterface, TopologyControllerInterface};
use super::types;
use super::types::computation::ComputationType;
use super::types::notification_types::{self, NotificationContent};
use super::types::topology::Topology;
use super::types::uni_types::{self, Stream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Weak};

pub static CNC_NOT_PRESENT: &str = "CNC is not present exiting...";

pub type NorthboundRef = Arc<dyn NorthboundAdapterInterface + Send + Sync>;
pub type SouthboundRef = Arc<dyn SouthboundAdapterInterface + Send + Sync>;
pub type StorageRef = Arc<dyn StorageAdapterInterface + Send + Sync>;
pub type TopologyRef = Arc<dyn TopologyAdapterInterface + Send + Sync>;
pub type SchedulerRef = Arc<dyn SchedulerAdapterInterface + Send + Sync>;

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
        println!("[SCHEDULER]: preparing computation...");

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

        println!("[SCHEDULER]: computing now...");

        let schedule = cnc.scheduler.compute_schedule(&topology, &domains);

        println!("[SCHEDULER]: computation successfull");

        cnc.northbound.compute_streams_completed(Vec::new());

        println!("[SCHEDULER]: configuring now...");

        cnc.southbound.configure_network(&topology, &schedule);

        println!("[SCHEDULER]: configuring successfull");

        cnc.storage.set_streams_configured(&domains);
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
                    let s = notification_types::Stream {
                        stream_id: stream.stream_id.clone(),
                        failure_code: 0,
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

impl NorthboundControllerInterface for Cnc {
    fn compute_streams(
        &self,
        computation: ComputationType,
    ) -> types::uni_types::stream_request::Output {
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
        _cuc_id: &String,
        request: Vec<(
            types::tsn_types::GroupTalker,
            Vec<types::tsn_types::GroupListener>,
        )>,
    ) {
        // TODO parse request and add to storage

        let s: Stream = Stream {
            stream_id: String::from("00-00-00-00-00-00:00-01"),
            stream_status: types::uni_types::StreamStatus::Planned,
            talker: types::uni_types::Talker {
                group_talker: request[0].0.clone(),
                group_status_talker_listener: types::tsn_types::GroupStatusTalkerListener {
                    accumulated_latency: 0,
                    interface_configuration: types::tsn_types::GroupInterfaceConfiguration {},
                },
            },
            listener: vec![types::uni_types::Listener {
                index: 0,
                group_listener: request[0].1[0].clone(),
                group_status_talker_listener: types::tsn_types::GroupStatusTalkerListener {
                    accumulated_latency: 0,
                    interface_configuration: types::tsn_types::GroupInterfaceConfiguration {},
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

        self.storage.set_stream(s);
    }
}

impl TopologyControllerInterface for Cnc {
    fn notify_topology_changed(&self) {
        // println!("[CNC] TODO: got notified about TopologyChange. But doing nothing about it...");
    }
}
