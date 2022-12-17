use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use variant_config::{hashbrown::HashMap, *};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertPayload {
    pub variants: String,
    pub content: String,
    pub type_: String,
}

pub async fn convert(payload: axum::Json<ConvertPayload>) -> impl IntoResponse {
    convert_inner(payload).await.unwrap()
}

async fn convert_inner(payload: axum::Json<ConvertPayload>) -> anyhow::Result<impl IntoResponse> {
    let mut variants = HashMap::new();
    let json: serde_json::Value = serde_json::from_str(&payload.variants)?;
    if let Some(object) = json.as_object() {
        for (k, v) in object {
            let variant_value = match v {
                serde_json::Value::String(s) => VariantValue::String(s.clone()),
                serde_json::Value::Bool(b) => VariantValue::String(format!("{}", b)),
                serde_json::Value::Number(n) => {
                    if let Some(int) = n.as_i64() {
                        VariantValue::Int(int)
                    } else {
                        VariantValue::String(format!("{}", n))
                    }
                }
                _ => continue,
            };
            variants.insert(k.clone(), variant_value);
        }
    }

    let t = &payload.0.type_;
    let ret_str = if t == "yaml" {
        let store = VariantConfigStore::new_from_yaml(&payload.content, variants).unwrap();
        let ret = store.resolve(&HashMap::with_capacity(0));
        let yaml = serde_json::from_value::<serde_yaml::Value>(ret)?;
        serde_yaml::to_string(&yaml).unwrap()
    } else if t == "toml" {
        let store = VariantConfigStore::new_from_toml(&payload.content, variants).unwrap();
        let ret = store.resolve(&HashMap::with_capacity(0));
        // format!("{}", ret)
        let toml = serde_json::from_value::<toml::Value>(ret)?;
        format!("{}", toml)
    } else {
        let value = serde_json::from_str(&payload.content)?;
        let store = VariantConfigStore::new(value, variants).unwrap();
        let ret = store.resolve(&HashMap::with_capacity(0));
        format!("{}", ret)
    };

    Ok(ret_str)
}
