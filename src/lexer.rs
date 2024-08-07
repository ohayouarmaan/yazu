#[derive(Debug,PartialEq)]
pub enum TokenType {
    LBrace,
    RBrace,
    LSquare,
    RSquare,
    Comma,
    Colon,

    // Datatypes according to the official json grammar (https://www.json.org/json-en.html)
    // Arrays and Objects should be built in the parser.
    String,
    Number,
    False,
    True,
    Null
}

#[derive(Debug,PartialEq)]
pub struct Token {
    index: usize,
    pub lexeme: String,
    pub token_type: TokenType,
    pub line: u32
}

const NUMERICS: [char; 12] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '_'];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];


pub struct Lexer {
    source_content: Vec<char>,
    pub tokens: Vec<Token>,
    index: usize,
    current_line: u32
} 

impl Lexer {
    pub fn new(source_content: String) -> Self {
        Self {
            source_content: source_content.chars().collect::<Vec<char>>(),
            tokens: vec![],
            index: 0,
            current_line: 0
        }
    }

    fn match_on_current_character(&mut self, c: &char) -> Result<(), String>{
        match c {
            '{' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "{".to_string(),
                    token_type: TokenType::LBrace,
                    line: self.current_line
                });
                self.index += 1;
                Ok(())
            },
            '}' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "}".to_string(),
                    token_type: TokenType::RBrace,
                    line: self.current_line
                });
                self.index += 1;
                Ok(())
            },
            '[' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "[".to_string(),
                    token_type: TokenType::LSquare,
                    line: self.current_line

                });
                self.index += 1;
                Ok(())
            },
            ']' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "]".to_string(),
                    token_type: TokenType::RSquare,
                    line: self.current_line
                });
                self.index += 1;
                Ok(())
            },
            '"' => {
                self.index += 1;
                let start_index = self.index;
                let built_string = self.build_string();
                self.tokens.push(Token {
                    index: start_index,
                    lexeme: built_string,
                    token_type: TokenType::String,
                    line: self.current_line

                });
                self.index += 1;
                Ok(())
            },
            ':' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: ":".to_owned(),
                    token_type: TokenType::Colon,
                    line: self.current_line

                });
                self.index += 1;
                Ok(())
            },
            ',' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: ",".to_owned(),
                    token_type: TokenType::Comma,
                    line: self.current_line
                });
                self.index += 1;
                Ok(())
            },
            c => {
                if vec![' ', '\t', '\n'].contains(c){
                    if *c == '\n' {
                        self.current_line += 1;
                    }
                    self.index += 1;
                    Ok(())
                } else if NUMBERS.contains(c) {
                    let start_index = self.index;
                    let built_number = self.build_number();
                    self.tokens.push(Token {
                        index: start_index,
                        lexeme: built_number?,
                        token_type: TokenType::Number,
                        line: self.current_line

                    });
                    Ok(())
                } else if *c == 'f' {
                    let start_index = self.index;
                    let mut built_string = String::from('f');
                    self.index += 1;
                    while let Some(&current_character) = self.source_content.get(self.index) {
                        if vec!['a', 'l', 's', 'e'].contains(&current_character) {
                            built_string.push(current_character);
                            self.index += 1;
                        } else {
                            break;
                        }
                    }
                    if built_string == "false" {
                        self.tokens.push(Token {
                            index: start_index,
                            lexeme: built_string,
                            token_type: TokenType::False,
                            line: self.current_line

                        });
                        return Ok(());
                    } else {
                        return Err(format!("Expected a false found: {}", built_string));
                    }
                } else if *c == 't' {
                    let start_index = self.index;
                    let mut built_string = String::from('t');
                    self.index += 1;
                    while let Some(&current_character) = self.source_content.get(self.index) {
                        if vec!['r', 'u', 'e'].contains(&current_character) {
                            built_string.push(current_character);
                            self.index += 1;
                        } else {
                            break;
                        }
                    }
                    if built_string == "true" {
                        self.tokens.push(Token {
                            index: start_index,
                            lexeme: built_string,
                            token_type: TokenType::True,
                            line: self.current_line

                        });
                        return Ok(());
                    } else {
                        return Err(format!("Expected true found: {}", built_string));
                    }
                } else if *c == 'n' {
                    let start_index = self.index;
                    let mut built_string = String::from('n');
                    self.index += 1;
                    while let Some(&current_character) = self.source_content.get(self.index) {
                        if vec!['u', 'l', 'l'].contains(&current_character) {
                            built_string.push(current_character);
                            self.index += 1;
                        } else {
                            break;
                        }
                    }
                    if built_string == "null" {
                        self.tokens.push(Token {
                            index: start_index,
                            lexeme: built_string,
                            token_type: TokenType::Null,
                            line: self.current_line

                        })
                    }
                    Ok(())
                } else {
                    Err(format!("Invalid character {c} at line:{}", self.current_line))
                }
            }
        }
    }

    pub fn build_number(&mut self) -> Result<String, String> {
        let mut number: String = String::new();
        let mut dot_count = 0;
        while let Some(&current_character) = self.source_content.get(self.index) {
            if NUMERICS.contains(&current_character) {
                if current_character == '.' {
                    if dot_count > 0 {
                        return Err(format!("Invalid Number: {:?}", self.index))
                    }
                    dot_count += 1;
                }
                number.push(current_character);
                self.index += 1;
            } else {
                break;
            }
        }
        return Ok(number);
    }

    pub fn build_string(&mut self) -> String {
        let mut built_string: String = String::new();
        loop {
            match self.source_content.get(self.index) {
                None => {},
                Some(x) => {
                    if *x != '"' {
                        if *x == '\\' {
                            built_string.push(*x);
                            self.index += 1;
                        }
                        built_string.push(*x);
                    } else {
                        break;
                    }
                }
            }
            self.index += 1;
        }
        return built_string;
    }

    pub fn lex(&mut self) {
        while let Some(&current_character) = self.source_content.get(self.index) {
            self.match_on_current_character(&current_character);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token, TokenType};

    #[test]
    fn test_lexer() {
        let mut l = Lexer::new("{\"foo\": { \"jane\": \"doe\" }}".to_string());
        l.lex();
        assert_eq!(l.tokens, vec![
            Token { 
                index: 0,
                lexeme: "{".to_owned(),
                token_type: TokenType::LBrace,
                line: 0

            },
            Token { 
                index: 2,
                lexeme: "foo".to_owned(),
                token_type: TokenType::String,
                line: 0
            },
            Token { 
                index: 6,
                lexeme: ":".to_owned(),
                token_type: TokenType::Colon,
                line: 0
            },
            Token { 
                index: 8,
                lexeme: "{".to_owned(),
                token_type: TokenType::LBrace,
                line: 0

            },
            Token { 
                index: 11,
                lexeme: "jane".to_owned(),
                token_type: TokenType::String,
                line: 0

            },
            Token { 
                index: 16,
                lexeme: ":".to_owned(),
                token_type: TokenType::Colon,
                line: 0

            },
            Token { 
                index: 19,
                lexeme: "doe".to_owned(),
                token_type: TokenType::String,
                line: 0

            },
            Token { 
                index: 24,
                lexeme: "}".to_owned(),
                token_type: TokenType::RBrace,
                line: 0

            },
            Token { 
                index: 25,
                lexeme: "}".to_owned(),
                token_type: TokenType::RBrace,
                line: 0
            }
        ]);
    }
}
