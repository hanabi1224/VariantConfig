#![feature(test)]

#[cfg(test)]
mod bench {
    extern crate test;
    use test::Bencher;

    use std::fs;
    use variant_config::{dsl::VariantValue, hashbrown::HashMap, *};

    #[bench]
    fn test_bench_simple(bencher: &mut Bencher) {
        let (store, variants) = setup("tests/data/simple.yaml");
        bencher.iter(|| store.resolve(&variants));
    }

    #[bench]
    fn bench_complicated(bencher: &mut Bencher) {
        let (store, variants) = setup("tests/data/c1.yaml");
        bencher.iter(|| store.resolve(&variants));
    }

    fn setup(file_path: &str) -> (VariantConfigStore, HashMap<String, VariantValue>) {
        let contents = fs::read_to_string(file_path).unwrap();
        let mut user_variants = HashMap::new();
        user_variants.insert("Dummy".to_owned(), VariantValue::Bool(false));
        user_variants.insert("VAR1".to_owned(), VariantValue::Bool(true));
        user_variants.insert("VAR2".to_owned(), VariantValue::Bool(false));
        (
            VariantConfigStore::new_from_yaml(&contents, HashMap::with_capacity(0)).unwrap(),
            user_variants,
        )
    }
}
