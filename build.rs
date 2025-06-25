use std::env;
use std::fs::{read_to_string,write};
use std::path::Path;

fn main()
{
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let grammar_files = [
	"src/frontend/grammars/value.grammar",
	"src/frontend/grammars/expr.grammar",
    ];

    let mut kaisa_grammar = String::new();

    for file in grammar_files.iter()
    {
	let file_path = Path::new(&manifest_dir).join(file);
	let content = read_to_string(&file_path)
	    .unwrap_or_else(|e| panic!("Failed to read grammar file: {}, {e}", file));
	kaisa_grammar.push_str(&format!("// BEGIN {}\n",file));
	kaisa_grammar.push_str(&content);
	kaisa_grammar.push_str("\n");
	kaisa_grammar.push_str(&format!("// END {}\n",file));
    }

    let out_path = Path::new(&manifest_dir).join("src/kaisa.lalrpop");
    write(out_path,&kaisa_grammar).unwrap_or_else(|e| panic!("Failed to write kaisa grammar: {}",e));

    lalrpop::process_root().unwrap();
}
