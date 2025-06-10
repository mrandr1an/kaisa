use pest::{iterators::Pair, pratt_parser::PrattParser, RuleType};

pub mod tree;

use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "kaisa.pest"]
pub struct KaisaParser;


