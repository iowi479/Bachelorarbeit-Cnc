use super::tsn_types::StreamIdTypeUpper;

pub type NotificationContent = Vec<Domain>;

#[derive(Debug)]
pub struct Domain {
    /// A unique identifier of a Configuration Domain. It is used to
    /// identify the Configuration Domain a CUC belongs to.
    pub domain_id: String,

    /// List of CUCs.
    ///
    /// This list exists so Streams can be associated with the CUC that
    /// initially requested them.
    pub cucs: Vec<Cuc>,
}

#[derive(Debug)]
pub struct Cuc {
    /// A unique identifier of a CNC. It is used to identify the CUC
    ///that a Streams belong to, i.e., that requested the creation
    /// of a Stream.
    pub cuc_id: String,

    /// List of Streams.
    ///
    /// Each Stream consists of a Stream ID, a request container, and
    /// a configuration container.
    ///
    /// In the fully centralized model of TSN configuration, the
    /// Stream ID and request originate from the CUC and is delivered
    /// to the CNC, while the configuration originates from the CNC
    /// and is delivered to the CUC.
    pub streams: Vec<Stream>,
}

#[derive(Debug)]
pub struct Stream {
    /// The Stream ID is a unique identifier of a Stream request
    /// and corresponding configuration. It is used to associate a
    /// CUC’s Stream request with a CNC’s corresponding response.
    pub stream_id: StreamIdTypeUpper,

    /// A code that indicates successful (0) or unsuccessful (1).
    pub failure_code: u8,
}
