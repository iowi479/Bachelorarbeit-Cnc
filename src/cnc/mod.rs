pub mod middleware;
pub mod northbound;
pub mod southbound;
pub mod storage;
pub mod topology;
pub mod tsntypes;

use self::middleware::SchedulerAdapterInterface;
use self::northbound::{NorthboundAdapterInterface, NorthboundControllerInterface};
use self::southbound::SouthboundAdapterInterface;
use self::storage::StorageAdapterInterface;
use self::topology::{TopologyAdapterInterface, TopologyControllerInterface};
use self::tsntypes::uni_types::{Domain, Stream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock, Weak};
use std::thread;
use std::time::Duration;

pub struct Cnc {
    id: u32,
    domain: String,
    schedule_sender: Sender<ComputationType>,

    northbound: Arc<RwLock<Box<dyn NorthboundAdapterInterface + Send + Sync>>>,
    southbound: Arc<RwLock<Box<dyn SouthboundAdapterInterface + Send + Sync>>>,
    storage: Arc<RwLock<Box<dyn StorageAdapterInterface + Send + Sync>>>,
    topology: Arc<RwLock<Box<dyn TopologyAdapterInterface + Send + Sync>>>,
    scheduler: Arc<RwLock<Box<dyn SchedulerAdapterInterface + Send + Sync>>>,
}

enum ComputationType {
    All(tsntypes::uni_types::compute_all_streams::Input),
    PlannedAndModified(tsntypes::uni_types::compute_planned_and_modified_streams::Input),
    List(tsntypes::uni_types::compute_streams::Input),
}

impl Cnc {
    pub fn run(
        id: u32,
        domain: String,
        mut northbound: Box<dyn NorthboundAdapterInterface + Send + Sync>,
        mut southbound: Box<dyn SouthboundAdapterInterface + Send + Sync>,
        mut storage: Box<dyn StorageAdapterInterface + Send + Sync>,
        mut topology: Box<dyn TopologyAdapterInterface + Send + Sync>,
        mut scheduler: Box<dyn SchedulerAdapterInterface + Send + Sync>,
    ) {
        // Channel for starting a computation
        // TODO create Enum for the operations
        let (start_schedule, rx): (Sender<ComputationType>, Receiver<ComputationType>) =
            mpsc::channel();

        let cnc_ref: Arc<RwLock<Self>> = Arc::new_cyclic(|my_weak_ref: &Weak<RwLock<Self>>| {
            // configure all components
            northbound.set_cnc_ref(my_weak_ref.clone());
            southbound.set_cnc_ref(my_weak_ref.clone());
            storage.set_cnc_ref(my_weak_ref.clone());
            topology.set_cnc_ref(my_weak_ref.clone());
            scheduler.set_cnc_ref(my_weak_ref.clone());

            RwLock::new(Self {
                id,
                domain,
                schedule_sender: start_schedule,
                northbound: Arc::new(RwLock::new(northbound)),
                southbound: Arc::new(RwLock::new(southbound)),
                storage: Arc::new(RwLock::new(storage)),
                topology: Arc::new(RwLock::new(topology)),
                scheduler: Arc::new(RwLock::new(scheduler)),
            })
        });
        println!("[CNC] Successfully configured. Its now ready for use...");

        // configuration of all Components
        let cnc = cnc_ref.read().unwrap();
        cnc.northbound.read().unwrap().run();
        cnc.topology.read().unwrap().run();
        cnc.storage.write().unwrap().configure_storage();

        // freeing aquired readlock
        drop(cnc);

        // wait for computation-requests
        for computation_type in rx {
            Cnc::execute_computation(cnc_ref.clone(), computation_type);
        }

        println!("[CNC] stopped...");
    }

    fn execute_computation(cnc: Arc<RwLock<Cnc>>, computation_type: ComputationType) {
        // for algorithm sorted by domain
        let _domains: Vec<Domain> = Vec::new();
        match computation_type {
            ComputationType::All(_domains) => {
                // TODO start calculation
            }
            ComputationType::PlannedAndModified(_domains) => {
                // TODO start calculation
            }
            ComputationType::List(_domains) => {
                // TODO start calculation
            }
        }

        println!("[SCHEDULER]: computing...");

        let scheduler_ref = cnc.read().unwrap().scheduler.clone();
        // TODO create flow
        // TODO This blocks... any other way?
        let s = scheduler_ref.read().unwrap().compute_schedule(
            cnc.read()
                .unwrap()
                .topology
                .as_ref()
                .read()
                .unwrap()
                .get_topology(),
            Vec::new(),
        );

        thread::sleep(Duration::from_secs(4));
        println!("[SCHEDULER]: computation successfull {s:?}");
        let nb_adapter_ref = cnc.read().unwrap().northbound.clone();
        nb_adapter_ref
            .read()
            .unwrap()
            .compute_streams_completed(Vec::new());

        println!("[SCHEDULER]: configuring now...");
        thread::sleep(Duration::from_secs(4));
        nb_adapter_ref
            .read()
            .unwrap()
            .configure_streams_completed(Vec::new());
    }
}

