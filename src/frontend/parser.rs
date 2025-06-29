use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    kaisa
);


#[cfg(test)]
mod tests
{
    use crate::frontend::{ast::{Binop, Expr, Node, UnaryL, UnaryR, Value}, tokenizer::KaisaLexer};

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

    #[test]
    fn parse_unary()
    {
	let input = "-1";
	let res = ExprParser::new().parse(input,KaisaLexer::new(input));
	assert_eq!(res,Ok(Box::new(Expr::Prefix
				   (UnaryL::Minus(Node::new(0..1,())),
				    Box::new(Expr::Val(Value::Number(Node::new(1..2,"1"))))))));
	let input = "- -someId";

	let res = ExprParser::new().parse(input,KaisaLexer::new(input));
	assert_eq!(res,Ok(Box::new(Expr::Prefix
				   (UnaryL::Minus(Node::new(0..1,())),
				    Box::new(Expr::Prefix
					     (UnaryL::Minus(Node::new(2..3,())),Box::new(Expr::Id(Node::new(3..9,"someId")))))))));
    }

    #[test]
    fn parse_call()
    {
	let input = "sayHello(\"Hello World!\")";
	let res = ExprParser::new().parse(input,KaisaLexer::new(input));
	assert_eq!(res,Ok(Box::new(Expr::Call
				   (Node::new(0..8,"sayHello"),
				    Node::new(8..24,
					      vec![Box::new(Expr::Val(Value::String(Node::new(9..23,"\"Hello World!\""))))].into_boxed_slice())))))
    }

    #[test]
    fn parse_fact()
    {
	let input = "x!";
	let res = ExprParser::new().parse(input,KaisaLexer::new(input));
	assert_eq!(res, Ok(Box::new(Expr::Postfix(Box::new(Expr::Id(Node::new(0..1,"x"))),UnaryR::Excl(Node::new(1..2,()))))))
    }

    #[test]
    fn parenthesied_expr()
    {
	let normal = "x + 1";
	let paren = "*(x + 1)";

	let res1 = ExprParser::new().parse(normal,KaisaLexer::new(normal));
	let res2 = ExprParser::new().parse(paren,KaisaLexer::new(paren));

	assert_eq!(res1,res2);
    }
}
