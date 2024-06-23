mod lexer;
mod parser;
mod transpiler;

fn main() {
    let json_string = r#"
    {
        "name": {
            "first_name": "Armaan",
            "last_name": "Gupta",
            "employement": {
                "test_number": 50000,
                "role": "software engineer",
                "test_boolean": false,
                "please work": {},
                "test_null": null,
                "test_array": [
                    "wow123",
                    { "a": "b" },
                    75_00_000
                ]
            }
        }
    }
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
