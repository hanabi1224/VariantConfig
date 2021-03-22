#[cfg(test)]
mod tests {
    use hashbrown::HashMap;
    use serde_json::*;
    use variant_config::*;

    #[test]
    fn test_no_variant_1() {
        let json = json!({
            "a":1,
            "b":2
        })
        .to_string();
        let resolver = JsonConfigResolver::new(&json).unwrap();
        let ctx = HashMap::new();
        let r = resolver.resolve(&ctx);
        assert_eq!(r.to_string(), json);
    }

    #[test]
    fn test_variant_1() {
        let json = json!({
            "a":1,
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
        })
        .to_string();
        let resolver = JsonConfigResolver::new(&json).unwrap();
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), ContextValue::Int(6));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b":5,
            })
            .to_string()
        );

        ctx.insert("a".to_owned(), ContextValue::Int(4));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b":3,
            })
            .to_string()
        );

        ctx.insert("a".to_owned(), ContextValue::Int(1));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b":0,
            })
            .to_string()
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
                    "if": "true",
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
        })
        .to_string();
        let resolver = JsonConfigResolver::new(&json).unwrap();
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), ContextValue::Int(6));
        ctx.insert("b".to_owned(), ContextValue::String("what".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b":{
                    "c":3,
                    "d":8
                },
            })
            .to_string()
        );

        ctx.insert("a".to_owned(), ContextValue::Int(6));
        ctx.insert("b".to_owned(), ContextValue::String("no".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b":{
                    "c":3,
                    "d":6
                },
            })
            .to_string()
        );

        ctx.insert("a".to_owned(), ContextValue::Int(4));
        ctx.insert("b".to_owned(), ContextValue::Int(4));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b":3,
            })
            .to_string()
        );

        ctx.insert("a".to_owned(), ContextValue::Int(1));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b": {
                    "c": 7,
                    "d":-1,
                },
            })
            .to_string()
        );

        ctx.insert("a".to_owned(), ContextValue::Int(1));
        ctx.insert("b".to_owned(), ContextValue::String("".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b": {
                    "c": 7,
                    "d":-3,
                },
            })
            .to_string()
        );

        ctx.insert("a".to_owned(), ContextValue::Int(1));
        ctx.insert("b".to_owned(), ContextValue::String("no".to_owned()));
        let r = resolver.resolve(&ctx);
        assert_eq!(
            r.to_string(),
            json!({
                "a":1,
                "b": {
                    "c": 7,
                    "d":-1,
                },
            })
            .to_string()
        );
    }
}
