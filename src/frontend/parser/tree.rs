use pest::{iterators::{Pair, Pairs}, pratt_parser::{Assoc, Op, PrattParser}};

use super::Rule;

pub type Data<'a> = Pair<'a,Rule>;


#[derive(Debug,PartialEq)]
pub enum Let<'a>
{
  LetFn(Data<'a>,Box<[Data<'a>]>,Box<Expr<'a>>),
  LetVal(Data<'a>,Box<Expr<'a>>),
  LetType(Data<'a>,Box<[Data<'a>]>,Box<Expr<'a>>), 
  LetIn(Box<Let<'a>>,Box<Expr<'a>>),
}

impl<'a> Let<'a>
{
    pub fn parse_let(mut pairs: Pairs<'a,Rule>) -> Box<Let<'a>>
    {
	match pairs.next()
	{
	    Some(pair) => {
		match pair.as_rule()
		{
		    Rule::let_type =>  {
			let mut inner_pair = pair.into_inner();
			let type_name = inner_pair.next().unwrap();
			let mut type_args: Vec<Data<'a>> = Vec::new();
			loop
			{
			    let curr = inner_pair.next().unwrap();
			    match curr.as_rule()
			    {
				Rule::id => type_args.push(curr),
				Rule::operation => {
				    let expr = Expr::parse_expr(curr.into_inner());
				    return Box::new(Let::LetType(type_name,type_args.into_boxed_slice(),expr))
				},
				_ => unreachable!(),
			    };
			}
		    },
		    Rule::let_function => {
			let mut inner_pair = pair.into_inner();
			let fn_name = inner_pair.next().unwrap();
			let mut fn_args: Vec<Data<'a>> = Vec::new();
			loop
			{
			    let curr = inner_pair.next().unwrap();
			    match curr.as_rule()
			    {
				Rule::id => fn_args.push(curr),
				Rule::operation => {
				    let expr = Expr::parse_expr(curr.into_inner());
				    return Box::new(Let::LetFn(fn_name,fn_args.into_boxed_slice(),expr))
				},
				_ => unreachable!(),
			    };
			}
		    },
		    Rule::let_variable => todo!(),
		    Rule::assignment => Self::parse_let(pair.into_inner()),
		    r => todo!("{:#?}",r),
		}
	    },
	    None => unreachable!(),
	}
    }
}

#[derive(Debug,PartialEq)]
pub enum Expr<'a>
{
    Val(Value<'a>),
    Bin(Box<Expr<'a>>,Infix,Box<Expr<'a>>),
    UnaryL(Prefix,Box<Expr<'a>>),
    UnaryR(Box<Expr<'a>>,Postfix),
}

impl<'a> Expr<'a>
{ 
    pub fn parse_expr(pairs: Pairs<'a,Rule>) -> Box<Expr<'a>>
    {
	let pratt =
	    PrattParser::new()
	    .op(Op::infix(Rule::plus,Assoc::Left) | Op::infix(Rule::minus,Assoc::Left))
	    .op(Op::infix(Rule::times,Assoc::Left) | Op::infix(Rule::div,Assoc::Left))
	    .op(Op::infix(Rule::pow,Assoc::Right)) 
	    .op(Op::prefix(Rule::neg)) 
	    .op(Op::postfix(Rule::fact))
	    .op(Op::prefix(Rule::reference)) 
	    .op(Op::prefix(Rule::deref)) 
	    .op(Op::infix(Rule::access,Assoc::Left));
	Self::parse(pairs,&pratt)
    }

    fn parse(pairs: Pairs<'a,Rule>,pratt: &PrattParser<Rule>) -> Box<Expr<'a>>
    {
	pratt
	    .map_primary(|primary|{
		match primary.as_rule()
		{
		    Rule::operation => {
			Self::parse(primary.into_inner(),pratt)
		    },
		    Rule::primary => {
			Self::parse(primary.into_inner(),pratt)
		    },
		    Rule::expr => {
			Self::parse(primary.into_inner(),pratt)
		    },
		    _val => {
			Box::new(Expr::Val(Value::parse_val(primary)))
		    },
		}
	    })
	    .map_prefix(|op,rhs| {
		match op.as_rule() {
		    Rule::neg => Box::new(Expr::UnaryL(Prefix::Neg,rhs)),
		    Rule::reference => Box::new(Expr::UnaryL(Prefix::Ref,rhs)),
		    Rule::deref => Box::new(Expr::UnaryL(Prefix::Deref,rhs)),
		    _ => unreachable!(),
		}
	    })
	    .map_postfix(|lhs,op| {

		match op.as_rule()
		{
		    Rule::fact => Box::new(Expr::UnaryR(lhs,Postfix::Fact)),
		    _ => unreachable!(),
		}
	    })
	    .map_infix(|lhs,op,rhs| {	
		match op.as_rule()
		{
		    Rule::plus => Box::new(Expr::Bin(lhs,Infix::Plus,rhs)),
		    Rule::minus => Box::new(Expr::Bin(lhs,Infix::Minus,rhs)),
		    Rule::times => Box::new(Expr::Bin(lhs,Infix::Times,rhs)),
		    Rule::div => Box::new(Expr::Bin(lhs,Infix::Div,rhs)),
		    Rule::pow => Box::new(Expr::Bin(lhs,Infix::Power,rhs)),
		    Rule::access => Box::new(Expr::Bin(lhs,Infix::Access,rhs)),
		    _ => unreachable!(),
		}
	    })
	    .parse(pairs)
    }
}

#[derive(Debug,PartialEq)]
pub enum Prefix
{
    Ref,
    Deref,
    Neg,
}

#[derive(Debug,PartialEq)]
pub enum Postfix
{
    Fact,
}

#[derive(Debug,PartialEq)]
pub enum Infix
{
    Plus,
    Minus,
    Times,
    Div,
    Power,
    Access, 
}

#[derive(Debug,PartialEq)]
pub enum Value<'a>
{
    Id(Data<'a>),
    Imm(Immediate<'a>),
}

impl<'a> Value<'a>
{
    pub fn parse_val(pair: Data<'a>) -> Value<'a>
    {
	match pair.as_rule()
	{
	    Rule::binary_number => Value::Imm(Immediate::BinaryNumber(pair)),
	    Rule::hex_number => Value::Imm(Immediate::HexNumber(pair)),
	    Rule::decimal_number => Value::Imm(Immediate::DecimalNumber(pair)),
	    Rule::default_number => Value::Imm(Immediate::DefaultNumber(pair)),
	    Rule::id => Value::Id(pair),
	    Rule::character => Value::Imm(Immediate::Char(pair)),
	    Rule::value => {
		Self::parse_val(pair.into_inner().next().unwrap())
	    },
	    Rule::tuple => {
		let mut tuple: Vec<Box<Expr>> = Vec::new();
		for pair in pair.into_inner()
		{
		    tuple.push(Expr::parse_expr(pair.into_inner()));
		}
		Value::Imm(Immediate::Tuple(tuple.into_boxed_slice()))
	    },
	    r => unimplemented!("{:#?} not implemented yet",r)
	}
    }
}

#[derive(Debug,PartialEq)]
pub enum Immediate<'a>
{
    DefaultNumber(Data<'a>),
    DecimalNumber(Data<'a>),
    BinaryNumber(Data<'a>),
    HexNumber(Data<'a>),
    Char(Data<'a>),
    Tuple(Box<[Box<Expr<'a>>]>),
    List(Data<'a>),
}
