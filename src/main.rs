use frontend::parser::{tree::{Expr, Let}, KaisaParser, Rule};
use crate::frontend::parser::tree::Value;
use pest::Parser;

mod frontend;
mod backend;


fn main()
{
    match KaisaParser::parse(Rule::assignment,"let type Animal name = name*int")
    {
	Ok(res) => println!("{:#?}",Let::parse_let(res)),
	Err(err) => println!("{:#?}",err),
    }
}
