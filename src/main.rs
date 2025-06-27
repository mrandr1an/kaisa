mod frontend;

use frontend::tokenizer;
use lalrpop_util::lalrpop_mod;
use logos::Logos;
lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    kaisa
);

fn main()
{
    let input = "123";
    let res = kaisa::ValueParser::new().parse(input);
    match res
    {
	Ok(val) => println!("{:#?}",val),
	Err(e) => println!("{:#?}",e)
    }
}
