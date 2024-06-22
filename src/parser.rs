use std::collections::HashMap;

use crate::lexer::Token;

#[derive(Debug)]
pub struct Parser {
    tokens: &'static Vec<Token>,
}

#[derive(Debug)]
pub enum ObjectType {
    Number,
    String,
    Array(Vec<Box<ObjectType>>),
    Map(HashMap<String, Box<ObjectType>>)
}

#[derive(Debug)]
pub struct Object {
    obj_type: ObjectType,
}

impl Parser {
    // I just know that tokens will exist (source: trust me bro)
    pub fn new(tokens: &'static Vec<Token>) -> Self {
        Self {
            tokens
        }
    }

    pub fn parse() {
        todo!("design the ast struct first");
    }
}

