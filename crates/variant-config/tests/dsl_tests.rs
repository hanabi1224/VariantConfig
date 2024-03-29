#[cfg(test)]
mod tests {
    use variant_config::dsl::{FnJitter, VariantValue};
    use variant_config::hashbrown::HashMap;

    #[test]
    fn test_str_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::String("what".to_owned()));
        ctx.insert("b".to_owned(), VariantValue::Bool(true));
        const CODE: &str = "a == 'what' and b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_str_2() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::String("no".to_owned()));
        ctx.insert("b".to_owned(), VariantValue::Bool(false));
        const CODE: &str = "a == 'what' or b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(!ret);
    }

    #[test]
    fn test_int_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        const CODE: &str = "a > 8 ";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_int_2() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        const CODE: &str = r#" a < 8 "#;
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(!ret);
    }

    #[test]
    fn test_add_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "a + b == 13";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_add_2() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "b + a == 13";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_sub_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "a - b == 7";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_sub_2() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "b - a == -7";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_mul_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "a * b == 30";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_div_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "a / b == 3";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_mod_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "1 == a % b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_and_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "1 and a > b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_and_2() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "1 and a < b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(!ret);
    }

    #[test]
    fn test_or_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "1 or a < b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_or_2() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "0 or a > b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_or_3() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "0 or a < b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(!ret);
    }

    #[test]
    fn test_bool_1() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "true and a < b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(!ret);
    }

    #[test]
    fn test_bool_2() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "true and a > b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_bool_3() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "false or a > b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(ret);
    }

    #[test]
    fn test_bool_4() {
        let mut ctx = HashMap::new();
        ctx.insert("a".to_owned(), VariantValue::Int(10));
        ctx.insert("b".to_owned(), VariantValue::Int(3));
        const CODE: &str = "false or a < b";
        let jitter = FnJitter::new(CODE).unwrap();
        let ret = jitter.evaluate(&ctx);
        assert!(!ret);
    }
}
