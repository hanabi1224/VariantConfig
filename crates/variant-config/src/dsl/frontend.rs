/// The AST node for expressions.
pub enum Expr {
    IntLiteral(i64),
    BoolLiteral(bool),
    StringLiteral(String),
    Identifier(String),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
}

peg::parser!(pub grammar parser() for str {
    pub rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    pub rule statement() -> Expr
        = [' ' | '\t' | '\n']* _ e:expression() _ [' ' | '\t' | '\n']* { e }

    pub rule expression() -> Expr
        = binary_op()

    rule binary_op() -> Expr = precedence!{
        a:@ _ "||" _ b:(@) { Expr::Or(Box::new(a), Box::new(b)) }
        a:@ _ "or" _ b:(@) { Expr::Or(Box::new(a), Box::new(b)) }
        --
        a:@ _ "&&" _ b:(@) { Expr::And(Box::new(a), Box::new(b)) }
        a:@ _ "and" _ b:(@) { Expr::And(Box::new(a), Box::new(b)) }
        --
        a:@ _ "==" _ b:(@) { Expr::Eq(Box::new(a), Box::new(b)) }
        a:@ _ "eq" _ b:(@) { Expr::Eq(Box::new(a), Box::new(b)) }
        a:@ _ "!=" _ b:(@) { Expr::Ne(Box::new(a), Box::new(b)) }
        a:@ _ "ne" _ b:(@) { Expr::Ne(Box::new(a), Box::new(b)) }
        a:@ _ "<"  _ b:(@) { Expr::Lt(Box::new(a), Box::new(b)) }
        a:@ _ "lt"  _ b:(@) { Expr::Lt(Box::new(a), Box::new(b)) }
        a:@ _ "<=" _ b:(@) { Expr::Le(Box::new(a), Box::new(b)) }
        a:@ _ "le" _ b:(@) { Expr::Le(Box::new(a), Box::new(b)) }
        a:@ _ ">"  _ b:(@) { Expr::Gt(Box::new(a), Box::new(b)) }
        a:@ _ "gt"  _ b:(@) { Expr::Gt(Box::new(a), Box::new(b)) }
        a:@ _ ">=" _ b:(@) { Expr::Ge(Box::new(a), Box::new(b)) }
        a:@ _ "ge" _ b:(@) { Expr::Ge(Box::new(a), Box::new(b)) }
        --
        a:@ _ "+" _ b:(@) { Expr::Add(Box::new(a), Box::new(b)) }
        a:@ _ "-" _ b:(@) { Expr::Sub(Box::new(a), Box::new(b)) }
        --
        a:@ _ "*" _ b:(@) { Expr::Mul(Box::new(a), Box::new(b)) }
        a:@ _ "/" _ b:(@) { Expr::Div(Box::new(a), Box::new(b)) }
        a:@ _ "%" _ b:(@) { Expr::Mod(Box::new(a), Box::new(b)) }
        --
        b:bool_literal(){ b }
        l:int_literal() { l }
        s: string_literal() { s }
        i:identifier() { Expr::Identifier(i) }
        --
        "(" _ b:binary_op() _ ")" { b }
    }

    rule types() -> String
        = n:$("int") { n.to_owned() }
        / n:$("bool") { n.to_owned() }

    rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_' | '@' | '$']['0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '@' | '$']*) { n.to_owned() } }
        / expected!("identifier")

    rule int_literal() -> Expr
        = n:$(['0'..='9']+) { Expr::IntLiteral(n.parse().unwrap()) }
        / n:$(['-']['1'..='9']['0'..='9']*) { Expr::IntLiteral(n.parse().unwrap()) }

    rule bool_literal() -> Expr
        = "true" { Expr::BoolLiteral(true) }
        / "false" { Expr::BoolLiteral(false) }

    rule string_literal_content() -> String
        = s:$(['0'..='9' | 'a'..='z' | 'A'..='Z' | '_' | '@' | '$' | ' ']*) { s.to_owned() }

    rule string_literal() -> Expr
        = "\"" s:string_literal_content() "\"" { Expr::StringLiteral(s) }
        / "'" s:string_literal_content() "'" { Expr::StringLiteral(s) }
        / "\"\"" { Expr::StringLiteral("".to_owned()) }
        / "''" { Expr::StringLiteral("".to_owned()) }
        / expected!("string_literal")

    rule _() =  quiet!{[' ' | '\t']*}
});
