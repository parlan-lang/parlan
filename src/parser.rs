//! Syntactic Analyzer or Parser using a Recursive Descent approach

fn span_to_range(span: (usize, usize)) -> std::ops::Range<usize> {
    span.0..span.1
}

use crate::ast::*;
use crate::lexer::*;

use codespan_reporting::diagnostic::*;

/// Represents the Syntactic Analyzer
pub struct Parser<'parser> {
    lexer: std::iter::Peekable<Lexer<'parser>>,
    file_id: usize,
    src: &'parser str,
    pub errors: Vec<Diagnostic<usize>>
}

impl<'parser> Parser<'parser> {
    pub fn new(file_id: usize, src: &'parser str, lexer: Lexer<'parser>) -> Self {
        Self {
            lexer: lexer.peekable(),
            file_id,
            src,
            errors: Vec::new()
        }
    }

    /// Returns the current token advancing if is not the end of the file,
    /// otherwise it returns `None`
    fn next_token(&mut self) -> Option<Token> {
        loop {
            match self.lexer.next()? {
                Ok(token) => return Some(token),
                Err(diag) => {
                    self.errors.push(diag);
                }
            }
        }
    }

    /// Returns the current token without advancing if is not the end of the file,
    /// otherwise it returns `None` 
    fn peek_token(&mut self) -> Option<&Token> {
        loop {
            if let Some(Err(_)) = self.lexer.peek() {
                if let Some(Err(diag)) = self.lexer.next() {
                    self.errors.push(diag);
                }
            } else {
                return match self.lexer.peek() {
                    Some(Ok(token)) => Some(token),
                    _ => None
                }
            }
        }
    }

    /// Skips tokens until it reaches a synchronization point (a `;`, `var`, or `func`)
    fn synchronize(&mut self) {
        self.next_token();

        while let Some(tok) = self.peek_token() {
            match tok.kind {
                TokenKind::Semi | TokenKind::VarKw | TokenKind::FuncKw => return,
                _ => {
                    self.next_token();
                }
            }
        }
    }

    /// Takes a [`TokenKind`] and checks if the current tokens matchs the type,
    /// returns the token if it match, or returns the token's span if not
    fn expect(
        &mut self, 
        kind: TokenKind,
        msg: String,
        eof_msg: String,
        label_msg: String,
    ) -> Result<Token, (usize, usize)> {
        match self.next_token() {
            Some(tk) if tk.kind == kind => Ok(tk),
            Some(other) => {
                self.errors.push(
                    Diagnostic::error()
                        .with_message(msg)
                        .with_label(
                            Label::primary(self.file_id, span_to_range(other.span)).with_message(label_msg)
                        )
                );
                self.synchronize();
                Err(other.span)
            }
            None => {
                self.errors.push(
                    Diagnostic::error()
                        .with_message("unexpected end of file")
                        .with_label(
                            Label::primary(self.file_id, self.src.len() - 1..self.src.len()).with_message(eof_msg)
                        )      
                );
                Err((self.src.len() - 1, self.src.len()))
            }
        }
    }

    fn get_span(&self, span: (usize, usize)) -> &str {
        &self.src[span.0..span.1]
    }

    fn parse_expr(&mut self) -> Expr<'parser> {
        let start_span = self.peek_token().map(|t| t.span).unwrap_or((0,0));
        
