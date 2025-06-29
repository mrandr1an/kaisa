use std::ops::Range;

use logos::{Lexer, Logos, SpannedIter};

#[derive(Default,Debug,Clone,PartialEq)]
pub struct LexerError;

#[derive(Logos,Clone, Debug, PartialEq)]
#[logos(error = LexerError)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token<'input> {
    #[regex(r"[0-9]+")]
    DefaultNumber(&'input str),
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral(&'input str),
    #[regex(r"'([^'\\]|\\.)'")]
    CharacterLiteral(&'input str),
    #[token("let")]
    Let,
    #[token("type")]
    Type,
    #[token("fn")]
    Fn,
    #[token("val")]
    Val,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[token("'")]
    SQUOTE,
    #[token("[")]
    LBRACK,
    #[token("]")]
    RBRACK,
    #[token(",")]
    COMMA,
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS, 
    #[token("*")]
    STAR, 
    #[token("/")]
    RSLASH,
    #[token("&")]
    REF, 
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'input str),
}

pub struct KaisaLexer<'input>
{
 stream: SpannedIter<'input,Token<'input>>,
}

impl<'input> KaisaLexer<'input>
{
    pub fn new(input: &'input str)  -> Self
    {
	Self
	{
	  stream: Token::lexer(input).spanned(),
	}
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for KaisaLexer<'input>
{
    type Item = Spanned<Token<'input>,usize,LexerError> ;

    fn next(&mut self) -> Option<Self::Item>
    {
	match self.stream.next()
	{
	    Some((token,range)) =>
	    {
             match token
             {
	       Ok(token) => Some(Ok((range.start,token,range.end))),
	       Err(err) => Some(Err(err)),
             }
	    },
	    None => None,
	}
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_default_number()
    {
	let input = "0111";
	let mut lexer = KaisaLexer::new(input);
	assert_eq!(lexer.next(),Some(Ok((0,Token::DefaultNumber("0111"),4))))
    }

    #[test]
    fn test_string_literal()
    {
	let input = "\"Hello World!\"";
	let mut lexer = KaisaLexer::new(input);
	assert_eq!(lexer.next(),Some(Ok((0,Token::StringLiteral("\"Hello World!\""),14))))
    }

    #[test]
    fn test_error()
    {
	let input = "?someIdent";
	let mut lexer = KaisaLexer::new(input);
	assert_eq!(lexer.next(),Some(Err(LexerError)));
	assert_eq!(lexer.next(),Some(Ok((1,Token::Identifier("someIdent"),10))))
    }
}
