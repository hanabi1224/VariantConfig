use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

pub fn get_string_hash(random_state: &RandomState, s: &str) -> i64 {
    if s.len() > 0 {
        let mut hasher = random_state.build_hasher();
        hasher.write(s.as_bytes());
        hasher.finish() as i64
    } else {
        0
    }
}
