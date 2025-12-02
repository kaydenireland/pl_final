#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::rc::Rc;
use crate::lexer_mockup::Lexer;
use crate::mtree::MTree;
use crate::parser::Parser;
use crate::token::{Token, Code};

pub struct BP {
    pub left : isize,
    pub right : isize,
    pub unary : isize,
}

impl Code {

    pub fn isPrefixOp(&self) -> bool {
        match self {
            // prefix operators
            Code::OP_NOT => { true } // negation (bool)
            Code::OP_SUB => { true } // opposite (addition)
            Code::OP_DIV => { true } // reciprocal (multiplication)
            // otherwise
            _ => { false }
        }
    }


    pub fn isId(&self) -> bool {
        match self {
            Code::ID(_) => { true } // identifier
            _ => { false } // otherwise
        }
    }


    pub fn isValueAtom(&self) -> bool {
        match self {
            // value atoms
            Code::VAL_BOOL(_) => { true }
            Code::VAL_CHAR(_) => { true }
            Code::VAL_I32(_) => { true }
            Code::VAL_F32(_) => { true }
            // otherwise
            _ => { false }
        }
    }


    pub fn bp(&self) -> BP {
        match self {

            Code::EOI => { BP { left: 0, right: 0, unary: 0 } }
            Code::ERROR => { BP { left: 0, right: 0, unary: 0 } }
            Code::ID(_) => { BP { left: 0, right: 0, unary: 0 } }

            // value atoms
            Code::VAL_BOOL(_) => { BP { left: 0, right: 0, unary: 0 } }
            Code::VAL_CHAR(_) => { BP { left: 0, right: 0, unary: 0 } }
            Code::VAL_I32(_) => { BP { left: 0, right: 0, unary: 0 } }
            Code::VAL_F32(_) => { BP { left: 0, right: 0, unary: 0 } }

            // type atoms
            Code::TYP_BOOL => { BP { left: 0, right: 0, unary: 0 } }
            Code::TYP_CHAR => { BP { left: 0, right: 0, unary: 0 } }
            Code::TYP_I32 => { BP { left: 0, right: 0, unary: 0 } }
            Code::TYP_F32 => { BP { left: 0, right: 0, unary: 0 } }

            // assignment operator
            Code::OP_ASSIGN => { BP { left: 0, right: 0, unary: 0 } }

            // logical operators
            Code::OP_OR => { BP { left: 10, right: 11, unary: 0 } }
            Code::OP_AND => { BP { left: 11, right: 12, unary: 0 } }
            Code::OP_NOT => { BP { left: 18, right: 19, unary: 100 } }

            // relational operators
            Code::OP_LT => { BP { left: 30, right: 30, unary: 0 } }
            Code::OP_GT => { BP { left: 30, right: 30, unary: 0 } }
            Code::OP_NOT_LT => { BP { left: 30, right: 30, unary: 0 } }
            Code::OP_NOT_GT => { BP { left: 30, right: 30, unary: 0 } }
            Code::OP_EQUAL => { BP { left: 30, right: 30, unary: 0 } }
            Code::OP_NOT_EQUAL => { BP { left: 30, right: 30, unary: 0 } }

            // arithmetic operators
            Code::OP_ADD => { BP { left: 30, right: 31, unary: 0 } }
            Code::OP_SUB => { BP { left: 30, right: 31, unary: 100 } }
            Code::OP_MUL => { BP { left: 31, right: 32, unary: 0 } }
            Code::OP_DIV => { BP { left: 31, right: 32, unary: 100 } }
            Code::OP_POW => { BP { left: 33, right: 32, unary: 0 } }

            // nesting
            Code::PAREN_L => { BP { left: 0, right: 0, unary: 0 } }
            Code::PAREN_R => { BP { left: 0, right: 0, unary: 0 } }
            Code::BRACKET_L => { BP { left: 0, right: 0, unary: 0 } }
            Code::BRACKET_R => { BP { left: 0, right: 0, unary: 0 } }
            Code::BRACE_L => { BP { left: 0, right: 0, unary: 0 } }
            Code::BRACE_R => { BP { left: 0, right: 0, unary: 0 } }

            // separator
            Code::POINT => { BP { left: 0, right: 0, unary: 0 } }
            Code::COMMA => { BP { left: 0, right: 0, unary: 0 } }
            Code::COLON => { BP { left: 0, right: 0, unary: 0 } }
            Code::SEMICOLON => { BP { left: 0, right: 0, unary: 0 } }
            Code::ARROW_R => { BP { left: 0, right: 0, unary: 0 } }

            // others: keywords, meta
            _ => { BP { left: 0, right: 0, unary: 0 } }
        }
    }
}

impl Parser {  // pratt expression parser

    pub fn parse_expr(&mut self) -> MTree {
        self.log.info("parse_expr()");
        self.log.indent_inc();
        let tree = self.parse_expr_tok(1);
        self.log.indent_dec();
        return tree;
    }


    pub fn parse_expr_tok(&mut self, rbl: isize) -> MTree {
        let code = self.current().code;

        if code.isPrefixOp() {
            let tree_prefix = self.parse_expr_prefix();
            self.parse_expr_infix(tree_prefix, rbl)
        } else if code == Code::PAREN_L {
            let tree_parens = self.parse_expr_parentheses();
            self.parse_expr_infix(tree_parens, rbl)
        } else if code.isId() || code.isValueAtom() {
            let tree_atom = self.parse_expr_atom();
            self.parse_expr_infix(tree_atom, rbl)
        } else {
            MTree::new(Token::from(Code::ERROR))
        }
    }



    pub fn parse_expr_prefix(&mut self) -> MTree {
        let token = self.current();
        self.advance();
        let tree = self.parse_expr_tok(token.code.bp().unary );
        MTree {
            token,
            children: vec![Rc::new(tree)]
        }
    }


    pub fn parse_expr_parentheses(&mut self) -> MTree {
        self.expect(Code::PAREN_L);
        let tree = self.parse_expr();
        self.expect(Code::PAREN_R);
        return tree;
    }


    pub fn parse_expr_atom(&mut self) -> MTree {
        let atom = self.current();
        self.advance();
        if self.peek(Code::PAREN_L) {
            self.parse_expr_call(atom)
        } else {
            MTree::new(atom)
        }
    }


    pub fn parse_expr_call(&mut self, token: Token) -> MTree {
        let mut tree = MTree::new(token);
        self.expect(Code::PAREN_L);
        if ! self.peek(Code::PAREN_R) {
            tree.children.push(Rc::new(self.parse_expr()) );
            while self.accept(Code::COMMA) {
                tree.children.push(Rc::new(self.parse_expr()) );
            }
        }
        self.expect(Code::PAREN_R);
        return tree;
    }


    pub fn parse_expr_infix(&mut self, mut left: MTree, rbl: isize) -> MTree {
        loop {
            let op_infix = self.current();
            if rbl > op_infix.code.bp().left {
                return left;
            }
            self.advance();
            let right = self.parse_expr_tok(op_infix.code.bp().right);
            left = MTree {
                token: op_infix,
                children: vec![
                    Rc::new(left),
                    Rc::new(right),
                ]
            }
        }
    }

}
