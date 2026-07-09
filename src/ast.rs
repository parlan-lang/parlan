//! in this file is implemented the AST as an enumeration

use std::fmt::Debug;

use crate::lexer::{Lexer, TkType};

// enum representing a node in the AST
#[derive(Debug, Clone)]
pub enum Node {
    Integer(usize),
    Float(f64),
    Bool(bool),
    Str(usize, usize), // start and end
    Id(usize, usize), // start and end
    BinOp {
        left:  Box<Node>,
        right: Box<Node>,
        op:    TkType
    },
    Unary {
        right: Box<Node>,
        op:    TkType
    },
    VarDecl {
        name_s: usize, // name start
        name_e: usize, // name end 
        vtype:  TkType, // variable type
        value:  Box<Node> // the performance cost is acceptable
    },
    VarReassing {
        name_s: usize, name_e: usize,
        value:  Box<Node>
    },
    Block(Vec<Node>),
    FuncDecl {
        name_s:    usize, name_e: usize,
        args:      Vec<(usize, usize, TkType)>,
        rtype:     TkType,
        body:      Option<Box<Node>>, // None in case if is an external function
        is_extern: bool
    },
    Return(Box<Node>),
    FuncCall {
        name: Box<Node>,
        args: Vec<Node>,
    },
    While {
        cond: Box<Node>,
        body: Box<Node>
    },
    If {
        cond:    Box<Node>,
        then_br: Box<Node>,
        else_br: Option<Box<Node>>
    }
}

// the parser
pub struct Parser {
    pub source: String,
    pub nodes:  Vec<Node>,
    pub lexer:  Lexer
}

impl Parser {
    pub fn dbg_print(&self) {
        for n in &self.nodes {
            eprintln!("{n:#?}");
        }
    }
}
