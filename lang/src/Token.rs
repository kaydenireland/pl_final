#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Token {

    // general
    EOI,
    ERROR,

    // atoms
    ID(String),
    LIT_INT32(i32),
    LIT_FLT32,
    
    TYPE_INT32,
    TYPE_FLT32,
    TYPE_CHAR,

    // arithmetic operators
    ADD,
    SUB,
    MUL,
    DIV,

    // relational operators
    LT,          // less than
    GT,          // greater than
    NLT,      // not less than == greater than or equal
    NGT,      // not greater than == less than or equal
    OP_EQUAL,       // equal
    OP_NOT_EQUAL,   // not equal

    // logical operators
    NOT,
    AND,
    OR,

    // other operators
    ASSIGN,

    // nesting
    PARENT_L,
    PARENT_R,
    BRACKET_L,
    BRACKET_R,
    BRACE_L,
    BRACE_R,

    // separators
    POINT,
    COMMA,
    COLON,
    SEMICOLON,
    ARROW_R,

    // keywords
    FUNC,
    LET,
    IF,
    ELSE,
    RETURN,

    // meta tokens
    META_BLOCK,
    META_IF,
    
    //meta operations
    START,
    FUNC_DECL,
    PARAM_LIST,
    PARAM,
    BLOCK,
}


impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for Token {
    
}

impl Token {
    pub fn id() -> Token {
        Token::ID(String::new())
    }
    pub fn lit_i32() -> Token { Token::LIT_INT32(0) }
    
    pub fn is_type(&self)-> bool{
        matches!(self,Token::TYPE_INT32 | Token::TYPE_FLT32 | Token::TYPE_CHAR)
    }
}
