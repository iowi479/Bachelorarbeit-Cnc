use std::sync::{RwLock, Weak};
use std::thread;
use std::time::Duration;

use super::tsntypes::notification_types::NotificationContent;
use super::tsntypes::uni_types::{
    compute_all_streams, compute_planned_and_modified_streams, compute_streams, remove_streams,
    request_domain_id, request_free_stream_id,
};
use super::Cnc;

use super::tsntypes::tsn_types::{
    DataFrameSpecificationElement, DataFrameSpecificationElementType, EndStationInterface,
    GroupIeee802VlanTag, GroupInterfaceCapabilities, GroupInterfaceId, GroupListener, GroupTalker,
    GroupUserToNetworkRequirements, StreamRankContainer, TrafficSpecificationContainer,
};

// Communication Component <--> CNC
pub trait NorthboundAdapterInterface {
    // Notifications: notfiy CUC on completed task
    fn compute_streams_completed(&self, notification: NotificationContent);
    fn configure_streams_completed(&self, notification: NotificationContent);
    fn remove_streams_completed(&self, notification: NotificationContent);

    /// running this component continously
    ///
    /// possibly in a new Thread
    ///
    /// # Important
    /// This has to be non-blocking!
    fn run(&self);

    /// CNC Configuration
    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>);
}

// Communication Component <--> CNC
pub trait NorthboundControllerInterface {
    // TODO overthink the stuff
    // uni rpc
    fn compute_streams(&self, input: compute_streams::Input) -> compute_streams::Output;

    fn compute_planned_and_modified_streams(
        &self,
        input: compute_planned_and_modified_streams::Input,
    ) -> compute_planned_and_modified_streams::Output;

    fn compute_all_streams(&self, input: compute_all_streams::Input)
        -> compute_all_streams::Output;

    fn request_domain_id(&self, input: request_domain_id::Input) -> request_domain_id::Output;

    fn request_free_stream_id(
        &self,
        input: request_free_stream_id::Input,
    ) -> request_free_stream_id::Output;

    // action remove streams is not a rpc? is a action on the tsn-uni container (yang-tsn-config-uni row 182)
    fn remove_streams(&self, input: remove_streams::Input) -> remove_streams::Output;

    // TODO type touple... correct?
    // fn stream_request(&self, request: Vec<(GroupTalker, Vec<GroupListener>)>);
    fn set_streams(&self, request: Vec<(GroupTalker, Vec<GroupListener>)>);
}

pub struct MockUniAdapter {
    cnc: Option<Weak<RwLock<Cnc>>>, // ref to cnc
}

// Implementation specific stuff
impl MockUniAdapter {
    pub fn new() -> Self {
        Self { cnc: None }
    }

