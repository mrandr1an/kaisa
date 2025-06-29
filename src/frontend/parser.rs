use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    kaisa
);


#[cfg(test)]
mod tests
{
    use crate::frontend::{ast::{Binop, Expr, Node, Value}, tokenizer::KaisaLexer};

    use super::{kaisa::{ValueParser,ExprParser}};

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

    #[test]
    fn parse_binary()
    {
	let input = "1 + ab";
	let res = ExprParser::new().parse(input,KaisaLexer::new(input));
	assert_eq!(res,Ok(Box::new(Expr::Binary(
	    Box::new(Expr::Val(Value::Number(Node::new(0..1,"1")))),
	    Binop::Plus(Node::new(2..3,())),
	    Box::new(Expr::Id(Node::new(4..6,"ab")))))));
    }
}