        if let Some(token) = self.next_token() {
            match token.kind {
                TokenKind::Int => Expr::Literal(Literal::Integer(self.get_span(token.span).parse().unwrap())),
                _ => {
                    self.errors.push(
                        Diagnostic::error()
                            .with_message(format!("expected an expression, found `{}` instead", self.get_span(token.span)))
                            .with_label(
                                Label::primary(self.file_id, span_to_range(token.span)).with_message("expected expression")
                            )
                    );
                    self.synchronize();
                    Expr::Error(start_span)   
                }
            }
        } else {
            self.errors.push(
                    Diagnostic::error()
                        .with_message("unexpected end of file")
                        .with_label(
                            Label::primary(self.file_id, span_to_range(start_span)).with_message("expected an expression")
                        )      
                );
                return Expr::Error(start_span)
        }
    }

    fn parse_stmt_var_decl(&mut self) -> Stmt<'parser> {
        let start_span = self.peek_token().map(|t| t.span).unwrap_or((0,0));

        // consume `var`
        self.next_token();

        let curr = self.peek_token().unwrap().kind;
        let name = match self.expect(
            TokenKind::Id, 
            format!("expected identifier after `var`, found {} instead", curr), 
            "unterminated `var` statemet".into(), 
            "expected identifier".into())
        {
            Ok(tok) => tok,
            Err(span) => return Stmt::Error((start_span.0,span.1))
        };

        // TODO: accept an optional explicit type
        let ty = Ty::Unknown;

        let curr = self.peek_token().unwrap().kind;
        match self.expect(
            TokenKind::Assign, 
            format!("expected `=`, found {} instead", curr), 
            "unterminaded variable declaration".into(), 
            "expected `=` here".into()
        ) {
            Ok(_) => {}
            Err(span) => return Stmt::Error((start_span.0,span.1))
        };

        let expr = self.parse_expr();

        match self.expect(
            TokenKind::Semi, 
            "expected `;` after variable declaration".into(), 
            "unterminated variable declaration".into(), 
            "expected `;` here".into()
        ) {
            Ok(_) => {}
            Err(span) => return Stmt::Error((start_span.0,span.1))
        };

        Stmt::VarDecl {
            name: &self.src[name.span.0..name.span.1], 
            ty, 
            expr 
        }
    }

    fn parse_stmt_block(&mut self) -> Stmt<'parser> {
        match self.expect(
            TokenKind::OpenBrace, 
            "expected `{`".into(), 
            "unterminaded block statement".into(), 
            "expected `{` here".into()
        ) {
            Ok(_) => {},
            Err(span) => return Stmt::Error(span)
        };

        let mut stmts = Vec::new();

        while self.peek_token().unwrap().kind != TokenKind::CloseBrace {
            stmts.push(self.parse_stmt());
        }

        match self.expect(
            TokenKind::CloseBrace, 
            "expected `}`".into(), 
            "unterminaded block statement".into(), 
            "expected `}` here".into()
        ) {
            Ok(_) => {},
            Err(span) => return Stmt::Error(span)
        };

        Stmt::Block { stmts }
    }

    pub fn parse_stmt(&mut self) -> Stmt<'parser> {
        match self.peek_token() {
            Some(tk) if tk.kind == TokenKind::VarKw => self.parse_stmt_var_decl(),
            Some(tk) if tk.kind == TokenKind::OpenBrace => self.parse_stmt_block(),
            Some(other) => {
                let span = other.span;

                self.errors.push(
                    Diagnostic::error()
                        .with_message("expected an statement")
                        .with_label(
                            Label::primary(self.file_id, span_to_range(span))
                        )
                );
                self.synchronize();
                Stmt::Error(span)
            }
            None => {
                self.errors.push(
                    Diagnostic::error()
                        .with_message("unexpected end of file, expected an statement")
                        .with_label(
                            Label::primary(self.file_id, self.src.len()..self.src.len()).with_message("expected an statement")
                        )
                );
                Stmt::Error((self.src.len(), self.src.len()))
            }
        }
    }

    fn parse_item_func_decl(&mut self) -> AstItem<'parser> {
        let start_span = self.peek_token().map(|t| t.span).unwrap_or((0,0));

        // consume `func`
        self.next_token();

        let name = match self.expect(
            TokenKind::Id, 
            "expected identifier after `func`".into(), 
            "unterminaded function declaration".into(), 
            "expected identifier".into()
        ) {
            Ok(tk) => tk,
            Err(span) => return AstItem::Error((start_span.0, span.1))
        };

        match self.expect(
            TokenKind::OpenParen, 
            "expected `(` after function name".into(), 
            "unterminaded function declaration".into(), 
            "expected `(`".into()
        ) {
            Ok(_) => {},
            Err(span) => return AstItem::Error((start_span.0, span.1))
        };

        let args : Vec<(Expr, Ty)>= vec![];

        match self.expect(
            TokenKind::CloseParen, 
            "expected `)`, functions doesn't support parameters yet".into(), 
            "unterminaded function declaration".into(), 
            "expected `)`".into()
        ) {
            Ok(_) => {},
            Err(span) => return AstItem::Error((start_span.0, span.1))
        };

        // TODO: accept optional return type (or default to void)
        let ret = Ty::Void;

        let body = self.parse_stmt_block();
        
        AstItem::Function { 
            name: &self.src[name.span.0..name.span.1], 
            args, 
            ret, 
            body 
        }
    }

    fn parse_item(&mut self) -> AstItem<'parser> {
        match self.peek_token() {
            Some(tk) if tk.kind == TokenKind::FuncKw => self.parse_item_func_decl(),
            Some(other) => {
                let span = other.span;

                self.errors.push(
                    Diagnostic::error()
                        .with_message("expected an item")
                        .with_label(
                            Label::primary(self.file_id, span_to_range(span))
                        )
                        .with_note(
                            r#"an item can be either a function or a constant"#
                        )
                );
                self.synchronize();
                AstItem::Error(span)
            }
            None => {
                self.errors.push(
                    Diagnostic::error()
                        .with_message("unexpected end of file, expected an item")
                        .with_label(
                            Label::primary(self.file_id, self.src.len()..self.src.len()).with_message("expected an item")
                        )
                );
                AstItem::Error((self.src.len(), self.src.len()))
            }
        }
    }

    pub fn parse(&mut self) -> AstModule<'parser> {
        let mut items = Vec::new();
        
        while let Some(_) = self.peek_token() {
            items.push(self.parse_item());
        }

        AstModule { 
            file_id: self.file_id, 
            items 
        }
    }
}