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
use self::types::uni_types::{Domain, Stream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Weak};
use std::thread;
use std::time::Duration;

pub struct Cnc {
    id: u32,
    domain: String,
    schedule_sender: Sender<ComputationType>,

    northbound: Arc<dyn NorthboundAdapterInterface + Send + Sync>,
    southbound: Arc<dyn SouthboundAdapterInterface + Send + Sync>,
    storage: Arc<dyn StorageAdapterInterface + Send + Sync>,
    topology: Arc<dyn TopologyAdapterInterface + Send + Sync>,
    scheduler: Arc<dyn SchedulerAdapterInterface + Send + Sync>,
}

enum ComputationType {
    All(types::uni_types::compute_all_streams::Input),
    PlannedAndModified(types::uni_types::compute_planned_and_modified_streams::Input),
    List(types::uni_types::compute_streams::Input),
}

impl Cnc {
    pub fn run(
        id: u32,
        domain: String,
        mut northbound: Arc<dyn NorthboundAdapterInterface + Send + Sync>,
        mut southbound: Arc<dyn SouthboundAdapterInterface + Send + Sync>,
        mut storage: Arc<dyn StorageAdapterInterface + Send + Sync>,
        mut topology: Arc<dyn TopologyAdapterInterface + Send + Sync>,
        mut scheduler: Arc<dyn SchedulerAdapterInterface + Send + Sync>,
    ) {
        // Channel for starting a computation
        // TODO create Enum for the operations
        let (start_schedule, rx): (Sender<ComputationType>, Receiver<ComputationType>) =
            mpsc::channel();

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
                schedule_sender: start_schedule,
                northbound,
                southbound,
                storage,
                topology,
                scheduler,
            }
        });
        println!("[CNC] Successfully configured. Its now ready for use...");

        // configuration of all Components
        cnc.northbound.run();
        cnc.topology.run();
        cnc.storage.configure_storage();

        // wait for computation-requests
        for computation_type in rx {
            Cnc::execute_computation(cnc.clone(), computation_type);
        }

        println!("[CNC] stopped...");
        drop(cnc);
    }

    fn execute_computation(cnc: Arc<Cnc>, computation_type: ComputationType) {
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

        let scheduler_ref = cnc.scheduler.clone();
        // TODO create flow
        // TODO This blocks... any other way?
        let s = scheduler_ref.compute_schedule(cnc.topology.get_topology(), Vec::new());

        thread::sleep(Duration::from_secs(4));
        println!("[SCHEDULER]: computation successfull");
        let nb_adapter_ref = cnc.northbound.clone();
        nb_adapter_ref.compute_streams_completed(Vec::new());

        println!("[SCHEDULER]: configuring now...");
        thread::sleep(Duration::from_secs(4));
        nb_adapter_ref.configure_streams_completed(Vec::new());
    }
}

impl NorthboundControllerInterface for Cnc {
    fn compute_all_streams(
        &self,
        input: types::uni_types::compute_all_streams::Input,
    ) -> types::uni_types::compute_all_streams::Output {
        match self.schedule_sender.send(ComputationType::All(input)) {
            Ok(_) => String::from("Success"),
            Err(e) => e.to_string(),
        }
    }

    fn compute_planned_and_modified_streams(
        &self,
        input: types::uni_types::compute_planned_and_modified_streams::Input,
    ) -> types::uni_types::compute_planned_and_modified_streams::Output {
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
        input: types::uni_types::compute_streams::Input,
    ) -> types::uni_types::compute_streams::Output {
        match self.schedule_sender.send(ComputationType::List(input)) {
            Ok(_) => String::from("Success"),
            Err(e) => e.to_string(),
        }
    }

    fn remove_streams(
        &self,
        input: types::uni_types::remove_streams::Input,
    ) -> types::uni_types::remove_streams::Output {
        for stream_id in input.iter() {
            self.storage.remove_stream(stream_id.clone());
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
        // TODO behaviour of CNC on topologychange
        println!("[CNC] TODO: got notified about TopologyChange. But doing nothing about it...");
    }
}
