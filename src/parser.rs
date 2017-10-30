include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use parser::*;

    #[test]
    fn assign_int() {
        assert!(statement("let number = 5;").is_ok());
        assert!(statement("let number=5;").is_ok());
        assert!(statement("let number =5;").is_ok());
        assert!(statement("let number= 5;").is_ok());
    }

    #[test]
    fn eval_int() {
        assert_eq!(
            expression("1").unwrap(),
            Expr::Literal(Value::Num(1.0))
        );
    }

    #[test]
    fn assign_float() {
        assert!(statement("let number = 13.2;").is_ok());
    }

    #[test]
    fn eval_float() {
        assert_eq!(
            expression("123.456").unwrap(),
            Expr::Literal(Value::Num(123.456))
        );
    }

    #[test]
    fn eval_arrays() {
        assert!(expression("foo[0]").is_ok());
        assert!(expression("foo[bar]").is_ok());

        assert!(expression("[1,2,3][0]").is_ok());
        assert!(expression("[[1,2], [foo, bar]][0][1]").is_ok());
        assert!(expression("foo[0][1]").is_ok());

        assert!(expression("[bar][0]").is_ok());
        assert!(expression("[bar][2 + baz]").is_ok());
    }

    #[test]
    fn eval_bools() {
        assert_eq!(
            expression("true").unwrap(),
            Expr::Literal(Value::Boolean(true))
        );

        assert_eq!(
            expression("false").unwrap(),
            Expr::Literal(Value::Boolean(false))
        );
    }
}
