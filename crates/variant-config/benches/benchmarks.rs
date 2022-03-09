use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use variant_config::{dsl::VariantValue, hashbrown::HashMap, *};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple", |b| {
        let (store, variants) = setup("tests/data/simple.yaml");
        b.iter(|| store.resolve(&variants))
    });

    c.bench_function("complicated", |b| {
        let (store, variants) = setup("tests/data/c1.yaml");
        b.iter(|| store.resolve(&variants))
    });
}

fn setup(file_path: &str) -> (VariantConfigStore, HashMap<String, VariantValue>) {
    let contents = fs::read_to_string(file_path).unwrap();
    let mut user_variants = HashMap::new();
    user_variants.insert("Dummy".to_owned(), VariantValue::Bool(false));
    user_variants.insert("VAR1".to_owned(), VariantValue::Bool(true));
    user_variants.insert("VAR2".to_owned(), VariantValue::Bool(false));
    (
        black_box(VariantConfigStore::new_from_yaml(&contents, HashMap::with_capacity(0)).unwrap()),
        black_box(user_variants),
    )
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
