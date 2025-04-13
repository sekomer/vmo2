#[cfg(test)]
mod tests {
    use crate::parser::*;
    use crate::types::*;
    use pest::Parser;

    #[test]
    fn test_parse_literal() {
        let number_pair = OxydeParser::parse(Rule::number, "42")
            .unwrap()
            .next()
            .unwrap();
        let number_literal = parse_literal(number_pair);
        assert!(matches!(number_literal, AstLiteral::UInt(42)));

        let bool_pair = OxydeParser::parse(Rule::bool, "true")
            .unwrap()
            .next()
            .unwrap();
        let bool_literal = parse_literal(bool_pair);
        assert!(matches!(bool_literal, AstLiteral::Bool(true)));

        let null_pair = OxydeParser::parse(Rule::null, "null")
            .unwrap()
            .next()
            .unwrap();
        let null_literal = parse_literal(null_pair);
        assert!(matches!(null_literal, AstLiteral::Null));

        let string_pair = OxydeParser::parse(Rule::string, r#""hello""#)
            .unwrap()
            .next()
            .unwrap();
        let string_literal = parse_literal(string_pair);
        assert_eq!(string_literal, AstLiteral::String("hello".to_string()));

        let string_expr_pair = OxydeParser::parse(Rule::string, r#""hello""#)
            .unwrap()
            .next()
            .unwrap();
        let string_expr = parse_expression(string_expr_pair);
        assert_eq!(
            string_expr,
            AstExpression::Literal(AstLiteral::String("hello".to_string()))
        );

        let inner_pair = OxydeParser::parse(Rule::string, r#""seko\"mer""#)
            .unwrap()
            .next()
            .unwrap();
        let inner_expr = parse_literal(inner_pair);
        // todo: check if its possible to parse with single escape character
        assert_eq!(inner_expr, AstLiteral::String("seko\\\"mer".to_string()));
    }

    #[test]
    fn test_parse_identifier() {
        let identifier_pair = OxydeParser::parse(Rule::identifier, "x")
            .unwrap()
            .next()
            .unwrap();
        let identifier_expr = parse_expression(identifier_pair);
        assert!(matches!(identifier_expr, AstExpression::Variable(s) if s == "x"));

        /*
         *  assignment = { identifier ~ "=" ~ expression }
         */
        let assignment_pair = OxydeParser::parse(Rule::assignment, "x = 42;")
            .unwrap()
            .next()
            .unwrap();
        let assignment_stmt = parse_statement(assignment_pair);
        assert_eq!(
            assignment_stmt,
            AstStatement::Assignment("x".to_owned(), AstExpression::Literal(AstLiteral::UInt(42)))
        );

        /*
         *  function_call = { identifier ~ "(" ~ arguments? ~ ")" }
         */
        let function_call_pair = OxydeParser::parse(Rule::function_call, "myFunc(1, 2, x);")
            .unwrap()
            .next()
            .unwrap();
        let function_call_stmt = parse_primary_expression(function_call_pair);
        assert_eq!(
            function_call_stmt,
            AstExpression::FunctionCall(
                "myFunc".to_owned(),
                vec![
                    AstExpression::Literal(AstLiteral::UInt(1)),
                    AstExpression::Literal(AstLiteral::UInt(2)),
                    AstExpression::Variable("x".to_owned()),
                ]
            )
        );
    }

    #[test]
    fn test_parse_statement() {
        let assignment_pair = OxydeParser::parse(Rule::assignment, "x = 42;")
            .unwrap()
            .next()
            .unwrap();
        let assignment_stmt = parse_statement(assignment_pair);
        match assignment_stmt {
            AstStatement::Assignment(id, expr) => {
                assert_eq!(id, "x");
                assert!(matches!(expr, AstExpression::Literal(AstLiteral::UInt(42))));
            }
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_parse_program() {
        let program_pair = OxydeParser::parse(
            Rule::program,
            r#"
            x = 42;
            y = 10;
            "#,
        )
        .unwrap()
        .next()
        .unwrap();
        let program = parse_program(program_pair);
        assert_eq!(program.statements.len(), 2);

        match &program.statements[0] {
            AstStatement::Assignment(id, expr) => {
                assert_eq!(id, "x");
                assert!(matches!(expr, AstExpression::Literal(AstLiteral::UInt(42))));
            }
            _ => panic!("Expected assignment statement"),
        }

        match &program.statements[1] {
            AstStatement::Assignment(id, expr) => {
                assert_eq!(id, "y");
                assert!(matches!(expr, AstExpression::Literal(AstLiteral::UInt(10))));
            }
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_parse_swap_a_and_b_with_temp() {
        let program_pair = OxydeParser::parse(
            Rule::program,
            r#"
            temp = a;
            a = b;
            b = temp;
            "#,
        )
        .unwrap()
        .next()
        .unwrap();

        let program = parse_program(program_pair);
        assert_eq!(program.statements.len(), 3);
        assert_eq!(
            program.statements,
            vec![
                AstStatement::Assignment(
                    String::from("temp"),
                    AstExpression::Variable(String::from("a"))
                ),
                AstStatement::Assignment(
                    String::from("a"),
                    AstExpression::Variable(String::from("b"))
                ),
                AstStatement::Assignment(
                    String::from("b"),
                    AstExpression::Variable(String::from("temp"))
                ),
            ]
        );
    }

    #[test]
    fn test_parse_while_true_statement() {
        let while_pair = OxydeParser::parse(
            Rule::while_statement,
            r#"
            while (true) { 
                x = 1; 
            } 
            "#
            .trim(),
        )
        .unwrap()
        .next()
        .unwrap();

        let while_stmt = parse_statement(while_pair);

        assert_eq!(
            while_stmt,
            AstStatement::While(
                AstExpression::Literal(AstLiteral::Bool(true)),
                vec![AstStatement::Assignment(
                    "x".to_owned(),
                    AstExpression::Literal(AstLiteral::UInt(1))
                )]
            )
        );
    }

    #[test]
    fn test_parse_equality_expr() {
        let program_pair = OxydeParser::parse(Rule::equality_expr, r#"x == 1"#)
            .unwrap()
            .next()
            .unwrap();

        let equality_expr = parse_expression(program_pair);
        assert_eq!(
            equality_expr,
            AstExpression::BinaryOperation(
                "==".to_owned(),
                Box::new(AstExpression::Variable("x".to_owned())),
                Box::new(AstExpression::Literal(AstLiteral::UInt(1)))
            )
        );
    }

    #[test]
    fn test_parse_unary_expr() {
        /*
         *  unary_expr = {
         *    primary_expr
         *    | (unary_op ~ primary_expr)
         *  }
         */
        let program_pair = OxydeParser::parse(Rule::unary_expr, r#"-y"#)
            .unwrap()
            .next()
            .unwrap();

        let unary_expr = parse_expression(program_pair);
        assert_eq!(
            unary_expr,
            AstExpression::UnaryOperation(
                "-".to_owned(),
                Box::new(AstExpression::Variable("y".to_owned()))
            )
        );

        let program_pair = OxydeParser::parse(Rule::unary_expr, r#"+x"#)
            .unwrap()
            .next()
            .unwrap();

        let unary_expr = parse_expression(program_pair);
        assert_eq!(
            unary_expr,
            AstExpression::UnaryOperation(
                "+".to_owned(),
                Box::new(AstExpression::Variable("x".to_owned()))
            )
        );

        let primary_expr_pair = OxydeParser::parse(Rule::primary_expr, r#"x"#)
            .unwrap()
            .next()
            .unwrap();

        let unary_expr = parse_unary_expression(primary_expr_pair);
        assert_eq!(unary_expr, AstExpression::Variable("x".to_owned()));
    }

    #[test]
    fn test_parse_while_with_condition_statement() {
        let program_pair = OxydeParser::parse(
            Rule::program,
            r#"
                while (x > 0) { 
                    x = x - 1; 
                } 
            "#,
        )
        .unwrap()
        .next()
        .unwrap();

        let program = parse_program(program_pair);

        assert_eq!(
            program.statements,
            vec![AstStatement::While(
                AstExpression::BinaryOperation(
                    ">".to_owned(),
                    Box::new(AstExpression::Variable("x".to_owned())),
                    Box::new(AstExpression::Literal(AstLiteral::UInt(0)))
                ),
                vec![AstStatement::Assignment(
                    "x".to_owned(),
                    AstExpression::BinaryOperation(
                        "-".to_owned(),
                        Box::new(AstExpression::Variable("x".to_owned())),
                        Box::new(AstExpression::Literal(AstLiteral::UInt(1)))
                    )
                )]
            )]
        );
    }

    #[test]
    fn test_parse_while_with_after() {
        let program_pair = OxydeParser::parse(
            Rule::program,
            r#"
            while (x > 0)
            { 
                x = x - 1;
            }
            y = 10;
            x = y;
            "#,
        )
        .unwrap()
        .next()
        .unwrap();

        let program = parse_program(program_pair);

        assert_eq!(
            program.statements,
            vec![
                AstStatement::While(
                    AstExpression::BinaryOperation(
                        ">".to_owned(),
                        Box::new(AstExpression::Variable("x".to_owned())),
                        Box::new(AstExpression::Literal(AstLiteral::UInt(0)))
                    ),
                    vec![AstStatement::Assignment(
                        "x".to_owned(),
                        AstExpression::BinaryOperation(
                            "-".to_owned(),
                            Box::new(AstExpression::Variable("x".to_owned())),
                            Box::new(AstExpression::Literal(AstLiteral::UInt(1)))
                        )
                    )]
                ),
                AstStatement::Assignment(
                    "y".to_owned(),
                    AstExpression::Literal(AstLiteral::UInt(10))
                ),
                AstStatement::Assignment("x".to_owned(), AstExpression::Variable("y".to_owned()))
            ]
        );
    }

    #[test]
    fn test_parse_function_call() {
        let program_pair = OxydeParser::parse(Rule::program, r#"myFunc(1, 2, x);"#)
            .unwrap()
            .next()
            .unwrap();

        let program = parse_program(program_pair);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            AstStatement::Expression(expr) => match expr {
                AstExpression::FunctionCall(name, args) => {
                    assert_eq!(name, "myFunc");
                    assert_eq!(args.len(), 3);
                    assert_eq!(
                        *args,
                        vec![
                            AstExpression::Literal(AstLiteral::UInt(1)),
                            AstExpression::Literal(AstLiteral::UInt(2)),
                            AstExpression::Variable("x".to_owned()),
                        ]
                    );
                }
                _ => panic!("Expected function call expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_function_call_no_args() {
        let program_pair = OxydeParser::parse(Rule::program, r#"myFunc();"#)
            .unwrap()
            .next()
            .unwrap();

        let program = parse_program(program_pair);
        assert_eq!(program.statements.len(), 1);

        match &program.statements[0] {
            AstStatement::Expression(expr) => match expr {
                AstExpression::FunctionCall(name, args) => {
                    assert_eq!(name, "myFunc");
                    assert_eq!(args.len(), 0);
                }
                _ => panic!("Expected function call expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }
}
