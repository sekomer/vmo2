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
    }

    #[test]
    fn test_parse_literal_and_identifier() {
        let identifier_pair = OxydeParser::parse(Rule::identifier, "x")
            .unwrap()
            .next()
            .unwrap();
        let identifier_expr = parse_expression(identifier_pair);
        assert!(matches!(identifier_expr, AstExpression::Variable(s) if s == "x"));

        let literal_pair = OxydeParser::parse(Rule::literal, "42")
            .unwrap()
            .next()
            .unwrap();
        let literal_expr = parse_expression(literal_pair);
        assert!(matches!(
            literal_expr,
            AstExpression::Literal(AstLiteral::UInt(42))
        ));

        let null_pair = OxydeParser::parse(Rule::literal, "null")
            .unwrap()
            .next()
            .unwrap();
        let null_expr = parse_expression(null_pair);
        assert!(matches!(
            null_expr,
            AstExpression::Literal(AstLiteral::Null)
        ));

        let string_pair = OxydeParser::parse(Rule::string, r#""hello""#)
            .unwrap()
            .next()
            .unwrap();
        let string_expr = parse_expression(string_pair);
        assert_eq!(
            string_expr,
            AstExpression::Literal(AstLiteral::String("hello".to_string()))
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
        let program_pair = OxydeParser::parse(
            Rule::equality_expr,
            r#"
            x == 1
            "#
            .trim(),
        )
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
    fn test_parse_while_with_condition_statement() {
        let program_pair = OxydeParser::parse(
            Rule::program,
            r#"
                while (x > 0) { 
                    x = x - 1; 
                } 
            "#
            .trim(),
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
            "#
            .trim(),
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
}
