use super::tsn_types;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Domain {
    pub domain_id: String,
    pub cnc_enabled: bool,
    pub cuc: Vec<Cuc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cuc {
    pub cuc_id: String,
    pub stream: Vec<Stream>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum StreamStatus {
    Planned = 0,
    Configured = 1,
    Modified = 2,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stream {
    pub stream_id: String,
    pub stream_status: StreamStatus,
    pub talker: Talker,
    pub listener: Vec<Listener>,
    pub group_status_stream: tsn_types::GroupStatusStream,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Talker {
    // evtl nicht als struct und fields...
    pub group_talker: tsn_types::GroupTalker,
    pub group_status_talker_listener: tsn_types::GroupStatusTalkerListener,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Listener {
    pub index: u32,
    pub group_listener: tsn_types::GroupListener,
    pub group_status_talker_listener: tsn_types::GroupStatusTalkerListener,
}

pub mod stream_request {
    pub type Input = Vec<Domain>;

    pub struct Domain {
        pub domain_id: String,
        pub cuc: Vec<CucElement>,
    }

    pub struct CucElement {
        pub cuc_id: String,
        pub stream_list: Option<Vec<crate::cnc::types::tsn_types::StreamIdTypeUpper>>,
    }

    pub type Output = String;
}

pub mod request_domain_id {
    pub type Input = String;
    pub type Output = String;
}

pub mod request_free_stream_id {
    pub struct Input {
        pub domain_id: String,
        pub cuc_id: String,
    }

    pub type Output = String;
}

pub mod remove_streams {
    use crate::cnc::types::tsn_types::StreamIdTypeUpper;
    pub type Input = Vec<StreamIdTypeUpper>;
    pub type Output = String;
}