impl NorthboundControllerInterface for Cnc {
    fn compute_all_streams(
        &self,
        input: tsntypes::uni_types::compute_all_streams::Input,
    ) -> tsntypes::uni_types::compute_all_streams::Output {
        match self.schedule_sender.send(ComputationType::All(input)) {
            Ok(_) => String::from("Success"),
            Err(e) => e.to_string(),
        }
    }

    fn compute_planned_and_modified_streams(
        &self,
        input: tsntypes::uni_types::compute_planned_and_modified_streams::Input,
    ) -> tsntypes::uni_types::compute_planned_and_modified_streams::Output {
        match self
            .schedule_sender
            .send(ComputationType::PlannedAndModified(input))
        {
            Ok(_) => String::from("Success"),
            Err(e) => e.to_string(),
        }
    }

    fn compute_streams(
        &self,
        input: tsntypes::uni_types::compute_streams::Input,
    ) -> tsntypes::uni_types::compute_streams::Output {
        match self.schedule_sender.send(ComputationType::List(input)) {
            Ok(_) => String::from("Success"),
            Err(e) => e.to_string(),
        }
    }

    fn remove_streams(
        &self,
        input: tsntypes::uni_types::remove_streams::Input,
    ) -> tsntypes::uni_types::remove_streams::Output {
        for stream_id in input.iter() {
            self.storage
                .write()
                .unwrap()
                .remove_stream(stream_id.clone());
        }
        // TODO what gets returned?? -> Success?
        String::from("Success")
    }

    fn request_domain_id(
        &self,
        input: tsntypes::uni_types::request_domain_id::Input,
    ) -> tsntypes::uni_types::request_domain_id::Output {
        return match self.storage.read().unwrap().get_domain_id_of_cuc(input) {
            None => String::from("Failure"),
            Some(domain_id) => domain_id,
        };
    }

    fn request_free_stream_id(
        &self,
        input: tsntypes::uni_types::request_free_stream_id::Input,
    ) -> tsntypes::uni_types::request_free_stream_id::Output {
        match self
            .storage
            .read()
            .unwrap()
            .get_free_stream_id(input.domain_id, input.cuc_id)
        {
            Some(id) => id,
            None => String::from("no id"),
        }
    }

    // TODO tuple als type definieren
    fn set_streams(
        &self,
        request: Vec<(
            tsntypes::tsn_types::GroupTalker,
            Vec<tsntypes::tsn_types::GroupListener>,
        )>,
    ) {
        // TODO parse request and add to storage

        let s: Stream = Stream {
            stream_id: String::from("00-00-00-00-00-00:00-01"),
            stream_status: tsntypes::uni_types::StreamStatus::Planned,
            talker: tsntypes::uni_types::Talker {
                group_talker: request[0].0.clone(),
                group_status_talker_listener: tsntypes::tsn_types::GroupStatusTalkerListener {
                    accumulated_latency: 0,
                    interface_configuration: tsntypes::tsn_types::GroupInterfaceConfiguration {},
                },
            },
            listener: vec![tsntypes::uni_types::Listener {
                index: 0,
                group_listener: request[0].1[0].clone(),
                group_status_talker_listener: tsntypes::tsn_types::GroupStatusTalkerListener {
                    accumulated_latency: 0,
                    interface_configuration: tsntypes::tsn_types::GroupInterfaceConfiguration {},
                },
            }],
            group_status_stream: tsntypes::tsn_types::GroupStatusStream {
                status_info: tsntypes::tsn_types::StatusInfoContainer {
                    talker_status: tsntypes::tsn_types::TalkerStatus::None,
                    listener_status: tsntypes::tsn_types::ListenerStatus::None,
                    failure_code: 0,
                },
                failed_interfaces: Vec::new(),
            },
        };

        self.storage.write().unwrap().set_stream(s);
    }
}

impl TopologyControllerInterface for Cnc {
    fn notify_topology_changed(&self) {
        // TODO behaviour of CNC on topologychange
        println!("[CNC] TODO: got notified about TopologyChange. But doing nothing about it...");
    }
}
