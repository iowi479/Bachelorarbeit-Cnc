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
