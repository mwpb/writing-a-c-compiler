use anyhow::anyhow;

use crate::tokens::{Keyword, Token};

#[derive(Debug)]
enum Identifier {
    Var(String),
}
enum Expression {
    Constant(i32),
}
enum Statement {
    Return(Expression),
}
struct Function {
    name: Identifier,
    body: Statement,
}
pub struct Programme {
    function: Function,
}

pub trait Node {
    fn parse<'a>(tokens: &'a [Token]) -> anyhow::Result<(Self, &'a [Token<'a>])>
    where
        Self: Sized;
    fn pretty_print<'a>(&self) -> String;
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

impl Node for Expression {
    fn parse<'a>(tokens: &'a [Token]) -> anyhow::Result<(Self, &'a [Token<'a>])>
    where
        Self: Sized,
    {
        match tokens {
            [Token::Constant(a), tokens @ ..] => Ok((Expression::Constant(*a), tokens)),
            _ => Err(anyhow!(
                "Token {:?} is not a constant. {:?}",
                tokens[0],
                tokens
            )),
        }
    }

    fn pretty_print<'a>(&self) -> String {
        match self {
            Expression::Constant(a) => format!("Constant({})", a.to_string()),
        }
    }
}

impl Node for Statement {
    fn parse<'a>(tokens: &'a [Token]) -> anyhow::Result<(Self, &'a [Token<'a>])>
    where
        Self: Sized,
    {
        let tokens = expect(&[Token::Keyword(Keyword::RETURN)], tokens)?;
        let (expression, tokens) = Expression::parse(tokens)?;
        let tokens = expect(&[Token::Semicolon], tokens)?;

        Ok((Statement::Return(expression), tokens))
    }

    fn pretty_print<'a>(&self) -> String {
        match self {
            Statement::Return(expression) => format!("Return({})", expression.pretty_print()),
        }
    }
}

impl Node for Identifier {
    fn parse<'a>(tokens: &'a [Token]) -> anyhow::Result<(Self, &'a [Token<'a>])>
    where
        Self: Sized,
    {
        match tokens {
            [Token::Identifier(name), tokens @ ..] => {
                Ok((Identifier::Var(name.to_string()), tokens))
            }
            _ => Err(anyhow!(
                "Token {:?} is not an identifier. {:?}",
                tokens[0],
                tokens
            )),
        }
    }

    fn pretty_print<'a>(&self) -> String {
        match self {
            Identifier::Var(a) => a.to_string(),
        }
    }
}

impl Node for Function {
    fn parse<'a>(tokens: &'a [Token]) -> anyhow::Result<(Self, &'a [Token<'a>])>
    where
        Self: Sized,
    {
        let tokens = expect(&[Token::Keyword(Keyword::INT)], tokens)?;
        let (name, tokens) = Identifier::parse(tokens)?;
        let tokens = expect(
            &[
                Token::OpenParen,
                Token::Keyword(Keyword::VOID),
                Token::CloseParen,
                Token::OpenBrace,
            ],
            tokens,
        )?;
        let (body, tokens) = Statement::parse(tokens)?;
        let tokens = expect(&[Token::CloseBrace], tokens)?;

        Ok((Function { name, body }, tokens))
    }

    fn pretty_print<'a>(&self) -> String {
        format!(
            "Function(
name={},
body={})",
            self.name.pretty_print(),
            self.body.pretty_print()
        )
    }
}

impl Node for Programme {
    fn parse<'a>(tokens: &'a [Token]) -> anyhow::Result<(Self, &'a [Token<'a>])>
    where
        Self: Sized,
    {
        let (function, tokens) = Function::parse(tokens)?;
        if tokens.len() > 0 {
            return Err(anyhow!(
                "Programme finished but there are remaining tokens {:?}.",
                tokens
            ));
        }

        Ok((Programme { function }, tokens))
    }

    fn pretty_print<'a>(&self) -> String {
        format!("Programme({})", self.function.pretty_print())
    }
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
            }
        };
        assert_eq!(programme.pretty_print(), "")
    }
}