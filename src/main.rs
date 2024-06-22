use lexer::Token;

mod lexer;
mod parser;

fn main() {
    let json_string = r#"
    {
        "name": {
            "first_name": "Armaan",
            "last_name": "Gupta"
        },
        "foo": 5_00_000,
        "test": ["Armaan", 2]
    }
    "#.to_string();

    let mut l = lexer::Lexer::new(json_string);
    l.lex();
    let mut p = parser::Parser::new(&l.tokens);
    p.parse();
}
