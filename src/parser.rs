use indexmap::IndexMap;
use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
    pub root_object: Option<Object>
}

#[derive(Debug)]
pub enum ObjectType {
    False,
    True,
    Null,
    Number(String),
    String(String),
    Array(Vec<Box<Object>>),
    Map(IndexMap<String, Box<Object>>)
}

#[derive(Debug)]
pub struct Object {
    pub obj_type: ObjectType,
}

impl<'a> Parser<'a> {
    // I just know that tokens will exist (source: trust me bro)
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            root_object: None
        }
    }

    pub fn parse_map(&mut self) -> Result<Object, String> {
        let mut map: IndexMap<String, Box<Object>> = IndexMap::new();
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
                            map.insert(key.lexeme.clone(), Box::new(value?));
                            if self.match_token(TokenType::Comma) && self.tokens[self.index + 1].token_type != TokenType::RBrace {
                                self.index += 1;
                                continue;
                            } else {
                                if self.tokens[self.index].token_type != TokenType::RBrace {
                                    return Err(format!("Expected a '}}' got {:?} line:{}", self.tokens[self.index], self.tokens[self.index].line));
                                } else {
                                    return Ok(Object {
                                        obj_type: ObjectType::Map(map)
                                    });
                                }
                            }
                        } else {
                            return Err(format!("Expected a ':' got {:?} line:{}", should_be_colon_token, should_be_colon_token.line));
                        }
                    } else {
                        return Err(format!("No token found."));
                    }
                }
                _ => {
                    return Err(format!("Invalid JSON Syntax expected a string got {:?}", token.token_type));
                }
            }
        }
        return Ok(Object{
            obj_type: ObjectType::Map(map)
        });
    }

    pub fn parse_array(&mut self) -> Result<Object, String> {
        let mut elements: Vec<Box<Object>> = vec![];
        while let Some(next_token) = self.tokens.get(self.index) {
            if next_token.token_type == TokenType::RSquare {
                break;
            } else if next_token.token_type == TokenType::Comma {
                self.index += 1;
                continue;
            }
            let element = self.parse_object();
            elements.push(Box::new(element?));
            self.index += 1;
        }
        return Ok(Object {
            obj_type: ObjectType::Array(elements)
        })
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

    pub fn parse_object(&mut self) -> Result<Object, String> {
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
                    return Ok(Object {
                        obj_type: ObjectType::String(token.lexeme.clone())
                    })
                },
                TokenType::Number => {
                    return Ok(Object {
                        obj_type: ObjectType::Number(token.lexeme.clone())
                    })
                },
                TokenType::False => {
                    return Ok(Object {
                        obj_type: ObjectType::False
                    })
                },
                TokenType::True => {
                    return Ok(Object {
                        obj_type: ObjectType::True
                    })
                },
                TokenType::Null => {
                    return Ok(Object {
                        obj_type: ObjectType::Null
                    })
                },
                _ => {
                    return Err(format!("invalid token: {:?} line:{}", token, token.line));
                }
            }
        } else {
            Err(format!("No token exists."))
        }
    }

    pub fn parse(&mut self) {
        if let Some(_) = self.tokens.get(self.index) {
            if let Ok(root) = self.parse_object() {
                self.root_object = Some(root);
            } else {
                self.root_object = None
            }
        }
    }
}

