use crate::dsl::{FnJitter, VariantValue};
use anyhow;
use hashbrown::HashMap;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::BTreeMap;

struct NodeCandidateResolver {
    index: usize,
    condition: FnJitter,
}

impl NodeCandidateResolver {
    pub fn new(index: usize, on: &str) -> anyhow::Result<Self> {
        let jitter = FnJitter::new(on)?;
        Ok(Self {
            index,
            condition: jitter,
        })
    }

    pub unsafe fn free_memory(self) {
        self.condition.free_memory();
    }
}

pub struct JsonConfigResolver {
    condition_path: String,
    value_path: String,
    json: Value,
    node_resolvers: BTreeMap<String, RefCell<Vec<NodeCandidateResolver>>>,
}

impl JsonConfigResolver {
    pub fn new(json: Value) -> anyhow::Result<JsonConfigResolver> {
        Self::new_with_custom_path(json, "if".to_owned(), "value".to_owned())
    }

    pub fn new_with_custom_path(
        json: Value,
        condition_path: String,
        value_path: String,
    ) -> anyhow::Result<Self> {
        let mut r = Self {
            condition_path,
            value_path,
            json: Value::default(),
            node_resolvers: BTreeMap::new(),
        };
        r.set_json(json)?;
        Ok(r)
    }

    pub fn resolve(&self, ctx: &HashMap<String, VariantValue>) -> Value {
        let mut ret = self.json.clone();
        if self.node_resolvers.len() > 0 {
            for (path, resolvers) in self.node_resolvers.iter().rev() {
                if let Some(ptr_mut) = ret.pointer_mut(path) {
                    let mut match_value = Value::Null;
                    for resolver in resolvers.borrow().iter() {
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

    fn set_json(&mut self, json: Value) -> anyhow::Result<()> {
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
                            if let Some(v) = item.get(&self.condition_path) {
                                match v {
                                    Value::String(on) => {
                                        let r = NodeCandidateResolver::new(idx, &on)?;
                                        node_resolvers.push(r);
                                    }
                                    Value::Null => {
                                        let r = NodeCandidateResolver::new(idx, "1")?;
                                        node_resolvers.push(r);
                                    }
                                    Value::Bool(true) => {
                                        let r = NodeCandidateResolver::new(idx, "1")?;
                                        node_resolvers.push(r);
                                    }
                                    Value::Number(n) => {
                                        let r = NodeCandidateResolver::new(idx, &n.to_string())?;
                                        node_resolvers.push(r);
                                    }
                                    _ => {}
                                }
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
                            .insert(merge_json_path(path), RefCell::new(node_resolvers));
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
            if let Some(_) = v.get(&self.value_path) {
                if let Some(c) = v.get(&self.condition_path) {
                    return !c.is_array() && !c.is_object();
                }
            }
        }
        return false;
    }

    fn get_keys(&self) -> Vec<String> {
        self.node_resolvers.keys().cloned().collect()
    }
}

impl Drop for JsonConfigResolver {
    fn drop(&mut self) {
        for k in self.get_keys() {
            if let Some(v) = self.node_resolvers.remove(&k) {
                let mut vec = v.borrow_mut();
                while vec.len() > 0 {
                    let r = vec.remove(0);
                    unsafe { r.free_memory() };
                }
            }
        }
    }
}

fn merge_json_path(path: &Vec<String>) -> String {
    format!("/{}", path.join("/"))
}
