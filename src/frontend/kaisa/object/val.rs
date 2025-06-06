use super::tuple::Tuple;

#[derive(PartialEq,Debug)]
pub enum Value<'a>
{
    Unsigned(usize,Size),
    Signed(isize,Size),
    Char(char),
    Tuple(Tuple<'a>),
    Unit,
}

#[derive(PartialEq,Debug)]
pub enum Size
{
    Byte,
    Word,
    DWord,
    QWord,
}
