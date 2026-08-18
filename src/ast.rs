//! Abstract Syntax Tree (AST) definition

// Added temporarily to eliminate warnings
// TODO: delete this 
#![allow(unused)]

/// Represents a type
#[derive(Debug)]
pub enum Ty {
    Void,
    Unknown, // an unknown type during parsing, must be infered by the type-checker
    Error,
}

/// Represents an literal 
#[derive(Debug)]
pub enum Literal {
    Integer(isize),
}

/// Represents an expression
#[derive(Debug)]
pub enum Expr<'expr> {
    Literal(Literal),
    Id(&'expr str),
    Error((usize, usize)),
}

/// Represents a statement
#[derive(Debug)]
pub enum Stmt<'stmt> {
    VarDecl {
        name: &'stmt str,
        ty: Ty,
        expr: Expr<'stmt>,
    },
    Block {
        stmts: Vec<Stmt<'stmt>>
    },
    Error((usize, usize))
}

/// Represents an item inside an module ([`AstModule`])
#[derive(Debug)]
pub enum AstItem<'item> {
    Function {
        name: &'item str,
        args: Vec<(Expr<'item>, Ty)>,
        ret: Ty,
        body: Stmt<'item>  
    },
    Error((usize, usize))
}

/// Represents a single compilation unit (a single file)
#[derive(Debug)]
pub struct AstModule<'module> {
    pub file_id: usize,
    pub items: Vec<AstItem<'module>>,
}