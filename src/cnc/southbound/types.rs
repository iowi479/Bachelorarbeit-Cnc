use netconf_client::netconf_client::NetconfClient;
use std::sync::Arc;

/// this is used to specify the yang-models that have to be loaded later.
#[derive(Debug, Clone, Copy)]
pub struct YangModule {
    pub name: &'static str,
    pub revision: Option<&'static str>,
    pub features: &'static [&'static str],
}

impl YangModule {
    /// this returnes a YangModule with only the name specified.
    /// No specific revisions or features are loaded.
    pub const fn new(name: &'static str, revision: &'static str) -> Self {
        Self {
            name,
            revision: Some(revision),
            features: &[],
        }
    }

    /// this returnes a YangModule with the name and features to load specified.
    /// No specific revisions are loaded.
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

pub struct NetconfConnection {
    pub netconf_client: NetconfClient,
    pub yang_ctx: Arc<yang2::context::Context>,
}

impl Drop for NetconfConnection {
    fn drop(&mut self) {
        let _ = self.netconf_client.close_session();
    }
}
