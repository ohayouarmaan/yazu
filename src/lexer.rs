#[derive(Debug)]
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


#[derive(Debug)]
pub struct Token {
    index: usize,
    lexeme: String,
    token_type: TokenType,
}

const NUMERICS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];


pub struct Lexer {
    source_content: Vec<char>,
    pub tokens: Vec<Token>,
    index: usize
} 

impl Lexer {
    pub fn new(source_content: String) -> Self {
        Self {
            source_content: source_content.chars().collect::<Vec<char>>(),
            tokens: vec![],
                index: 0
        }
    }

    fn match_on_current_character(&mut self, c: &char) {
        match c {
            '{' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "{".to_string(),
                    token_type: TokenType::LBrace
                });
                self.index += 1;
            },
            '}' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "}".to_string(),
                    token_type: TokenType::RBrace
                });
                self.index += 1;
            },
            '[' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "[".to_string(),
                    token_type: TokenType::LSquare
                });
                self.index += 1;
            },
            ']' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: "]".to_string(),
                    token_type: TokenType::RSquare
                });
                self.index += 1;
            },
            '"' => {
                self.index += 1;
                let built_string = self.build_string();
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: built_string,
                    token_type: TokenType::String
                });
                self.index += 1;
            },
            ':' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: ":".to_owned(),
                    token_type: TokenType::Colon
                });
                self.index += 1;
            },
            ',' => {
                self.tokens.push(Token {
                    index: self.index,
                    lexeme: ",".to_owned(),
                    token_type: TokenType::Comma
                });
                self.index += 1;
            },
            c => {
                if vec![' ', '\t', '\n'].contains(c){
                    self.index += 1;
                } else if NUMERICS.contains(c) {
                    let start_index = self.index;
                    let built_number = self.build_number();
                    self.tokens.push(Token {
                        index: start_index,
                        lexeme: built_number,
                        token_type: TokenType::Number
                    })
                } else if *c == 'f' {
                    let start_index = self.index;
                    let mut built_string = String::new();
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
                            token_type: TokenType::False
                        })
                    }
                } else if *c == 't' {
                    let start_index = self.index;
                    let mut built_string = String::new();
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
                            token_type: TokenType::True
                        })
                    }
                } else if *c == 'n' {
                    let start_index = self.index;
                    let mut built_string = String::new();
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
                            token_type: TokenType::Null
                        })
                    }
                }
            }
        }
    }

    pub fn build_number(&mut self) -> String {
        let mut number: String = String::new();
        while let Some(&current_character) = self.source_content.get(self.index) {
            if NUMERICS.contains(&current_character) {
                number.push(current_character);
                self.index += 1;
            } else {
                break;
            }
        }
        return number;
    }

    pub fn build_string(&mut self) -> String {
        let mut built_string: String = String::new();
        loop {
            match self.source_content.get(self.index) {
                None => {},
                Some(x) => {
                    if *x != '"' {
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
