pub mod middleware;
pub mod northbound;
pub mod southbound;
pub mod storage;
pub mod topology;
pub mod tsntypes;

use self::northbound::NorthboundAdapterInterface;
use self::southbound::SouthboundAdapterInterface;
use self::storage::{StorageAdapterInterface, StorageControllerInterface};
use self::topology::TopologyAdapterInterface;
use self::tsntypes::uni_types::Stream;
use self::{middleware::SchedulerAdapterInterface, northbound::NorthboundControllerInterface};
use std::sync::{Arc, RwLock, Weak};

pub struct Cnc {
    id: u32,
    domain: String,

    northbound: Arc<RwLock<Box<dyn NorthboundAdapterInterface + Send + Sync>>>,
    southbound: Arc<RwLock<Box<dyn SouthboundAdapterInterface + Send + Sync>>>,
    storage: Arc<RwLock<Box<dyn StorageAdapterInterface + Send + Sync>>>,
    topology: Arc<RwLock<Box<dyn TopologyAdapterInterface + Send + Sync>>>,
    scheduler: Arc<RwLock<Box<dyn SchedulerAdapterInterface + Send + Sync>>>,
}

impl Cnc {
    pub fn new(
        id: u32,
        domain: String,
        mut northbound: Box<dyn NorthboundAdapterInterface + Send + Sync>,
        mut southbound: Box<dyn SouthboundAdapterInterface + Send + Sync>,
        mut storage: Box<dyn StorageAdapterInterface + Send + Sync>,
        mut topology: Box<dyn TopologyAdapterInterface + Send + Sync>,
        mut scheduler: Box<dyn SchedulerAdapterInterface + Send + Sync>,
    ) -> Arc<RwLock<Self>> {
        let controller: Arc<RwLock<Self>> = Arc::new_cyclic(|my_weak_ref: &Weak<RwLock<Self>>| {
            // configure all components
            northbound.set_cnc_ref(my_weak_ref.clone());
            southbound.set_cnc_ref(my_weak_ref.clone());
            storage.set_cnc_ref(my_weak_ref.clone());
            topology.set_cnc_ref(my_weak_ref.clone());
            scheduler.set_cnc_ref(my_weak_ref.clone());

            let configured_cnc: RwLock<Cnc> = RwLock::new(Self {
                id,
                domain,
                northbound: Arc::new(RwLock::new(northbound)),
                southbound: Arc::new(RwLock::new(southbound)),
                storage: Arc::new(RwLock::new(storage)),
                topology: Arc::new(RwLock::new(topology)),
                scheduler: Arc::new(RwLock::new(scheduler)),
            });

            println!("[CNC] Successfully configured. Its now ready for use...");
            return configured_cnc;
        });

        // configuration after cnc creation due to use of the cnc reference
        controller
            .read()
            .unwrap()
            .storage
            .write()
            .unwrap()
            .configure_storage();

        // TODO for testing
        // controller.read().unwrap().northbound.read().unwrap().test();

        return controller;
    }

    pub fn get_id(&self) -> u32 {
        self.id.clone()
    }
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }
}

impl NorthboundControllerInterface for Cnc {
    fn compute_all_streams(
        &self,
        input: tsntypes::uni_types::compute_all_streams::Input,
    ) -> tsntypes::uni_types::compute_all_streams::Output {
        String::from("TODO")
    }

    fn compute_planned_and_modified_streams(
        &self,
        input: tsntypes::uni_types::compute_planned_and_modified_streams::Input,
    ) -> tsntypes::uni_types::compute_planned_and_modified_streams::Output {
        String::from("TODO")
    }

    fn compute_streams(
        &self,
        input: tsntypes::uni_types::compute_streams::Input,
    ) -> tsntypes::uni_types::compute_streams::Output {
        String::from("TODO")
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
        // TODO what gets returned
        String::from("success")
    }

    fn request_domain_id(
        &self,
        input: tsntypes::uni_types::request_domain_id::Input,
    ) -> tsntypes::uni_types::request_domain_id::Output {
        return match self.storage.read().unwrap().get_domain_id_of_cuc(input) {
            None => String::new(),
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
    fn stream_request(
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

impl StorageControllerInterface for Cnc {
    fn get_cnc_domain_id(&self) -> String {
        self.domain.clone()
    }
}
