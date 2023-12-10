use super::tsn_types::StreamIdTypeUpper;

pub type NotificationContent = Vec<Domain>;

#[derive(Debug)]
pub struct Domain {
    pub domain_id: String,
    pub cucs: Vec<Cuc>,
}

#[derive(Debug)]
pub struct Cuc {
    pub cuc_id: String,
    pub streams: Vec<Stream>,
}

#[derive(Debug)]
pub struct Stream {
    pub stream_id: StreamIdTypeUpper,

    /// # Values
    /// successful (0) or unsuccessful (1)
    pub failure_code: u8,
}
