use logos::{Logos, SpannedIter};

#[derive(Default,Debug,Clone,PartialEq)]
pub struct LexerError
{
  
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexerError)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token<'input> {
    #[regex(r"[0-9]+")]
    DefaultNumber(&'input str),
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral(&'input str),
}

pub struct KaisaLexer<'input>
{
  inner: SpannedIter<'input,Token<'input>>,
  errors: Vec<LexerError>
}

impl<'input> KaisaLexer<'input>
{
    pub fn new(input: &'input str) -> Self
    {
	Self
	{
	    inner: Token::lexer(input).spanned(),
	    errors: Vec::new(),
	}
    }
}
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for KaisaLexer<'input>
{
    type Item = Spanned<Token<'input>,usize,LexerError>;

    fn next(&mut self) -> Option<Self::Item>
    {
	match self.inner.next()
	{
	    Some((token_res,pos)) => {
		match token_res
		{
		    Ok(token) => todo!(),
		    Err(err) => todo!(),
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
	let input = "1234";
	let mut lexer = Token::lexer(input);

	assert_eq!(lexer.next(),Some(Ok(Token::DefaultNumber("1234"))))
    }

    #[test]
    fn test_string_literal()
    {
	let input = "\"Hello World!\"";
	let mut lexer = Token::lexer(input);

	assert_eq!(lexer.next(),Some(Ok(Token::StringLiteral("\"Hello World!\""))))
    }

    #[test]
    fn test_error()
    {
	let input = "h";
	let mut lexer = Token::lexer(input);

	assert_eq!(lexer.next(),Some(Err(LexerError::default())))
    }
}
