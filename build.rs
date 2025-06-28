use std::{fs, io};
use std::fs::{read_to_string, write};
use std::path::Path;

fn read_grammars_to_string<P: AsRef<Path>>(dir: P) -> io::Result<String> {
    let mut contents = String::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.ends_with(".grammar") {
		contents.push_str(format!("\n// BEGIN {}\n",file_name).as_str());
                contents.push_str(&fs::read_to_string(&path)?);
		contents.push_str(format!("\n// END {}\n",file_name).as_str());
                contents.push('\n'); 
            }
        }
    }

    Ok(contents)
}

fn main()
{
    let lalrpop_prolog_path = "src/frontend/lalrpop/prologue";
    let lalrpop_grammars_path = "src/frontend/lalrpop/grammars/";
    let lalrpop_extern = "src/frontend/lalrpop/extern/logos.tokens";

    if let Ok(mut prologue) = read_to_string(lalrpop_prolog_path)
        &&
       let Ok(external) = read_to_string(lalrpop_extern)
	&&
       let Ok(grammars) = read_grammars_to_string(lalrpop_grammars_path)
	
    {
	prologue.push('\n');
	prologue.push_str(external.as_str());
	prologue.push_str(grammars.as_str());
	let out_path = Path::new("src/kaisa.lalrpop");
        write(out_path,&prologue).unwrap_or_else(|e| panic!("Failed to write kaisa grammar: {}",e));
	lalrpop::process_root().unwrap();
    }
}
