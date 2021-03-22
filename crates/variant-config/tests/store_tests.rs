#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use variant_config::*;

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    struct ConfigP3 {
        pub p4: String,
        pub p5: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    struct Config {
        pub p1: i32,
        pub p2: Vec<i32>,
        pub p3: ConfigP3,
    }

    impl Config {
        pub fn expected_base() -> Self {
            Self {
                p1: 1,
                p2: vec![2, 3, 4],
                p3: ConfigP3 {
                    p4: "p4".to_owned(),
                    p5: "p5".to_owned(),
                },
            }
        }
    }

    #[test]
    fn test_load_json() {
        let fp = "tests/data/c1.json";
        let contents = fs::read_to_string(fp).unwrap();
        let json = serde_json::from_str(&contents).unwrap();
        let store = VariantConfigStore::new(json, HashMap::new()).unwrap();
        test_resolve_config(&store);
    }

    #[test]
    fn test_load_toml() {
        let fp = "tests/data/c1.toml";
        let contents = fs::read_to_string(fp).unwrap();
        let store = VariantConfigStore::new_from_toml(&contents, HashMap::new()).unwrap();
        test_resolve_config(&store);
    }

    #[test]
    fn test_load_yaml() {
        let fp = "tests/data/c1.yaml";
        let contents = fs::read_to_string(fp).unwrap();
        let store = VariantConfigStore::new_from_yaml(&contents, HashMap::new()).unwrap();
        test_resolve_config(&store);
    }

    fn test_resolve_config(store: &VariantConfigStore) {
        let mut variants = HashMap::new();
        variants.insert("VAR1".to_owned(), VariantValue::Int(100));
        variants.insert("VAR2".to_owned(), VariantValue::String("what".to_owned()));
        let config = store.resolve_typed::<Config>(&variants).unwrap();
        let mut expected = Config::expected_base();
        expected.p3.p4 = "p41".to_owned();
        expected.p3.p5 = "what".to_owned();
        assert_eq!(config, expected);

        variants.insert("VAR1".to_owned(), VariantValue::Int(100));
        variants.insert("VAR2".to_owned(), VariantValue::String("why".to_owned()));
        let config = store.resolve_typed::<Config>(&variants).unwrap();
        let mut expected = Config::expected_base();
        expected.p3.p4 = "p41".to_owned();
        expected.p3.p5 = "why".to_owned();
        assert_eq!(config, expected);

        variants.insert("VAR1".to_owned(), VariantValue::Int(100));
        variants.insert("VAR2".to_owned(), VariantValue::String("how".to_owned()));
        let config = store.resolve_typed::<Config>(&variants).unwrap();
        let mut expected = Config::expected_base();
        expected.p3.p4 = "p41".to_owned();
        expected.p3.p5 = "how".to_owned();
        assert_eq!(config, expected);

        variants.insert("VAR1".to_owned(), VariantValue::Int(100));
        variants.insert(
            "VAR2".to_owned(),
            VariantValue::String("whatever".to_owned()),
        );
        let config = store.resolve_typed::<Config>(&variants).unwrap();
        let mut expected = Config::expected_base();
        expected.p3.p4 = "p41".to_owned();
        expected.p3.p5 = "".to_owned();
        assert_eq!(config, expected);

        variants.insert("VAR1".to_owned(), VariantValue::Int(10));
        variants.insert(
            "VAR2".to_owned(),
            VariantValue::String("whatever".to_owned()),
        );
        let config = store.resolve_typed::<Config>(&variants).unwrap();
        let mut expected = Config::expected_base();
        expected.p3.p4 = "p42".to_owned();
        expected.p3.p5 = "no".to_owned();
        assert_eq!(config, expected);
    }
}
