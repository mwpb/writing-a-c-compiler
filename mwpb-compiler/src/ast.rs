use anyhow::anyhow;

use crate::tokens::{self, Keyword, Token};

#[derive(Debug)]
pub enum Identifier {
    Var(String),
}
#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
}
#[derive(Debug)]
pub struct Unary {
    operator: UnaryOperator,
    expression: Expression,
}
#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    Unary(Box<Unary>),
}
#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}
#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub body: Statement,
}
#[derive(Debug)]
pub struct Programme {
    pub function: Function,
}

fn expect<'a>(expected: &[Token], mut tokens: &'a [Token<'a>]) -> anyhow::Result<&'a [Token<'a>]> {
    for token in expected {
        tokens = match tokens.split_first() {
            Some((x, tokens)) if x == token => tokens,
            _ => return Err(anyhow!("Expected {:?}, got {:?}", token, tokens[0])),
        };
    }
    Ok(tokens)
}

fn parse_matching_braces<'a>(tokens: &'a [Token]) -> anyhow::Result<(Expression, &'a [Token<'a>])> {
    let tokens = expect(&[Token::OpenParen], tokens)?;
    let (expression, tokens) = parse_expression(tokens)?;
    let tokens = expect(&[Token::CloseParen], tokens)?;
    Ok((expression, tokens))
}

fn parse_unary<'a>(
    operator: UnaryOperator,
    tokens: &'a [Token],
) -> anyhow::Result<(Expression, &'a [Token<'a>])> {
    Ok((
        Expression::Unary(Box::new(Unary {
            operator,
            expression: parse_expression(tokens)?.0,
        })),
        parse_expression(tokens)?.1,
    ))
}
fn parse_expression<'a>(tokens: &'a [Token]) -> anyhow::Result<(Expression, &'a [Token<'a>])> {
    match tokens {
        [Token::Complement, tokens @ ..] => parse_unary(UnaryOperator::Complement, tokens),
        [Token::Negation, tokens @ ..] => parse_unary(UnaryOperator::Negate, tokens),
        [Token::OpenParen, ..] => parse_matching_braces(tokens),
        [Token::Constant(a), tokens @ ..] => Ok((Expression::Constant(*a), tokens)),
        _ => Err(anyhow!(
            "Token {:?} is not a constant. {:?}",
            tokens[0],
            tokens
        )),
    }
}

fn parse_statement<'a>(tokens: &'a [Token]) -> anyhow::Result<(Statement, &'a [Token<'a>])> {
    let tokens = expect(&[Token::Keyword(Keyword::RETURN)], tokens)?;
    let (expression, tokens) = parse_expression(tokens)?;
    let tokens = expect(&[Token::Semicolon], tokens)?;

    Ok((Statement::Return(expression), tokens))
}

fn parse_identifier<'a>(tokens: &'a [Token]) -> anyhow::Result<(Identifier, &'a [Token<'a>])> {
    match tokens {
        [Token::Identifier(name), tokens @ ..] => Ok((Identifier::Var(name.to_string()), tokens)),
        _ => Err(anyhow!(
            "Token {:?} is not an identifier. {:?}",
            tokens[0],
            tokens
        )),
    }
}

fn parse_function<'a>(tokens: &'a [Token]) -> anyhow::Result<(Function, &'a [Token<'a>])> {
    let tokens = expect(&[Token::Keyword(Keyword::INT)], tokens)?;
    let (name, tokens) = parse_identifier(tokens)?;
    let tokens = expect(
        &[
            Token::OpenParen,
            Token::Keyword(Keyword::VOID),
            Token::CloseParen,
            Token::OpenBrace,
        ],
        tokens,
    )?;
    let (body, tokens) = parse_statement(tokens)?;
    let tokens = expect(&[Token::CloseBrace], tokens)?;

    Ok((Function { name, body }, tokens))
}

pub fn parse_programme<'a>(tokens: &'a [Token]) -> anyhow::Result<(Programme, &'a [Token<'a>])> {
    let (function, tokens) = parse_function(tokens)?;
    if tokens.len() > 0 {
        return Err(anyhow!(
            "Programme finished but there are remaining tokens {:?}.",
            tokens
        ));
    }

    Ok((Programme { function }, tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_print() {
        let programme = Programme {
            function: Function {
                name: Identifier::Var("main".to_string()),
                body: Statement::Return(Expression::Constant(5)),
            },
        };
        assert_eq!(
            format!("{:?}", programme),
            r#"Programme { function: Function { name: Var("main"), body: Return(Constant(5)) } }"#
        )
    }
}
