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
    fn test_parse_expression() {
        let var_pair = OxydeParser::parse(Rule::variable, "x")
            .unwrap()
            .next()
            .unwrap();
        let var_expr = parse_expression(var_pair);
        assert!(matches!(var_expr, AstExpression::Variable(s) if s == "x"));

        let literal_pair = OxydeParser::parse(Rule::literal, "42")
            .unwrap()
            .next()
            .unwrap();
        let literal_expr = parse_expression(literal_pair);
        assert!(matches!(
            literal_expr,
            AstExpression::Literal(AstLiteral::UInt(42))
        ));

        let bin_op_pair = OxydeParser::parse(Rule::binary_operation, "x + y")
            .unwrap()
            .next()
            .unwrap();
        let bin_op_expr = parse_expression(bin_op_pair);
        match bin_op_expr {
            AstExpression::BinaryOperation(op, left, right) => {
                assert_eq!(op, "+");
                assert!(matches!(*left, AstExpression::Variable(s) if s == "x"));
                assert!(matches!(*right, AstExpression::Variable(s) if s == "y"));
            }
            _ => panic!("Expected binary operation"),
        }
    }

    #[test]
    fn test_parse_statement() {
        let assignment_pair = OxydeParser::parse(Rule::assignment, "x = 42")
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

        let expr_pair = OxydeParser::parse(Rule::expression, "42")
            .unwrap()
            .next()
            .unwrap();
        let expr_stmt = parse_statement(expr_pair);
        assert!(matches!(
            expr_stmt,
            AstStatement::Expression(AstExpression::Literal(AstLiteral::UInt(42)))
        ));
    }

    #[test]
    fn test_parse_program() {
        let program_pair = OxydeParser::parse(Rule::program, "x = 42; y = 10;")
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
}
