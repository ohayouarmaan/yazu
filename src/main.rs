use lexer::Token;

mod lexer;
mod parser;

fn main() {
    let json_string = r#"
    {
        "name": "John Doe",
        "foo": "bar"
    }
    "#.to_string();

    let mut l = lexer::Lexer::new(json_string);
    l.lex();
    let mut p = parser::Parser::new(&l.tokens);
    p.parse();
}
