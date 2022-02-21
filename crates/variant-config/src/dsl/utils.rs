use super::RandomState;
use std::hash::{BuildHasher, Hasher};

pub fn get_string_hash(random_state: &RandomState, s: &str) -> i64 {
    if !s.is_empty() {
        let mut hasher = random_state.build_hasher();
        hasher.write(s.as_bytes());
        hasher.finish() as i64
    } else {
        0
    }
}
