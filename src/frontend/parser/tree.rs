use pest::iterators::Pair;

use super::Rule;

#[derive(Debug)]
pub struct Literal<'a>(Pair<'a,Rule>);

impl<'a> PartialEq for Literal<'a>
{
    fn eq(&self, other: &Self) -> bool
    {
	self.0.as_str() == other.0.as_str()
    }
}

#[derive(Debug,PartialEq)]
pub struct Id<'a>(Literal<'a>);

#[derive(Debug,PartialEq)]
pub struct Parameter<'a>
{
    identifier: Id<'a>,
    ty: Id<'a>,
}

#[derive(Debug,PartialEq)]
pub enum Value<'a>
{
 Id(Id<'a>),
 DecimalNumber(Literal<'a>),
 HexNumber(Literal<'a>),
 BinaryNumber(Literal<'a>),
 DefaultNumber(Literal<'a>), 
 Char(Literal<'a>),
 String(Literal<'a>),
 Tuple(Box<[Expr<'a>]>),
 List(Box<[Expr<'a>]>),
}

#[derive(Debug,PartialEq)]
pub enum Expr<'a>
{
 Val(Value<'a>),
 Binary,
 UnaryLeft,	
 UnaryRight,
 Call,
}

#[derive(Debug,PartialEq)]
pub enum Let<'a>
{
    LetType
    {
	type_name: Id<'a>,
	type_params: Box<[Parameter<'a>]>,
	expr: Box<Expr<'a>>,
    },
    LetFn
    {
	fn_name: Id<'a>,
	fn_params: Box<[Parameter<'a>]>,
	expr: Box<Expr<'a>>,
    },
}
