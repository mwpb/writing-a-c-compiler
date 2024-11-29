use anyhow::{anyhow, Context};
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Keyword {
    INT,
    VOID,
    RETURN,
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Identifier(&'a str),
    Constant(i32),
    Keyword(Keyword),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

struct TokenExtractor {
    regex: Regex,
    token_from_text: fn(&str) -> anyhow::Result<Token>,
}

struct Programme<'a> {
    text: &'a str,
    tokens: Vec<Token<'a>>,
}
fn token_extractors() -> anyhow::Result<Vec<TokenExtractor>> {
    let extractors = vec![
        TokenExtractor {
            regex: Regex::new(r"^\(")?,
            token_from_text: |_| Ok(Token::OpenParen),
        },
        TokenExtractor {
            regex: Regex::new(r"^\)")?,
            token_from_text: |_| Ok(Token::CloseParen),
        },
        TokenExtractor {
            regex: Regex::new(r"^\{")?,
            token_from_text: |_| Ok(Token::OpenBrace),
        },
        TokenExtractor {
            regex: Regex::new(r"^\}")?,
            token_from_text: |_| Ok(Token::CloseBrace),
        },
        TokenExtractor {
            regex: Regex::new(r"^;")?,
            token_from_text: |_| Ok(Token::Semicolon),
        },
        TokenExtractor {
            regex: Regex::new(r"^[0-9]+\b")?,
            token_from_text: |x| {
                x.parse()
                    .map(Token::Constant)
                    .with_context(|| "Failed to extract integer.")
            },
        },
        TokenExtractor {
            regex: Regex::new(r"^int\b")?,
            token_from_text: |_| Ok(Token::Keyword(Keyword::INT)),
        },
        TokenExtractor {
            regex: Regex::new(r"^void\b")?,
            token_from_text: |_| Ok(Token::Keyword(Keyword::VOID)),
        },
        TokenExtractor {
            regex: Regex::new(r"^return\b")?,
            token_from_text: |_| Ok(Token::Keyword(Keyword::RETURN)),
        },
        TokenExtractor {
            regex: Regex::new(r"^[a-zA-Z_]+\b")?,
            token_from_text: |x| Ok(Token::Identifier(x)),
        },
    ];
    Ok(extractors)
}

fn extract_token(extractor: TokenExtractor, programme: &mut Programme) -> anyhow::Result<bool> {
    let Some(found) = extractor.regex.find(programme.text) else {
        return Ok(false);
    };

    let token = (extractor.token_from_text)(found.as_str())?;
    programme.tokens.push(token);
    programme.text = &programme.text[found.range().end..];

    Ok(true)
}

fn tokenize_programme<'a>(programme: &'a mut Programme) -> anyhow::Result<()> {
    programme.text = programme.text.trim();
    if programme.text.is_empty() {
        return Ok(());
    }

    let extractors = token_extractors()?;
    for extractor in extractors {
        if extract_token(extractor, programme)? {
            return tokenize_programme(programme);
        }
    }

    Err(anyhow!("Token not recognised.",))
}

fn tokenize(text: &str) -> anyhow::Result<Vec<Token>> {
    let mut programme = Programme {
        text,
        tokens: vec![],
    };
    tokenize_programme(&mut programme)?;
    Ok(programme.tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let raw_text = "int main()";
        let tokens = tokenize(raw_text).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::INT),
                Token::Identifier("main"),
                Token::OpenParen,
                Token::CloseParen
            ]
        )
    }
}
