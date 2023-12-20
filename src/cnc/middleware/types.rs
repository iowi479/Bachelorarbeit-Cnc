use crate::cnc::types::{scheduling::Schedule, tsn_types::StreamIdTypeUpper, uni_types::Domain};

pub struct ComputationResult {
    pub schedule: Schedule,
    pub domains: Vec<Domain>,
    pub failed_streams: Vec<FailedStream>,
}

pub struct FailedStream {
    pub stream_id: StreamIdTypeUpper,
    pub cuc_id: String,
    pub domain_id: String,
    // TODO acutally here?
    pub failure_code: u32,
}
