use super::tsn_types;
use serde::{Deserialize, Serialize};

/// Top-level container for the TSN UNI module.
#[derive(Serialize, Deserialize, Clone)]
pub struct TsnUni {
    /// List of Configuration Domains.
    ///
    /// This list exists so CUCs can be associated with the Configuration
    /// Domain they are located in and can be used to restrict access to
    /// CUCs, e.g., by using standard mechanism as described in RFC 8341.
    domain: Vec<Domain>,
}

/// List of Configuration Domains.
///
/// This list exists so CUCs can be associated with the Configuration
/// Domain they are located in and can be used to restrict access to
/// CUCs, e.g., by using standard mechanism as described in RFC 8341.
#[derive(Serialize, Deserialize, Clone)]
pub struct Domain {
    /// The Domain ID is a unique identifier of a Configuration
    /// Domain. It is used to identify the Configuration Domain a CUC
    /// belongs to.
    pub domain_id: String,

    /// cnc-enabled is used to enable or disable the CNC functionality
    /// of a station capable of acting as a CNC. If this object is set
    /// to TRUE the CNC functionality is enabled. If it is set to FALSE
    /// the CNC functionality is disabled.
    pub cnc_enabled: bool,

    /// List of CUCs.
    ///
    /// This list exists so Streams can be associated with the CUC that
    /// initially requested them and can be used to restrict access to
    /// Streams, e.g., by using standard mechanisms as described in RFC
    /// 8341.
    pub cuc: Vec<Cuc>,
}

/// List of CUCs.
///
/// This list exists so Streams can be associated with the CUC that
/// initially requested them and can be used to restrict access to
/// Streams, e.g., by using standard mechanisms as described in RFC
/// 8341.
#[derive(Serialize, Deserialize, Clone)]
pub struct Cuc {
    /// The CUC ID is a unique identifier of a CUC. It is used to
    /// identify the CUC that a Stream belongs to, i.e., that
    /// requested the creation of a Stream.
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
    pub stream: Vec<Stream>,
}

/// The stream-status indicates what status the Stream has in
/// the CNC.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum StreamStatus {
    /// The Stream has been requested but has not yet been
    /// configured by the CNC.
    Planned = 0,

    /// The Stream has been computed and configured by the
    /// CNC.
    Configured = 1,

    /// The Stream has been configured but Stream parameters
    /// have been modified after configuration.
    Modified = 2,
}

/// List of Streams.
///
/// Each Stream consists of a Stream ID, a request container, and
/// a configuration container.
///
/// In the fully centralized model of TSN configuration, the
/// Stream ID and request originate from the CUC and is delivered
/// to the CNC, while the configuration originates from the CNC
/// and is delivered to the CUC.
#[derive(Serialize, Deserialize, Clone)]
pub struct Stream {
    /// The Stream ID is a unique identifier of a Stream request
    /// and corresponding configuration. It is used to associate a
    /// CUC’s Stream request with a CNC’s corresponding response.
    pub stream_id: String,

    /// The stream-status indicates what status the Stream has in
    /// the CNC.
    pub stream_status: StreamStatus,

    /// The Talker container contains: - Talker’s behavior for
    /// Stream (how/when transmitted) - Talker’s requirements from
    /// the network - TSN capabilities of the Talker’s
    /// interface(s).
    pub talker: Talker,

    /// Each Listener list entry contains: - Listener’s
    /// requirements from the network - TSN capabilities of the
    /// Listener’s interface(s).
    pub listener: Vec<Listener>,
    pub group_status_stream: tsn_types::GroupStatusStream,
}

/// The Talker container contains: - Talker’s behavior for
/// Stream (how/when transmitted) - Talker’s requirements from
/// the network - TSN capabilities of the Talker’s
/// interface(s).
#[derive(Serialize, Deserialize, Clone)]
pub struct Talker {
    pub group_talker: tsn_types::GroupTalker,
    pub group_status_talker_listener: tsn_types::GroupStatusTalkerListener,
}

