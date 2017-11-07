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

    #[test]
    fn typecast() {
        assert!(expression("5 as string").is_ok());
        assert!(expression("5 + 5 as string").is_ok());
        assert!(expression("5 as string + 5").is_ok());
        assert!(expression("ARGV[1] as num").is_ok());
        assert!(expression("![true][0] as string").is_ok());
    }

    #[test]
    fn declare_function() {
        assert!(statement("let print_one = func() { println 1; };").is_ok());
        assert!(statement("let print_num = func(n: num) { println n; };").is_ok());
        assert!(statement(r#"let print_num_and_str = func(n: num, s: string) { println n, " ", s; };"#).is_ok());
    }

    #[test]
    fn func_as_expr() {
        assert!(expression("func() { println 1; }").is_ok());
    }

    #[test]
    fn call_function() {
        assert!(expression("some_func(1)").is_ok());
        assert!(expression("some_func(1, 2)").is_ok());
        assert!(expression("some_func(1 + 2, 2)").is_ok());
        assert!(expression("3 + square(5) * 3").is_ok());
        assert!(ast("let foo = func() { };\nprintln foo();").is_ok());
    }

    #[test]
    fn call_func_in_array() {
        assert!(expression("some_array[1](3)").is_ok());
    }

    #[test]
    fn call_func_from_func() {
        assert!(expression("returns_func()(3)").is_ok());
        assert!(expression("returns_func(5)()").is_ok());
    }

    #[test]
    fn return_statement() {
        assert!(statement("return 1;").is_ok());
    }

    #[test]
    fn get_length() {
        assert!(length(r#"length("test" as array)"#).is_ok());
    }

    #[test]
    fn string_builtins() {
        assert!(to_upper(r#"to_upper("foo")"#).is_ok());
        assert!(to_lower(r#"to_lower("bar")"#).is_ok());
    }

    #[test]
    fn mutate_var() {
        assert!(statement("foo = 1;").is_ok());
        assert!(statement("foo *= 2;").is_ok());
        assert!(statement("foo /= 3;").is_ok());
        assert!(statement("foo += 4;").is_ok());
        assert!(statement("foo -= 5;").is_ok());
        assert!(statement("foo %= 6;").is_ok());
        assert!(statement("foo **= 7;").is_ok());
    }

    #[test]
    fn type_of() {
        assert!(expression("typeof(5)").is_ok());
        assert!(expression("typeof(\"bar\")").is_ok());
        assert!(statement(r#"let var = 3 as typeof("foobar");"#).is_ok());
    }
}
