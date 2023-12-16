pub struct YangModule {
    pub name: &'static str,
    pub revision: Option<&'static str>,
    pub features: &'static [&'static str],
}

impl YangModule {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            revision: None,
            features: &[],
        }
    }

    pub const fn new_with_features(name: &'static str, features: &'static [&'static str]) -> Self {
        Self {
            name,
            revision: None,
            features,
        }
    }
}
