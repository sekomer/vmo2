use crate::types::*;
use pest::iterators::Pair;

pub fn parse_program(pair: Pair<Rule>) -> AstProgram {
    AstProgram {
        statements: pair
            .into_inner()
            .filter(|p| p.as_rule() != Rule::EOI)
            .map(parse_statement)
            .collect(),
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
        Rule::expression => AstStatement::Expression(parse_expression(pair)),
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
        Rule::variable => AstExpression::Variable(pair.as_str().to_string()),
        Rule::condition => parse_expression(pair.into_inner().next().unwrap()),
        Rule::binary_operation => {
            let mut inner = pair.into_inner();
            let left = Box::new(parse_expression(inner.next().unwrap()));
            let op = inner.next().unwrap().as_str().to_string();
            let right = Box::new(parse_expression(inner.next().unwrap()));

            AstExpression::BinaryOperation(op, left, right)
        }
        Rule::left => parse_expression(pair.into_inner().next().unwrap()),
        Rule::right => parse_expression(pair.into_inner().next().unwrap()),
        Rule::identifier => AstExpression::Variable(pair.as_str().to_string()),
        _ => unreachable!(),
    }
}

pub fn parse_literal(pair: Pair<Rule>) -> AstLiteral {
    match pair.as_rule() {
        Rule::number => AstLiteral::UInt(pair.as_str().parse().unwrap()),
        Rule::string => AstLiteral::String(pair.as_str().to_string()),
        Rule::bool => AstLiteral::Bool(pair.as_str().parse().unwrap()),
        Rule::null => AstLiteral::Null,
        _ => unreachable!(),
    }
}
