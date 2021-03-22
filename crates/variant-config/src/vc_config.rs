#[derive(Debug, Clone)]
pub struct VariantConfigStoreConfig {
    pub condition_path: String,
    pub value_path: String,
}

impl Default for VariantConfigStoreConfig {
    fn default() -> Self {
        Self {
            condition_path: "if".to_owned(),
            value_path: "value".to_owned(),
        }
    }
}
