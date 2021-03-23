use actix_web::*;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use variant_config::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertPayload {
    pub variants: serde_json::Value,
    pub content: String,
    pub type_: String,
}

pub async fn convert(payload: web::Json<ConvertPayload>) -> Result<HttpResponse> {
    let mut variants = HashMap::new();
    if let Some(object) = payload.variants.as_object() {
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
    let store = if t == "yaml" {
        VariantConfigStore::new_from_yaml(&payload.content, variants).unwrap()
    } else if t == "toml" {
        VariantConfigStore::new_from_toml(&payload.content, variants).unwrap()
    } else {
        let value = serde_json::from_str(&payload.content)?;
        VariantConfigStore::new(value, variants).unwrap()
    };

    let ret = store.resolve(&HashMap::with_capacity(0));
    Ok(HttpResponse::Ok().content_type("plain/text").json(ret))
}
