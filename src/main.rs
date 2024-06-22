mod lexer;
mod parser;

fn main() {
    let json_string = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "courses": ["Rust", "Algorithms", "Data Structures"],
        "address": {
            "street": "123 Main St",
            "city": "Anytown",
            "zip_code": "12345"
        },
        "grades": {
            "Rust": 95,
            "Algorithms": 87,
            "Data Structures": 92
        }
    }
    "#.to_string();

    let mut l = lexer::Lexer::new(json_string);
    l.lex();
    let p = parser::Parser::new(l.tokens);
}
