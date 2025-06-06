pub struct Temp<'a>
{
  name: &'a str,
  id: usize,
}

pub enum Value
{

}

pub enum Instruction<'a>
{
  Mov(Temp<'a>), 
}

pub enum ExitNode
{
    If,
    Call,
    Return,
}

pub struct BasicBlock<'a>
{
    inner: Vec<Instruction<'a>>,
    name: Option<&'a str>,
    id: usize,
}

