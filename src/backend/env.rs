use std::collections::HashMap;
use crate::frontend::kaisa::{object::{ty::Type, val::Value}};

pub struct SymbolTable<'a,T>
{
    table: HashMap<&'a str,T>,
    id: usize,
}

impl<'a,T> SymbolTable<'a,T>
{
    fn new(id: usize) -> Self
    {
	Self
	{
	    table: HashMap::new(),
	    id,
	}
    }
}

impl<'a> SymbolTable<'a,&'a Value<'a>>
{

}

impl<'a> SymbolTable<'a,&'a Box<Type<'a>>>
{

}

pub struct Environment<T>
{
    inner: Vec<T>,
    curr: usize,
}

impl<T> Environment<T>
{

}

impl<'a,T> Environment<SymbolTable<'a,T>>
{
    fn lookup<E>(&self,name: &'a str) -> Result<&'a T,E>
    {
	todo!()
    }

    fn insert<E>(&mut self,name: &'a str,val: T) -> Result<(),E>
    {
	match self.inner.get_mut(self.curr)
	{
	    Some(ref mut st) => {
		match st.table.insert(name,val)
		{
		    Some(_) => Ok(()),
		    None => Ok(()),
		}
	    }
	    None => todo!(),
	}
    }

    fn initializeScope(&mut self)
    {
	self.curr = self.curr + 1;
	let st: SymbolTable<T> = SymbolTable::new(self.curr);
	self.inner.push(st);
    }

    fn finalizeScope<E>(&mut self) -> Result<(),E>
    {
	self.curr = self.curr - 1;

	match self.inner.pop()
	{
	    Some(_) => Ok(()),
	    None => todo!(),
	}
    }
}
