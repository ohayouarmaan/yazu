use std::{fs,env};

mod lexer;
mod parser;
mod transpiler;

fn main() {
    let arguments = env::args().collect::<Vec<String>>();
    let first_argument = arguments.get(1);
    if let Some(file_name) = first_argument {
        let json_string = fs::read_to_string(file_name).expect("[Error]: Can not open specified file.");
        let mut l = lexer::Lexer::new(json_string);
        l.lex();
        let mut p = parser::Parser::new(&l.tokens);
        p.parse();
        if let Some(root_object) = p.root_object {
            let t = transpiler::Transpiler::new(&root_object);
            t.format();
        }
    } else {
        println!("No file provided");
    }
}
