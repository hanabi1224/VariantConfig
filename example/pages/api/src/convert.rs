use super::*;

fn convert_handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let body = request.into_body();
    let ret_txt = match body {
        Body::Empty => "empty body".to_owned(),
        Body::Text(payload_str) => {
            if let Ok(ret) = process_convert_payload(&payload_str) {
                ret
            } else {
                payload_str
            }
        }
        Body::Binary(payload_bytes) => {
            if let Ok(payload_str) = String::from_utf8(payload_bytes) {
                if let Ok(ret) = process_convert_payload(&payload_str) {
                    ret
                } else {
                    payload_str
                }
            } else {
                "binary body".to_owned()
            }
        }
    };

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(ret_txt)
        .expect("Internal Server Error");

    Ok(response)
}

fn process_convert_payload(payload_str: &str) -> Result<String, anyhow::Error> {
    let payload = serde_json::from_str::<ConvertPayload>(payload_str)?;
    let mut variants = HashMap::new();
    let json: serde_json::Value = serde_json::from_str(&payload.variants)?;
    if let Some(object) = json.as_object() {
        for (k, v) in object {
            let variant_value = match v {
                serde_json::Value::String(s) => VariantValue::String(s.clone()),
                serde_json::Value::Bool(b) => VariantValue::String(format!("{b}")),
                serde_json::Value::Number(n) => {
                    if let Some(int) = n.as_i64() {
                        VariantValue::Int(int)
                    } else {
                        VariantValue::String(format!("{n}"))
                    }
                }
                _ => continue,
            };
            variants.insert(k.clone(), variant_value);
        }
    }

    let t = &payload.type_;
    let ret = if t == "yaml" {
        let store = VariantConfigStore::new_from_yaml(&payload.content, variants)?;
        let ret = store.resolve(&HashMap::with_capacity(0));
        let yaml = serde_json::from_value::<serde_yaml::Value>(ret)?;
        serde_yaml::to_string(&yaml)?
    } else if t == "toml" {
        let store = VariantConfigStore::new_from_toml(&payload.content, variants)?;
        let ret = store.resolve(&HashMap::with_capacity(0));
        let toml = serde_json::from_value::<toml::Value>(ret)?;
        format!("{toml}")
    } else {
        let value = serde_json::from_str(&payload.content)?;
        let store = VariantConfigStore::new(value, variants)?;
        let ret = store.resolve(&HashMap::with_capacity(0));
        format!("{ret}")
    };
    Ok(ret)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConvertPayload {
    pub variants: String,
    pub content: String,
    pub type_: String,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(lambda!(convert_handler))
}