/// Each Listener list entry contains: - Listener’s
/// requirements from the network - TSN capabilities of the
/// Listener’s interface(s).
#[derive(Serialize, Deserialize, Clone)]
pub struct Listener {
    /// This index is provided in order to provide a unique key
    /// per list entry.
    pub index: u32,
    pub group_listener: tsn_types::GroupListener,
    pub group_status_talker_listener: tsn_types::GroupStatusTalkerListener,
}

/// These types are for the rpc-calls compute_streams, compute_planned_and_modified_streams and compute_all_streams from the parent yang model.
///
/// # compute_streams
/// Starts computation of path and resource allocation for one or more
/// Stream. The Streams that are included in the computation are the
/// ones that have their domain-id, cuc-id, and stream-id provided.
/// This RPC can be applied to compute new Streams as well as recompute
/// Streams that have been modified.
///
/// # compute_planned_and_modified_streams
///
/// Starts computation of path and resource allocation for all Streams
/// that are in the domain provided by domain-id and are associated
/// with the CUC provided by cuc-id, and that have not been computed
/// (i.e., that have a Stream status of planned or modified.
///
/// # compute_all_streams
///
/// Starts computation of path and resource allocation for all Streams
/// that are in the domain provided by domain-id and are associated
/// with the CUC provided by cuc-id.
pub mod compute_streams {
    /// List of Configuration Domains.
    ///
    /// This list exists so CUCs can be associated with the
    /// Configuration Domain they are located in.
    pub type Input = Vec<Domain>;

    pub struct Domain {
        /// A unique identifier of a Configuration Domain. It is used to
        /// identify the Configuration Domain a CUC belongs to.
        pub domain_id: String,

        /// List of CUCs.
        ///
        /// This list exists so Streams can be associated with the CUC
        /// that initially requested them.
        pub cuc: Vec<CucElement>,
    }

    pub struct CucElement {
        /// A unique identifier of a CNC. It is used to identify the
        /// CUC that a Streams belong to, i.e., that requested the
        /// creation of a Stream.
        pub cuc_id: String,

        /// List of stream-ids that are used to identify the Streams
        /// that are requested to be computed and configured.
        ///
        /// In case of compute_all_streams a None is used
        pub stream_list: Option<Vec<crate::cnc::types::tsn_types::StreamIdTypeUpper>>,
    }

    /// Only returns status information indicating if the computation
    /// has been started. It does not return status information on the
    /// success or failure of the actual Stream computation. A
    /// notifcation can be used to inform the caller of this RPC on the
    /// results of Stream computation after the computation has
    /// finished.
    pub type Output = String;
}

/// These types are for the rpc-call request_domain_id from the parent yang model.
///
/// Returns the DomainId of the Configuration Domain that the
/// requesting CUC belongs to.
pub mod request_domain_id {
    /// A unique identifier of a CNC. It is used to identify the CUC,
    /// allowing the CNC to return the DomainId this CUC belongs to.
    pub type Input = String;

    ///Returns the DomainId of the Configuration Domain that the
    /// requesting CUC belongs to.
    pub type Output = String;
}

/// These types are for the rpc-call request_free_stream_id from the parent yang model.
///
/// Returns a free StreamId available for the Configuration Domain
/// identified by the DomainId.
///
/// In case of our fully centralized model, this should be done by the CUC because it knows who the talker will be and their MAC-Adress.
pub mod request_free_stream_id {
    pub struct Input {
        /// A unique identifier of a Configuration Domain. It is used to
        /// identify the Configuration Domain a CUC belongs to.
        pub domain_id: String,

        /// A unique identifier of a CNC. It is used to identify the CUC,
        /// allowing the CNC to return the DomainId this CUC belongs to.
        pub cuc_id: String,
    }

    /// Returns a free StreamId available for the Configuration Domain
    /// identified by the DomainId.
    pub type Output = String;
}

/// These types are for the action remove-streams specified in the parent yang model.
///
/// Removes the Streams with the ids provided in the stream-id
/// list.
pub mod remove_streams {
    /// List of stream-ids that are used to identify the Streams
    /// that are requested to be removed.
    pub type Input = Vec<crate::cnc::types::tsn_types::StreamIdTypeUpper>;

    /// Returns status information indicating if Stream removal
    /// has been successfully started.
    pub type Output = String;
}
