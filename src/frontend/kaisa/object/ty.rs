#[derive(PartialEq,Debug)]
pub enum Type<'a>
{
 Number(Sign,Size),
 Char,
 List(Box<Type<'a>>),
 Tuple(Box<[Box<Type<'a>>]>,usize),
 Sum(Box<[(&'a str,Box<Type<'a>>)]>,usize),
 Aggregate(Box<[(&'a str,Box<Type<'a>>)]>,usize),
 Fn(&'a str,Box<[(&'a str,Box<Type<'a>>)]>,Box<Type<'a>>),
 Unit,
 Type,
}

#[derive(PartialEq,Debug)]
pub enum Sign
{
    Signed,
    Unsigned
}

#[derive(PartialEq,Debug)]
pub enum Size
{
    Byte,
    Word,
    DWord,
    QWord,
}
