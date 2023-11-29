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
use std::cell::RefCell;
use std::sync::{Arc, Weak};

// Define which Components should be loaded
pub type NorthboundComponent = northbound::MockUniAdapter;
pub type SouthboundComponent = southbound::NetconfAdapter;
pub type TopologyComponent = topology::MockTopology;
pub type StorageComponent = storage::FileStorage;
pub type ScheduleComponent = middleware::IPVSDsyncTSNScheduling;

pub struct Cnc {
    id: u32,
    domain: String,

    northbound: Arc<RefCell<NorthboundComponent>>,
    southbound: Arc<RefCell<SouthboundComponent>>,
    storage: Arc<RefCell<StorageComponent>>,
    topology: Arc<RefCell<TopologyComponent>>,
    scheduler: Arc<RefCell<ScheduleComponent>>,
}

impl Cnc {
    pub fn new(
        id: u32,
        domain: String,
        mut northbound: NorthboundComponent,
        mut southbound: SouthboundComponent,
        mut storage: StorageComponent,
        mut topology: TopologyComponent,
        mut scheduler: ScheduleComponent,
    ) -> Arc<RefCell<Self>> {
        let controller: Arc<RefCell<Self>> =
            Arc::new_cyclic(|my_weak_ref: &Weak<RefCell<Self>>| {
                // configure all components
                northbound.set_cnc_ref(my_weak_ref.clone());
                southbound.set_cnc_ref(my_weak_ref.clone());
                storage.set_cnc_ref(my_weak_ref.clone());
                topology.set_cnc_ref(my_weak_ref.clone());
                scheduler.set_cnc_ref(my_weak_ref.clone());

                let configured_cnc: RefCell<Cnc> = RefCell::new(Self {
                    id,
                    domain,
                    northbound: Arc::new(RefCell::new(northbound)),
                    southbound: Arc::new(RefCell::new(southbound)),
                    storage: Arc::new(RefCell::new(storage)),
                    topology: Arc::new(RefCell::new(topology)),
                    scheduler: Arc::new(RefCell::new(scheduler)),
                });

                println!("[CNC] Successfully configured. Its now ready for use...");
                return configured_cnc;
            });

        // configuration after cnc creation due to use of the cnc reference
        controller.borrow().storage.borrow_mut().configure_storage();

        // TODO for testing
        controller.borrow().northbound.borrow().test();

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
            self.storage.borrow_mut().remove_stream(stream_id.clone());
        }
        // TODO what gets returned
        String::from("success")
    }

    fn request_domain_id(
        &self,
        input: tsntypes::uni_types::request_domain_id::Input,
    ) -> tsntypes::uni_types::request_domain_id::Output {
        return match self.storage.borrow().get_domain_id_of_cuc(input) {
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
            .borrow()
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

        self.storage.borrow_mut().set_stream(s);
    }
}

impl StorageControllerInterface for Cnc {
    fn get_cnc_domain_id(&self) -> String {
        self.domain.clone()
    }
}
