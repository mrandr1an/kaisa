use std::ops::Range;


#[derive(Debug,PartialEq, Eq)]
pub struct Node<T>
{
  loc: Range<usize>,
  literal: T,
}

impl<T> Node<T>
{
    pub fn new(loc: Range<usize>, lit: T) -> Self
    {
	Self
	{
	    loc,
	    literal: lit	
	}
    }
}

#[derive(Debug,PartialEq, Eq)]
pub enum Value<'a>
{
 Number(Node<&'a str>),
 Character(Node<&'a str>),
 String(Node<&'a str>),
 List(Node<Box<[Box<Expr<'a>>]>>),
 Tuple(Node<Box<[Box<Expr<'a>>]>>),
}

#[derive(Debug,PartialEq, Eq)]
pub enum Expr<'a>
{
 Id(Node<&'a str>),
 Val(Value<'a>),
 Binary(Box<Expr<'a>>,Binop,Box<Expr<'a>>),    
 Prefix(UnaryL,Box<Expr<'a>>),    
 Postfix(Box<Expr<'a>>,UnaryR),    
}

#[derive(Debug,PartialEq, Eq)]
pub enum Binop
{
 Plus(Node<()>),
 Minus(Node<()>),
 Times(Node<()>),
 Div(Node<()>),
 Access(Node<()>),
}

#[derive(Debug,PartialEq, Eq)]
pub enum UnaryL
{
 Ref(Node<()>), 
 Deref(Node<()>), 
 Minus(Node<()>), 
}

#[derive(Debug,PartialEq, Eq)]
pub enum UnaryR
{
 Excl(Node<()>), 
}
