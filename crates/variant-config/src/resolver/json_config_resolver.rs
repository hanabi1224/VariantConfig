use crate::dsl::{ContextValue, FnJitter};
use anyhow;
use hashbrown::HashMap;
use serde_json::Value;
use std::collections::BTreeMap;

struct NodeCandidateResolver {
    index: usize,
    condition: FnJitter,
}

impl NodeCandidateResolver {
    pub fn new(index: usize, on: &str) -> anyhow::Result<NodeCandidateResolver> {
        let jitter = FnJitter::new(on)?;
        Ok(Self {
            index,
            condition: jitter,
        })
    }
}

pub struct JsonConfigResolver {
    condition_path: String,
    value_path: String,
    json: Value,
    node_resolvers: BTreeMap<String, Vec<NodeCandidateResolver>>,
}

impl JsonConfigResolver {
    pub fn new(json: &str) -> anyhow::Result<JsonConfigResolver> {
        Self::new_with_custom_path(json, "if".to_owned(), "value".to_owned())
    }

    pub fn new_with_custom_path(
        json: &str,
        condition_path: String,
        value_path: String,
    ) -> anyhow::Result<JsonConfigResolver> {
        let mut r = Self {
            condition_path,
            value_path,
            json: Value::default(),
            node_resolvers: BTreeMap::new(),
        };
        r.set_json(json)?;
        Ok(r)
    }

    pub fn resolve(&self, ctx: &HashMap<String, ContextValue>) -> Value {
        let mut ret = self.json.clone();
        if self.node_resolvers.len() > 0 {
            for (path, resolvers) in self.node_resolvers.iter().rev() {
                if let Some(ptr_mut) = ret.pointer_mut(path) {
                    let mut match_value = Value::Null;
                    for resolver in resolvers {
                        if resolver.condition.evaluate(ctx) {
                            let k = format!("/{}/{}", resolver.index, self.value_path);
                            if let Some(v) = ptr_mut.pointer(&k) {
                                match_value = v.clone();
                                break;
                            }
                        }
                    }
                    *ptr_mut = match_value;
                }
            }
        }
        ret
    }

    fn set_json(&mut self, json: &str) -> anyhow::Result<()> {
        let json: Value = serde_json::from_str(json)?;
        let mut path = Vec::new();
        self.parse_variants(&json, &mut path)?;
        self.json = json;
        Ok(())
    }

    fn parse_variants(&mut self, node: &Value, path: &mut Vec<String>) -> anyhow::Result<()> {
        match node {
            Value::Array(vec) => {
                if vec.len() > 0 {
                    let is_variant_array = vec.iter().all(|i| self.is_variant_array(i));
                    let mut node_resolvers: Vec<NodeCandidateResolver>;
                    if is_variant_array {
                        node_resolvers = Vec::with_capacity(vec.len());
                    } else {
                        node_resolvers = Vec::with_capacity(0);
                    }
                    for (idx, item) in vec.iter().enumerate() {
                        if is_variant_array {
                            let value = item.get(&self.value_path).unwrap();
                            if let Some(Value::String(on)) = item.get(&self.condition_path) {
                                let r = NodeCandidateResolver::new(idx, &on)?;
                                node_resolvers.push(r);
                            }

                            path.push(format!("{}", idx));
                            path.push(format!("{}", self.value_path));
                            self.parse_variants(value, path)?;
                            path.pop();
                            path.pop();
                        } else {
                            path.push(format!("{}", idx));
                            self.parse_variants(item, path)?;
                            path.pop();
                        }
                    }
                    if is_variant_array {
                        self.node_resolvers
                            .insert(merge_json_path(path), node_resolvers);
                    }
                }
            }
            Value::Object(map) => {
                for (k, v) in map {
                    path.push(k.clone());
                    self.parse_variants(v, path)?;
                    path.pop();
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn is_variant_array(&self, v: &Value) -> bool {
        if v.is_object() {
            if let Some(Value::String(_)) = v.get(&self.condition_path) {
                if let Some(_) = v.get(&self.value_path) {
                    return true;
                }
            }
        }
        return false;
    }
}

fn merge_json_path(path: &Vec<String>) -> String {
    format!("/{}", path.join("/"))
}
