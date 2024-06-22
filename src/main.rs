mod lexer;
mod parser;
mod transpiler;

fn main() {
    let json_string = r#"
        {"name":{"first_name":"Armaan","last_name":"Gupta","employement":{"role":"software engineer","test_number":50000,"test_boolean":false,"test_null":null,"please work":{}}}}
    "#.to_string();

    let mut l = lexer::Lexer::new(json_string);
    l.lex();
    let mut p = parser::Parser::new(&l.tokens);
    p.parse();
    if let Some(root_object) = p.root_object {
        let t = transpiler::Transpiler::new(&root_object);
        t.format();
    }
}
