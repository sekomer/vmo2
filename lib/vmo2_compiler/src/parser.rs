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
        Rule::expression_stmt => {
            let expr = parse_expression(pair.into_inner().next().unwrap());
            AstStatement::Expression(expr)
        }
        _ => unreachable!(),
    }
}

pub fn parse_expression(pair: Pair<Rule>) -> AstExpression {
    match pair.as_rule() {
        Rule::expression => parse_expression(pair.into_inner().next().unwrap()),
        Rule::identifier => AstExpression::Variable(pair.as_str().to_string()),
        Rule::string | Rule::inner => {
            AstExpression::Literal(parse_literal(pair.into_inner().next().unwrap()))
        }
        Rule::equality_expr
        | Rule::relational_expr
        | Rule::additive_expr
        | Rule::multiplicative_expr => parse_binary_expression(pair),
        Rule::unary_expr => parse_unary_expression(pair),
        _ => unreachable!(),
    }
}

pub fn parse_unary_expression(pair: Pair<Rule>) -> AstExpression {
    match pair.as_rule() {
        Rule::unary_expr => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();

            // if it's a primary_expr, just parse it directly
            if first.as_rule() == Rule::primary_expr {
                return parse_primary_expression(first);
            }

            // otherwise it's a unary operation
            let operator = first.as_str().to_string();
            let expr = parse_primary_expression(inner.next().unwrap());
            AstExpression::UnaryOperation(operator, Box::new(expr))
        }
        Rule::primary_expr => parse_primary_expression(pair),
        _ => unreachable!(),
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

pub fn parse_arguments(pair: Pair<Rule>) -> Vec<AstExpression> {
    match pair.as_rule() {
        Rule::arguments => {
            let mut args = Vec::new();
            let mut inner = pair.into_inner();

            while let Some(arg) = inner.next() {
                args.push(parse_expression(arg));
            }

            args
        }
        _ => unreachable!(),
    }
}

pub fn parse_primary_expression(pair: Pair<Rule>) -> AstExpression {
    match pair.as_rule() {
        Rule::identifier => AstExpression::Variable(pair.as_str().to_string()),
        Rule::literal => AstExpression::Literal(parse_literal(pair.into_inner().next().unwrap())),
        Rule::function_call => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let args = if let Some(args_pair) = inner.next() {
                parse_arguments(args_pair)
            } else {
                Vec::new()
            };
            AstExpression::FunctionCall(name, args)
        }
        Rule::primary_expr => parse_primary_expression(pair.into_inner().next().unwrap()),
        _ => unreachable!(),
    }
}

pub fn parse_literal(pair: Pair<Rule>) -> AstLiteral {
    match pair.as_rule() {
        Rule::number => AstLiteral::UInt(pair.as_str().parse().unwrap()),
        Rule::bool => AstLiteral::Bool(pair.as_str().parse().unwrap()),
        Rule::null => AstLiteral::Null,
        Rule::string => {
            let inner = pair.into_inner().next().unwrap();
            AstLiteral::String(inner.as_str().to_string())
        }
        Rule::inner => AstLiteral::String(pair.as_str().to_string()),
        _ => unreachable!(),
    }
}
