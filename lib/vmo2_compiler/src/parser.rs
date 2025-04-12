use crate::types::*;
use pest::iterators::Pair;

pub fn parse_program(pair: Pair<Rule>) -> AstProgram {
    AstProgram {
        statements: pair
            .into_inner()
            .filter(|p| p.as_rule() != Rule::EOI)
            .map(parse_statements)
            .flatten()
            .collect(),
    }
}

pub fn parse_statements(pair: Pair<Rule>) -> Vec<AstStatement> {
    match pair.as_rule() {
        Rule::statements => pair.into_inner().map(parse_statement).collect(),
        _ => unreachable!(),
    }
}

pub fn parse_statement(pair: Pair<Rule>) -> AstStatement {
    match pair.as_rule() {
        Rule::statement => parse_statement(pair.into_inner().next().unwrap()),
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let identifier = inner.next().unwrap().as_str().trim().to_string();
            let expr = parse_expression(inner.next().unwrap());
            AstStatement::Assignment(identifier, expr)
        }
        Rule::assignment_stmt => parse_statement(pair.into_inner().next().unwrap()),
        Rule::while_statement => {
            let mut inner = pair.into_inner();
            let condition = parse_expression(inner.next().unwrap());
            let body = inner
                .next()
                .unwrap()
                .into_inner()
                .map(parse_statement)
                .collect();
            AstStatement::While(condition, body)
        }
        // ? asagidakiler burada olmali mi?
        Rule::expression => {
            AstStatement::Expression(parse_expression(pair.into_inner().next().unwrap()))
        }
        Rule::identifier => {
            AstStatement::Expression(AstExpression::Variable(pair.as_str().trim().to_string()))
        }
        _ => unreachable!(),
    }
}

pub fn parse_expression(pair: Pair<Rule>) -> AstExpression {
    match pair.as_rule() {
        Rule::expression => parse_expression(pair.into_inner().next().unwrap()),
        Rule::literal => AstExpression::Literal(parse_literal(pair.into_inner().next().unwrap())),
        Rule::identifier => AstExpression::Variable(pair.as_str().to_string()),
        Rule::primary_expr => parse_primary_expression(pair.into_inner().next().unwrap()),
        Rule::string => AstExpression::Literal(AstLiteral::String(
            pair.into_inner().next().unwrap().as_str().to_string(),
        )),
        Rule::inner => AstExpression::Literal(AstLiteral::String(pair.as_str().to_string())),
        Rule::equality_expr
        | Rule::relational_expr
        | Rule::additive_expr
        | Rule::multiplicative_expr => parse_binary_expression(pair),
        Rule::unary_expr => parse_unary_expression(pair.into_inner().next().unwrap()),
        _ => unreachable!(),
    }
}

pub fn parse_unary_expression(pair: Pair<Rule>) -> AstExpression {
    let mut inner = pair.into_inner();
    let first = inner.clone().next().unwrap();
    if first.as_str() == "+" || first.as_str() == "-" {
        let operator = first.as_str().to_string();
        let expr = parse_primary_expression(inner.next().unwrap());
        AstExpression::UnaryOperation(operator, Box::new(expr))
    } else {
        parse_primary_expression(inner.next().unwrap())
    }
}

pub fn parse_binary_expression(pair: Pair<Rule>) -> AstExpression {
    /*
     *  parses binary and unary expressions with the following precedence:
     *    equality_expr > relational_expr > additive_expr > multiplicative_expr > unary_expr
     */

    match pair.as_rule() {
        Rule::equality_expr
        | Rule::relational_expr
        | Rule::additive_expr
        | Rule::multiplicative_expr => {
            /*
             * Parse first expr as left side, then combine with any additional
             * operator-expr pairs into binary operations, building left-to-right.
             */
            let mut inner = pair.into_inner();
            let mut left = parse_expression(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let operator = op_pair.as_str().to_string();
                let right = parse_expression(inner.next().unwrap());
                left = AstExpression::BinaryOperation(operator, Box::new(left), Box::new(right));
            }

            left
        }
        Rule::unary_expr => parse_unary_expression(pair.into_inner().next().unwrap()),
        _ => unreachable!(),
    }
}

pub fn parse_primary_expression(pair: Pair<Rule>) -> AstExpression {
    match pair.as_rule() {
        Rule::identifier => AstExpression::Variable(pair.as_str().to_string()),
        Rule::literal => AstExpression::Literal(parse_literal(pair.into_inner().next().unwrap())),
        _ => unreachable!(),
    }
}

pub fn parse_literal(pair: Pair<Rule>) -> AstLiteral {
    match pair.as_rule() {
        Rule::number => AstLiteral::UInt(pair.as_str().parse().unwrap()),
        Rule::bool => AstLiteral::Bool(pair.as_str().parse().unwrap()),
        Rule::null => AstLiteral::Null,
        _ => unreachable!(),
    }
}
