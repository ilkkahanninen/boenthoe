use crate::ast::*;

peg::parser! {
  grammar bs_parser() for str {

    pub rule script() -> Expr
        = __ e:(export() / expr()) ** __ __ { Expr::list(KW_ROOT, e) }

    pub rule expr_list() -> Vec<Expr>
        = quiet! { l:expr() ** __ { l } }
        / expected!("expression list")

    pub rule expr() -> Expr
        = quiet! {
            line_comment()
            / block()
            / array()
            / fn_call()
            / define()
            / number()
            / symbol()
        }
        / expected!("expression")

    pub rule number() -> Expr
        = quiet!{ n:$(['+'|'-']?['0'..='9']+(['.']['0'..='9']+)?) {
            let f: f64 = n.parse().unwrap();
            f.into()
        } }
        / expected!("number")

    rule symbol() -> Expr
        = s:symbol_str() { Expr::Symbol(s) }

    rule symbol_str() -> String
        = quiet!{ s:$(['a'..='z'|'A'..='Z']['a'..='z'|'A'..='Z'|'0'..='9'|'.'|'_']*) { String::from(s) } }
        / expected!("symbol")

    rule fn_call() -> Expr
        = quiet! { n:symbol_str() _ "(" __ c:expr() ** ("," __) __ ")" { Expr::list(&n, c) } }
        / expected!("function call")

    rule define() -> Expr
        = quiet! { s:symbol() __ "=" __ v:expr() { Expr::list(KW_DEFINE, vec![s, v]) } }
        / expected!("assignment")

    rule block() -> Expr
        = quiet! { "{" __ l:expr_list() __ "}" { Expr::list(KW_BLOCK, l) } }
        / expected!("block")

    rule array() -> Expr
        = quiet! { "[" __ l:expr() ** ("," __) __ "]" { Expr::list(KW_ARRAY, l) } }
        / expected!("array")

    rule export() -> Expr
        = quiet! { "out" _ s:symbol() __ "=" __ v:expr() { Expr::list(KW_EXPORT, vec![s, v] )} }
        / expected!("export expression")

    rule _() = quiet!{ [' '|'\t']* }
    rule __() = quiet!{ [' '|'\t'|'\n']* }

    rule line_comment() -> Expr
        = quiet! { "//" c:$([x if x != '\n']*) "\n" { Expr::Comment(String::from(c)) } }
        / expected!("comment")
  }
}

pub fn parse(script: &str) -> Result<Expr, peg::error::ParseError<peg::str::LineCol>> {
    bs_parser::script(script)
}

#[test]
fn number_parsing() {
    assert_eq!(bs_parser::expr("123"), Ok(123.0.into()));
    assert_eq!(bs_parser::expr("123.2"), Ok(123.2.into()));
}

#[test]
fn symbol_parsing() {
    assert_eq!(bs_parser::expr("foobar.zap1"), Ok("foobar.zap1".into()));
}

#[test]
fn fn_call_parsing() {
    assert_eq!(
        bs_parser::expr("hold(1, 4)"),
        Ok(Expr::list("hold", vec![1.0.into(), 4.0.into()]))
    );

    assert_eq!(
        bs_parser::expr("linear(black, red, 1.1)"),
        Ok(Expr::list(
            "linear",
            vec!["black".into(), "red".into(), 1.1.into()]
        ))
    );

    assert_eq!(
        bs_parser::expr("concat(hold(1, 1.0), linear(1, 0, 1.5))"),
        Ok(Expr::list(
            "concat",
            vec![
                Expr::list("hold", vec![1.0.into(), 1.0.into()]),
                Expr::list("linear", vec![1.0.into(), 0.0.into(), 1.5.into()])
            ]
        ))
    );
}

#[test]
fn define_parsing() {
    assert_eq!(
        bs_parser::expr("pi = 3.14"),
        Ok(Expr::list(KW_DEFINE, vec!["pi".into(), 3.14.into()]))
    );

    assert_eq!(
        bs_parser::expr("off = hold(0)"),
        Ok(Expr::list(
            KW_DEFINE,
            vec!["off".into(), Expr::List("hold".into(), vec![0.0.into()])]
        ))
    );
}

#[test]
fn expr_list_parsing() {
    assert_eq!(
        bs_parser::expr_list(
            "a = 1\n\
             b = 2\n\
             c = add(a, b)"
        ),
        Ok(vec![
            Expr::list(KW_DEFINE, vec!["a".into(), 1.0.into()]),
            Expr::list(KW_DEFINE, vec!["b".into(), 2.0.into()]),
            Expr::list(
                KW_DEFINE,
                vec!["c".into(), Expr::list("add", vec!["a".into(), "b".into()])]
            )
        ])
    )
}

#[test]
fn scope_parsing() {
    assert_eq!(
        bs_parser::expr_list(
            "a = 1\n\
             b = {\n\
                foo([a, 2])\n\
             }"
        ),
        Ok(vec![
            Expr::list(KW_DEFINE, vec!["a".into(), 1.0.into()]),
            Expr::list(
                KW_DEFINE,
                vec![
                    "b".into(),
                    Expr::list(
                        KW_BLOCK,
                        vec![Expr::list(
                            "foo",
                            vec![Expr::list(KW_ARRAY, vec!["a".into(), 2.0.into()])]
                        )]
                    )
                ]
            )
        ])
    )
}
