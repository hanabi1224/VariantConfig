use super::utils::get_string_hash;
use super::RandomState;

#[derive(Debug, Clone)]
pub enum VariantValue {
    Int(i64),
    Bool(bool),
    String(String),
}

impl VariantValue {
    pub fn to_i64(&self, random_state: &RandomState) -> i64 {
        match self {
            Self::Int(i) => *i,
            Self::Bool(true) => 1,
            Self::Bool(false) => 0,
            Self::String(s) => get_string_hash(random_state, s),
        }
    }
}
