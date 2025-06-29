use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    kaisa
);


#[cfg(test)]
mod tests
{
    use crate::frontend::{ast::{Node, Value}, tokenizer::KaisaLexer};

    use super::{kaisa::ValueParser};

    #[test]
    fn parse_value()
    {
	let input = "23";
	let res = ValueParser::new().parse(input,KaisaLexer::new(input));
	assert_eq!(res,Ok(Value::Number(Node::new(0..2,"23"))));
    }

    #[test]
    fn parse_string()
    {
	let input = "\"Hello world!\"";
	let res = ValueParser::new().parse(input,KaisaLexer::new(input));
	assert_eq!(res,Ok(Value::String(Node::new(0..14,"\"Hello world!\""))));
    }
}
