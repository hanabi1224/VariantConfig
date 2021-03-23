use hashbrown::HashMap;
use std::fs;
use std::time;
use variant_config::*;

fn main() {
    let fp = "tests/data/c1.json";
    let contents = fs::read_to_string(fp).unwrap();
    let json = serde_json::from_str(&contents).unwrap();
    let mut user_variants = HashMap::new();
    user_variants.insert("Dummy".to_owned(), VariantValue::Bool(false));
    user_variants.insert("VAR1".to_owned(), VariantValue::Bool(true));
    user_variants.insert("VAR2".to_owned(), VariantValue::Bool(false));
    let store = VariantConfigStore::new(json, HashMap::with_capacity(0)).unwrap();

    let n_loop = 200000;
    bench("complicated", n_loop, || {
        resolve_json(&store, &user_variants)
    });

    let fp = "tests/data/simple.yaml";
    let contents = fs::read_to_string(fp).unwrap();
    let store = VariantConfigStore::new_from_yaml(&contents, HashMap::with_capacity(0)).unwrap();
    bench("simple", n_loop, || resolve_json(&store, &user_variants));
}

fn bench<F>(metric: &str, n_loop: usize, func: F)
where
    F: Fn(),
{
    let t_start = time::Instant::now();
    for _ in 0..n_loop {
        func();
    }
    let t_end = time::Instant::now();
    let diff = t_end - t_start;
    println!("[{}] Total time elapsed: {}s", metric, diff.as_secs_f32());
    println!(
        "[{}] Avg time elapsed: {}ms",
        metric,
        diff.as_secs_f32() * 1000.0 / n_loop as f32
    );
}

fn resolve_json(store: &VariantConfigStore, variants: &HashMap<String, VariantValue>) {
    store.resolve(variants);
}