    pub fn get_example_add_stream() -> Vec<(GroupTalker, Vec<GroupListener>)> {
        let mut result: Vec<(GroupTalker, Vec<GroupListener>)> = Vec::new();

        // 1
        let talker: GroupTalker = GroupTalker {
            stream_rank: StreamRankContainer { rank: 1 },
            end_station_interfaces: vec![EndStationInterface {
                index: 0,
                interface_id: GroupInterfaceId {
                    interface_name: "".to_string(),
                    mac_address: "00-00-00-00-00-01".to_string(),
                },
            }],
            data_frame_specification: vec![
                DataFrameSpecificationElement {
                    index: 0,
                    field: DataFrameSpecificationElementType::Ieee802MacAddresses(
                        super::tsntypes::tsn_types::GroupIeee802MacAddress {
                            destination_mac_adress: "00-00-00-0F-00-00".to_string(),
                            source_mac_adress: "00-00-00-00-00-01".to_string(),
                        },
                    ),
                },
                DataFrameSpecificationElement {
                    index: 1,
                    field: DataFrameSpecificationElementType::Ieee802VlanTag(GroupIeee802VlanTag {
                        priority_code_point: 6,
                        vlan_id: 0,
                    }),
                },
            ],
            traffic_specification: TrafficSpecificationContainer {
                interval: super::tsntypes::tsn_types::TrafficSpecificationInterval {
                    numerator: 1000000,
                    denominator: 1000000000,
                },
                max_frames_per_interval: 1,
                max_frame_size: 1,
                transmission_selection: 0,
                time_aware: super::tsntypes::tsn_types::TimeAwareContainer {
                    earliest_transmit_offset: 100,
                    latest_transmit_offset: 500000,
                    jitter: 0,
                },
            },
            user_to_network_requirements:
                super::tsntypes::tsn_types::GroupUserToNetworkRequirements {
                    num_seemless_trees: 1,
                    max_latency: 100000,
                },
            interface_capabilities: super::tsntypes::tsn_types::GroupInterfaceCapabilities {
                vlan_tag_capable: true,
                // default to empty list - IEEE 8021Q 46.2.3.7.2
                cb_stream_iden_type_list: Vec::new(),
                cb_sequence_type_list: Vec::new(),
            },
        };

        let listener: Vec<GroupListener> = vec![GroupListener {
            index: 0, // TODO stream_id??? and index???
            end_station_interfaces: vec![EndStationInterface {
                index: 0,
                interface_id: GroupInterfaceId {
                    mac_address: "00-00-00-0F-00-00".to_string(),
                    interface_name: "".to_string(),
                },
            }],
            user_to_network_requirements: GroupUserToNetworkRequirements {
                num_seemless_trees: 1,
                max_latency: 100000,
            },
            interface_capabilities: GroupInterfaceCapabilities {
                vlan_tag_capable: true,
                // default to empty list - IEEE 8021Q 46.2.3.7.2
                cb_sequence_type_list: Vec::new(),
                cb_stream_iden_type_list: Vec::new(),
            },
        }];

        // TODO configure stream element... What gets send from cuc, what is default and what is configured by CNC/Algo...
        // let stream: Stream = Stream { stream_id: "00-01".to_string(), stream_status: StreamStatus::Planned, talker: Talker{group_talker: talker,group_status_talker_listener: GroupStatusTalkerListener{}}, listener, group_status_stream: super::tsntypes::tsn_types::GroupStatusStream { status_info: super::tsntypes::tsn_types::StatusInfoContainer { talker_status: (), listener_status: (), failure_code: () }, failed_interfaces: () } }

        result.push((talker, listener));

        return result;
    }
}

impl NorthboundAdapterInterface for MockUniAdapter {
    fn compute_streams_completed(&self, notification: NotificationContent) {
        println!("[Northbound]-[MockUniAdapter] Notification: compute_stream_completed \n {notification:?}");
    }
    fn configure_streams_completed(&self, notification: NotificationContent) {
        println!("[Northbound]-[MockUniAdapter] Notification: configure_streams_completed \n {notification:?}");
    }
    fn remove_streams_completed(&self, notification: NotificationContent) {
        println!("[Northbound]-[MockUniAdapter] Notification: remove_streams_completed \n {notification:?}");
    }

    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>) {
        self.cnc = Some(cnc);
    }

    /// currently this does:
    /// - addStream ...00:00-01
    /// - compute call
    /// - remove stream ...00:00-01
    fn run(&self) {
        // ref to cnc for moving to second thread
        let cnc = self.cnc.as_ref().unwrap().clone().upgrade().unwrap();

        // TODO implement what this component does
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(2));
            // set stream-data
            cnc.read()
                .unwrap()
                .set_streams(MockUniAdapter::get_example_add_stream());

            thread::sleep(Duration::from_secs(5));
            // start a scheduling run
            // TODO fill vector
            cnc.read().unwrap().compute_all_streams(Vec::new());

            thread::sleep(Duration::from_secs(5));
            let res = cnc
                .read()
                .unwrap()
                .remove_streams(vec![String::from("00-00-00-00-00-00:00-01")]);
            println!("[Northbound] response to remove_streams {res}");

            thread::sleep(Duration::from_secs(5));
        });
    }
}
