use netconf_client::netconf_client::NetconfClient;
use std::{collections::HashMap, sync::Arc};

/// this is used to specify the yang-models that have to be loaded later.
#[derive(Debug, Clone, Copy)]
pub struct YangModule {
    pub name: &'static str,
    pub revision: Option<&'static str>,
    pub features: &'static [&'static str],
}

impl YangModule {
    /// this returnes a YangModule with the name and revision specified.
    pub const fn new(name: &'static str, revision: &'static str) -> Self {
        Self {
            name,
            revision: Some(revision),
            features: &[],
        }
    }

    /// this returnes a YangModule with all info specified.
    pub const fn new_with_features(
        name: &'static str,
        revision: &'static str,
        features: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            revision: Some(revision),
            features,
        }
    }
}

// is used to hold a established netconf_connection as well as the yang_context for parsing the
// exchanged data.
//
// the xpath_dict is used as a lookuptable to find the specified field.
pub struct NetconfConnection {
    pub netconf_client: NetconfClient,
    pub yang_ctx: Arc<yang2::context::Context>,
    pub xpath_dict: HashMap<String, String>,
}
