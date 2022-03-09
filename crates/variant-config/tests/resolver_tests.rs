#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use serde_json::*;
    use variant_config::*;

    #[test]
    fn test_no_variant_1() {
        let json = json!({
            "a":1,
            "b":3
        });
        let resolver = JsonConfigResolver::new(json.clone()).unwrap();
        let ctx = HashMap::new();
        let r = resolver.resolve(&ctx);
        assert_eq!(r, json);
    }

    #[test]
    fn test_condition_non_str_1() {
        let json = json!({
            "a": [
                {
                    "if": Value::Null,
                    "value": 1
                },
            ]
        });
        let resolver = JsonConfigResolver::new(json).unwrap();
        let ctx = HashMap::new();
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
            })
        );
    }

    #[test]
    fn test_condition_non_str_2() {
        let json = json!({
            "a": [
                {
                    "if": true,
                    "value": 1
                },
            ]
        });
        let resolver = JsonConfigResolver::new(json).unwrap();
        let ctx = HashMap::new();
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
            })
        );
    }

    #[test]
    fn test_condition_non_str_3() {
        let json = json!({
            "a": [
                {
                    "if": 1,
                    "value": 1
                },
            ]
        });
        let resolver = JsonConfigResolver::new(json).unwrap();
        let ctx = HashMap::new();
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
            })
        );
    }

    #[test]
    fn test_variant_1() {
        let json = json!({
            "a": [1,2],
            "b": [
                {
                    "if": "a > 5",
                    "value": 5
                },
                {
                    "if": "a > 3",
                    "value": 3
                },
                {
                    "if": "true",
                    "value": 0
                },
            ]
        });
        let resolver = JsonConfigResolver::new(json).unwrap();
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(6));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":[1,2],
                "b":5,
            })
        );

        ctx.insert("a".to_owned(), VariantValue::Int(4));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":[1,2],
                "b":3,
            })
        );

        ctx.insert("a".to_owned(), VariantValue::Int(1));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":[1,2],
                "b":0,
            })
        );
    }

    #[test]
    fn test_variant_2() {
        let json = json!({
            "a":1,
            "b": [
                {
                    "if": "a > 5",
                    "value": {
                        "c": 3,
                        "d": [{
                            "if": "b == 'what'",
                            "value": 8,
                        },
                        {
                            "if": "true",
                            "value": 6,
                        }],
                    }
                },
                {
                    "if": "a > 3",
                    "value": 3
                },
                {
                    "if": true,
                    "value": {
                        "c": 7,
                        "d": [{
                            "if": "b == ''",
                            "value": -3,
                        },
                        {
                            "if": "true",
                            "value": -1,
                        }],
                    }
                },
            ]
        });
        let resolver = JsonConfigResolver::new(json).unwrap();
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(6));
        ctx.insert("b".to_owned(), VariantValue::String("what".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
                "b":{
                    "c":3,
                    "d":8
                },
            })
        );

        ctx.insert("a".to_owned(), VariantValue::Int(6));
        ctx.insert("b".to_owned(), VariantValue::String("no".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
                "b":{
                    "c":3,
                    "d":6
                },
            })
        );

        ctx.insert("a".to_owned(), VariantValue::Int(4));
        ctx.insert("b".to_owned(), VariantValue::Int(4));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
                "b":3,
            })
        );

        ctx.insert("a".to_owned(), VariantValue::Int(1));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
                "b": {
                    "c": 7,
                    "d":-1,
                },
            })
        );

        ctx.insert("a".to_owned(), VariantValue::Int(1));
        ctx.insert("b".to_owned(), VariantValue::String("".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
                "b": {
                    "c": 7,
                    "d":-3,
                },
            })
        );

        ctx.insert("a".to_owned(), VariantValue::Int(1));
        ctx.insert("b".to_owned(), VariantValue::String("no".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r,
            json!({
                "a":1,
                "b": {
                    "c": 7,
                    "d":-1,
                },
            })
        );
    }
}
