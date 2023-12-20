use crate::cnc::types::tsn_types::{GroupInterfaceId, StreamIdTypeUpper};
use std::collections::HashSet;

/// this is used to specify the yang-models that have to be loaded later.
pub struct YangModule {
    pub name: &'static str,
    pub revision: Option<&'static str>,
    pub features: &'static [&'static str],
}

impl YangModule {
    /// this returnes a YangModule with only the name specified.
    /// No specific revisions or features are loaded.
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            revision: None,
            features: &[],
        }
    }

    /// this returnes a YangModule with the name and features to load specified.
    /// No specific revisions are loaded.
    pub const fn new_with_features(name: &'static str, features: &'static [&'static str]) -> Self {
        Self {
            name,
            revision: None,
            features,
        }
    }
}

/// This struct provides information about failed configurations
pub struct FailedInterfaces {
    pub interfaces: Vec<FailedInterface>,
}

/// This struct is provided for each interface that failed configuration.
/// This Information is essential for the CNC to further configure streams.
pub struct FailedInterface {
    pub interface: GroupInterfaceId,
    pub node_id: u32,
    pub affected_streams: HashSet<StreamIdTypeUpper>,
}
