pub mod dsl;
mod resolver;
mod vc_config;

pub use dsl::VariantValue;
use hashbrown::HashMap;
pub use resolver::*;
use serde::de::DeserializeOwned;
use serde_json;
use std::sync::Arc;
pub use vc_config::*;

pub struct VariantConfigStore {
    resolver: Arc<JsonConfigResolver>,
    pub global_variants: Arc<HashMap<String, VariantValue>>,
    pub config: Arc<VariantConfigStoreConfig>,
}

impl VariantConfigStore {
    pub fn new(
        json: serde_json::Value,
        global_variants: HashMap<String, VariantValue>,
    ) -> anyhow::Result<Self> {
        Self::new_with_config(json, global_variants, VariantConfigStoreConfig::default())
    }

    pub fn new_with_config(
        json: serde_json::Value,
        global_variants: HashMap<String, VariantValue>,
        config: VariantConfigStoreConfig,
    ) -> anyhow::Result<Self> {
        let resolver = JsonConfigResolver::new_with_custom_path(
            json,
            config.condition_path.clone(),
            config.value_path.clone(),
        )?;

        Ok(Self {
            resolver: Arc::new(resolver),
            global_variants: Arc::new(global_variants),
            config: Arc::new(config),
        })
    }

    pub fn new_from_yaml(
        yaml: &str,
        global_variants: HashMap<String, VariantValue>,
    ) -> anyhow::Result<Self> {
        Self::new_from_yaml_with_config(yaml, global_variants, VariantConfigStoreConfig::default())
    }

    pub fn new_from_yaml_with_config(
        yaml: &str,
        global_variants: HashMap<String, VariantValue>,
        config: VariantConfigStoreConfig,
    ) -> anyhow::Result<Self> {
        let json = serde_yaml::from_str(yaml)?;
        Self::new_with_config(json, global_variants, config)
    }

    pub fn new_from_toml(
        toml: &str,
        global_variants: HashMap<String, VariantValue>,
    ) -> anyhow::Result<Self> {
        Self::new_from_toml_with_config(toml, global_variants, VariantConfigStoreConfig::default())
    }

    pub fn new_from_toml_with_config(
        toml: &str,
        global_variants: HashMap<String, VariantValue>,
        config: VariantConfigStoreConfig,
    ) -> anyhow::Result<Self> {
        let json = toml::from_str(toml)?;
        Self::new_with_config(json, global_variants, config)
    }

    pub fn resolve(&self, variants: &HashMap<String, VariantValue>) -> serde_json::Value {
        if variants.len() > 0 {
            let mut merged = HashMap::with_capacity(self.global_variants.len() + variants.len());
            for (k, v) in variants {
                merged.insert(k.to_owned(), v.clone());
            }
            for (k, v) in &*self.global_variants {
                if !merged.contains_key(k) {
                    merged.insert(k.to_owned(), v.clone());
                }
            }
            self.resolver.resolve(&merged)
        } else {
            self.resolver.resolve(&self.global_variants)
        }
    }

    pub fn resolve_typed<T: DeserializeOwned>(
        &self,
        variants: &HashMap<String, VariantValue>,
    ) -> anyhow::Result<T> {
        let json = self.resolve(variants);
        let r = serde_json::from_value(json)?;
        Ok(r)
    }

    pub fn update_json(&mut self, json: serde_json::Value) -> anyhow::Result<()> {
        let resolver = JsonConfigResolver::new_with_custom_path(
            json,
            self.config.condition_path.clone(),
            self.config.value_path.clone(),
        )?;
        self.resolver = Arc::new(resolver);
        Ok(())
    }

    pub fn update_json_with_config(
        &mut self,
        json: serde_json::Value,
        config: &VariantConfigStoreConfig,
    ) -> anyhow::Result<()> {
        let resolver = JsonConfigResolver::new_with_custom_path(
            json,
            config.condition_path.clone(),
            config.value_path.clone(),
        )?;
        self.resolver = Arc::new(resolver);
        self.config = Arc::new(config.clone());
        Ok(())
    }

    pub fn update_global_variants(&mut self, global_variants: HashMap<String, VariantValue>) {
        self.global_variants = Arc::new(global_variants);
    }
}
