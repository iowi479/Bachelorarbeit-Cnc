use super::types::computation::ComputationType;
use super::types::notification_types::NotificationContent;
use super::types::tsn_types::{
    DataFrameSpecificationElement, DataFrameSpecificationElementType, GroupIeee802VlanTag,
    GroupInterfaceCapabilities, GroupInterfaceId, GroupListener, GroupTalker,
    GroupUserToNetworkRequirements, StreamRankContainer, TrafficSpecificationContainer,
};
use super::types::uni_types::{
    compute_streams, remove_streams, request_domain_id, request_free_stream_id,
};
use super::{Cnc, CNC_NOT_PRESENT};
use std::sync::Weak;
use std::time::Duration;
use std::{thread, vec};

/// # Northbound Interface
/// This Trait has to be implemented to use the Component as a Northbound-Interface in the CNC.
///
/// This Component is used to communicate with the CUC in a "fully centralized model" of a TSN Network.
///
/// This model and the communication (UNI) is specified in the corresponding IEEE Standards 802.1Q and Qdj
pub trait NorthboundAdapterInterface {
    /// Notification to the CUC.
    ///
    /// This gets called when the requested computation of streams is finished.
    ///
    /// The NotificationBody including the StreamStatus has to be forwarded to the corresponding CUC instance.
    fn compute_streams_completed(&self, notification: NotificationContent);

    /// Notification to the CUC.
    ///
    /// This gets called when the computated streams are configured on all Bridges.
    ///
    /// The NotificationBody including the StreamStatus has to be forwarded to the corresponding CUC instance.
    fn configure_streams_completed(&self, notification: NotificationContent);

    /// Notification to the CUC.
    ///
    /// This gets called when the requested deletion of streams is finished.
    ///
    /// The NotificationBody including the StreamStatus has to be forwarded to the corresponding CUC instance.
    fn remove_streams_completed(&self, notification: NotificationContent);

    /// Running this component continously
    ///
    ///  -> possibly in a new Thread
    ///
    /// While running, all calls of the connected CUC's has to be forwarded to the CNC
    ///
    /// # Important
    /// This has to be non-blocking!
    fn run(&self);

    /// CNC Configuration
    ///
    /// additional Setup can be performed here
    ///
    /// # Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

/// This Trait is implemented by the CNC and provides endpoints for the Northbound-Component to trigger actions in the CNC
pub trait NorthboundControllerInterface {
    fn compute_streams(&self, computation: ComputationType) -> compute_streams::Output;

    fn request_domain_id(&self, input: request_domain_id::Input) -> request_domain_id::Output;

    fn request_free_stream_id(
        &self,
        input: request_free_stream_id::Input,
    ) -> request_free_stream_id::Output;

    // action remove streams is not a rpc? is a action on the tsn-uni container (yang-tsn-config-uni row 182)
    fn remove_streams(
        &self,
        cuc_id: &String,
        input: remove_streams::Input,
    ) -> remove_streams::Output;

    // TODO type touple... correct?
    // fn stream_request(&self, request: Vec<(GroupTalker, Vec<GroupListener>)>);
    fn set_streams(&self, cuc_id: &String, request: Vec<(GroupTalker, Vec<GroupListener>)>);
}

pub struct MockUniAdapter {
    cnc: Weak<Cnc>,
    cuc_id: String,
}

// Implementation specific stuff
impl MockUniAdapter {
    pub fn new(cuc_id: String) -> Self {
        Self {
            cnc: Weak::default(),
            cuc_id,
        }
    }

    pub fn get_example_add_stream() -> Vec<(GroupTalker, Vec<GroupListener>)> {
        let mut result: Vec<(GroupTalker, Vec<GroupListener>)> = Vec::new();

        // 1
        let talker: GroupTalker = GroupTalker {
            stream_rank: StreamRankContainer { rank: 1 },
            end_station_interfaces: vec![GroupInterfaceId {
                interface_name: "".to_string(),
                mac_address: "00-00-00-00-00-01".to_string(),
            }],
            data_frame_specification: vec![
                DataFrameSpecificationElement {
                    index: 0,
                    field: DataFrameSpecificationElementType::Ieee802MacAddresses(
                        super::types::tsn_types::GroupIeee802MacAddress {
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
                interval: super::types::tsn_types::TrafficSpecificationInterval {
                    numerator: 1000000,
                    denominator: 1000000000,
                },
                max_frames_per_interval: 1,
                max_frame_size: 1,
                transmission_selection: 0,
                time_aware: super::types::tsn_types::TimeAwareContainer {
                    earliest_transmit_offset: 100,
                    latest_transmit_offset: 500000,
                    jitter: 0,
                },
            },
            user_to_network_requirements: super::types::tsn_types::GroupUserToNetworkRequirements {
                num_seemless_trees: 1,
                max_latency: 100000,
            },
            interface_capabilities: super::types::tsn_types::GroupInterfaceCapabilities {
                vlan_tag_capable: true,
                // default to empty list - IEEE 8021Q 46.2.3.7.2
                cb_stream_iden_type_list: Vec::new(),
                cb_sequence_type_list: Vec::new(),
            },
        };

        let listener: Vec<GroupListener> = vec![GroupListener {
            index: 0, // TODO stream_id??? and index???
            end_station_interfaces: vec![GroupInterfaceId {
                mac_address: "00-00-00-0F-00-00".to_string(),
                interface_name: "".to_string(),
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

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }

    /// currently this does:
    /// - addStream ...00:00-01
    /// - compute call
    /// - remove stream ...00:00-01
    fn run(&self) {
        // these get moved to the new thread
        let cnc = self.cnc.upgrade().expect(CNC_NOT_PRESENT).clone();
        let cuc_id = self.cuc_id.clone();

        println!("[Northbound] running now...");
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(2));
            // set stream-data
            cnc.set_streams(&cuc_id, MockUniAdapter::get_example_add_stream());

            thread::sleep(Duration::from_secs(5));

            // start a scheduling run
            let domain: Vec<compute_streams::Domain> = vec![compute_streams::Domain {
                domain_id: cnc.domain.clone(),
                cuc: vec![compute_streams::CucElement {
                    cuc_id: cuc_id.clone(),
                    stream_list: None,
                }],
            }];
            cnc.compute_streams(ComputationType::All(domain));

            thread::sleep(Duration::from_secs(5));
            // let res = cnc.remove_streams(&cuc_id, vec![String::from("00-00-00-00-00-00:00-01")]);
            // println!("[Northbound] response to remove_streams {res}");

            thread::sleep(Duration::from_secs(5));
        });
    }
}
