use std::collections::HashMap;

use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    index: usize
}

#[derive(Debug)]
pub enum ObjectType {
    False,
    True,
    Null,
    Number(String),
    String(String),
    Array(Vec<Box<Object>>),
    Map(HashMap<String, Box<Object>>)
}

#[derive(Debug)]
pub struct Object {
    obj_type: ObjectType,
}

impl<'a> Parser<'a> {
    // I just know that tokens will exist (source: trust me bro)
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0
        }
    }
    
    pub fn parse_map(&mut self) -> Object {
        let mut map: HashMap<String, Box<Object>> = HashMap::new();
        while let Some(token) = self.tokens.get(self.index) {
            match token.token_type {
                TokenType::RBrace => { break; },
                TokenType::String => {
                    let key = &self.tokens[self.index];
                    self.index += 1;
                    if let Some(should_be_colon_token) = self.tokens.get(self.index) {
                        if should_be_colon_token.token_type == TokenType::Colon {
                            self.index += 1;
                            let value = self.parse_object();
                            self.index += 1;
                            map.insert(key.lexeme.clone(), Box::new(value));
                            if self.match_token(TokenType::Comma) && self.tokens[self.index + 1].token_type != TokenType::RBrace {
                                self.index += 1;
                                continue;
                            } else {
                                if self.tokens[self.index].token_type != TokenType::RBrace {
                                    panic!("Expected a '}}' got {:?}", self.tokens[self.index]);
                                } else {
                                    return Object {
                                        obj_type: ObjectType::Map(map)
                                    }
                                }
                            }
                        } else {
                            panic!("Expected a ':' got {:?}", should_be_colon_token);
                        }
                    } else {
                        panic!("No token found.");
                    }
                }
                _ => {
                    panic!("Invalid JSON Syntax expected a string got {:?}", token.token_type);
                }
            }
        }
        todo!();
    }

    pub fn parse_array(&mut self) -> Object {
        let mut elements: Vec<Box<Object>> = vec![];
        while let Some(current_token) = self.tokens.get(self.index) {
            let element = self.parse_object();
            elements.push(Box::new(element));
            self.index += 1;
            if let Some(next_token) = self.tokens.get(self.index) {
                if next_token.token_type == TokenType::RSquare {
                    break;
                } else if next_token.token_type == TokenType::Comma {
                    self.index += 1;
                    continue;
                } else {
                    panic!("Invalid Token.");
                }
            }
        }
        return Object {
            obj_type: ObjectType::Array(elements)
        }
    }

    pub fn match_token(&mut self, expected_type: TokenType) -> bool {
        if let Some(token) = self.tokens.get(self.index) {
            if token.token_type == expected_type {
                return true;
            } else {
                return false;
            }
        }
        panic!("PARSER ERROR: no token exists at index: {}", self.index);
    }

    pub fn consume(&self, expected_type: TokenType, message: &str) -> &Token {
        if let Some(token) = self.tokens.get(self.index) {
            if token.token_type == expected_type {
                return token;
            } else {
                panic!("PARSER ERROR: {message}");
            }
        }
        panic!("PARSER ERROR: no token exists at index: {}", self.index);
    }

    pub fn parse_object(&mut self) -> Object {
        if let Some(token) = self.tokens.get(self.index) {
            match token.token_type {
                TokenType::LBrace => {
                    self.index += 1;
                    return self.parse_map();
                },
                TokenType::LSquare => {
                    self.index += 1;
                    return self.parse_array();
                },
                TokenType::String => {
                    return Object {
                        obj_type: ObjectType::String(token.lexeme.clone())
                    }
                },
                TokenType::Number => {
                    return Object {
                        obj_type: ObjectType::Number(token.lexeme.clone())
                    }
                },
                TokenType::False => {
                    return Object {
                        obj_type: ObjectType::False
                    }
                },
                TokenType::True => {
                    return Object {
                        obj_type: ObjectType::True
                    }
                },
                TokenType::Null => {
                    return Object {
                        obj_type: ObjectType::Null
                    }
                },
                _ => {
                    panic!("invalid token: {:?}", token);
                }
            }
        } else {
            panic!("No token exists.")
        }
    }

    pub fn parse(&mut self) {
        if let Some(token) = self.tokens.get(self.index) {
            println!("{:?}", self.parse_object());
        }
    }
}

