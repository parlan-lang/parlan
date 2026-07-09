
use crate::ast::*;
use crate::lexer::*;

impl Parser {
    pub fn new(src: String) -> Self {
        return Parser {
            source: src.clone(),
            nodes: Vec::new(),
            lexer: Lexer::new(src)
        }
    }

    // auxiliar functions //

    fn peek(&mut self) -> Tk {
        return self.lexer.peek_token();
    }

    fn eat(&mut self, ttype: TkType) -> Tk {
        if self.peek().tk_type == ttype {
            return self.lexer.next_token();
        } else {
            eprintln!("[ERROR] expected token `{:?}` found `{:?}`", ttype, self.peek().tk_type);
            panic!()
        }
    }

    // parser //

    fn parse_decl(&mut self) -> Node {
        match self.peek().tk_type {
            TkType::Func | TkType::Extern => self.parse_func_decl(),
            TkType::Return => self.parse_return(),
            TkType::While => self.parse_while(),
            TkType::If => self.parse_if(),
            TkType::Var => self.parse_var_decl(),
            _ => {
                let node = self.parse_expr();
                self.eat(TkType::Semicolon);
                node
            }
        }
    }

    // statements //

    fn parse_var_decl(&mut self) -> Node {
        self.eat(TkType::Var);
        
        let name_tk = self.eat(TkType::Id);

        let mut vtype = TkType::Err;
        
        if self.peek().tk_type == TkType::Colon {
            self.eat(TkType::Colon);
        
            vtype = self.lexer.next_token().tk_type; // we call the lexer directly
        }
        
        self.eat(TkType::Assing);

        let expr = self.parse_expr();

        self.eat(TkType::Semicolon);

        return Node::VarDecl {
            name_s: name_tk.start, 
            name_e: name_tk.end, 
            vtype, 
            value: Box::new(expr) 
        }
    }

    fn parse_block(&mut self) -> Node {
        self.eat(TkType::Lbrace);

        let mut nodes = Vec::new();

        while self.peek().tk_type != TkType::Rbrace && self.peek().tk_type != TkType::Eof {
            nodes.push(self.parse_decl());
        }

        self.eat(TkType::Rbrace);

        return Node::Block(nodes);
    }

    fn parse_func_decl(&mut self) -> Node {
        let mut is_extern = false;
        if self.peek().tk_type == TkType::Extern {
            self.eat(TkType::Extern);
            is_extern = true
        }

        self.eat(TkType::Func);

        let name_tk = self.eat(TkType::Id);

        self.eat(TkType::Lparen);

        let mut params = Vec::new();

        if self.peek().tk_type != TkType::Rparen {
            let id = self.eat(TkType::Id);
            self.eat(TkType::Colon);
            let ptype = self.lexer.next_token().tk_type; // parameter type
            params.push((id.start, id.end, ptype));

            while self.peek().tk_type == TkType::Comma {
                self.eat(TkType::Comma);
                
                if self.peek().tk_type == TkType::VaArgs {
                    let tk = self.eat(TkType::VaArgs);
                    params.push((tk.start,tk.end,TkType::VaArgs));
                    break;
                }

                let id = self.eat(TkType::Id);
                self.eat(TkType::Colon);
                let ptype = self.lexer.next_token().tk_type;
                params.push((id.start, id.end, ptype));
            }
        }

        self.eat(TkType::Rparen);
        
        self.eat(TkType::Colon);

        let rtype = self.lexer.next_token().tk_type;

        if is_extern {
            self.eat(TkType::Semicolon);
            
            return Node::FuncDecl { 
                name_s: name_tk.start, name_e: name_tk.end, 
                args: params, 
                rtype, 
                body: None, 
                is_extern: true
            }
        }

        let block = self.parse_block();

        return Node::FuncDecl { 
            name_s: name_tk.start, name_e: name_tk.end, 
            args: params, 
            rtype, 
            body: Some(Box::new(block)), 
            is_extern: false 
        }
    }

    fn parse_return(&mut self) -> Node {
        self.eat(TkType::Return);

        let expr = self.parse_expr();

        self.eat(TkType::Semicolon);

        return Node::Return(Box::new(expr));
    }

    fn parse_while(&mut self) -> Node {
        self.eat(TkType::While);

        let expr = self.parse_expr();

        let body = self.parse_block();

        return Node::While {
            cond: Box::new(expr),
            body: Box::new(body)
        }
    }

    fn parse_if(&mut self) -> Node {
        self.eat(TkType::If);

        let expr = self.parse_expr();

        let then_br = self.parse_block();

        if self.peek().tk_type == TkType::Else {
            self.lexer.next_token();

            if self.peek().tk_type == TkType::If {
                let elif = self.parse_if();

                return Node::If {
                    cond: Box::new(expr),
                    then_br: Box::new(then_br),
                    else_br: Some(Box::new(elif))
                }
            }
            let else_br = self.parse_block();

            return Node::If {
                cond: Box::new(expr),
                then_br: Box::new(then_br),
                else_br: Some(Box::new(else_br))
            }
        }

        return Node::If {
            cond: Box::new(expr),
            then_br: Box::new(then_br),
            else_br: None
        }
    }

    // expretions //

    fn parse_expr(&mut self) -> Node {
        return self.parse_or()
    }

    fn parse_or(&mut self) -> Node {
        let mut left = self.parse_and();

        while self.peek().tk_type == TkType::Or {
            let op = self.lexer.next_token().tk_type;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(self.parse_and()),
                op
            }
        }
        
        return left;
    }

    fn parse_and(&mut self) -> Node {
        let mut left = self.parse_eq();

        while self.peek().tk_type == TkType::And {
            let op = self.lexer.next_token().tk_type;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(self.parse_eq()),
                op
            }
        }
        
        return left;
    }

    fn parse_eq(&mut self) -> Node {
        let mut left = self.parse_comparation();

        while self.peek().tk_type == TkType::Eq || 
              self.peek().tk_type == TkType::Ne 
        {
            let op = self.lexer.next_token().tk_type;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(self.parse_comparation()),
                op
            }
        }
        
        return left;
    }

    fn parse_comparation(&mut self) -> Node {
        let mut left = self.parse_term();

        while self.peek().tk_type == TkType::Lt || 
              self.peek().tk_type == TkType::Gt ||
              self.peek().tk_type == TkType::Le ||
              self.peek().tk_type == TkType::Ge 
        {
            let op = self.lexer.next_token().tk_type;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(self.parse_term()),
                op
            }
        }
        
        return left;
    }

    fn parse_term(&mut self) -> Node {
        let mut left = self.parse_factor();

        while self.peek().tk_type == TkType::Plus || 
              self.peek().tk_type == TkType::Minus 
        {
            let op = self.lexer.next_token().tk_type;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(self.parse_factor()),
                op
            }
        }
        
        return left;
    }

    fn parse_factor(&mut self) -> Node {
        let mut left = self.parse_unary();

        while self.peek().tk_type == TkType::Star || 
              self.peek().tk_type == TkType::Slash 
        {
            let op = self.lexer.next_token().tk_type;

            left = Node::BinOp {
                left: Box::new(left),
                right: Box::new(self.parse_unary()),
                op
            }
        }
        
        return left;
    }

    fn parse_unary(&mut self) -> Node {
        if self.peek().tk_type == TkType::Not || self.peek().tk_type == TkType::Minus {
            let tk = self.lexer.next_token().tk_type;
            return Node::Unary { 
                right: Box::new(self.parse_unary()), 
                op: tk 
            }
        } else {
            return self.parse_func_call();
        }
    }

    fn parse_func_call(&mut self) -> Node {
        let mut left = self.parse_primary();

        if self.peek().tk_type == TkType::Lparen {
            self.lexer.next_token();

            let mut args = Vec::new();

            if self.peek().tk_type != TkType::Rparen {
                let expr = self.parse_expr();
                args.push(expr);

                while self.peek().tk_type == TkType::Comma {
                    self.lexer.next_token();

                    let expr = self.parse_expr();
                    args.push(expr);
                }
            }
            self.eat(TkType::Rparen);
            left = Node::FuncCall {
                name: Box::new(left),
                args
            };
        }
        
        return left;
    }

    fn parse_primary(&mut self) -> Node {
        match self.peek().tk_type {
            TkType::IntL => {
                let tk = self.lexer.next_token();
                return Node::Integer(self.source.get(tk.start..tk.end).unwrap().parse().unwrap());
            }
            TkType::FloatL => {
                let tk = self.lexer.next_token();
                return Node::Float(self.source.get(tk.start..tk.end).unwrap().parse().unwrap());
            }
            TkType::StringL => {
                let tk = self.lexer.next_token();
                return Node::Str(tk.start, tk.end);
            }
            TkType::False => {
                self.lexer.next_token();
                return Node::Bool(false)
            }
            TkType::True => {
                self.lexer.next_token();
                return Node::Bool(true)
            }
            TkType::Id => {
                let id = self.lexer.next_token();
                if self.peek().tk_type == TkType::Assing {
                    self.lexer.next_token();

                    let expr = self.parse_expr();

                    return Node::VarReassing { 
                        name_s: id.start, 
                        name_e: id.end, 
                        value: Box::new(expr) 
                    }
                }
                return Node::Id(id.start, id.end)
            }
            TkType::Lparen => {
                let expr;
                self.eat(TkType::Lparen);
                expr = self.parse_expr();
                self.eat(TkType::Rparen);

                return expr;
            }
            TkType::Lbrace => {
                return self.parse_block()
            }
            _ => {
                todo!("case for {:?} in parse_primary not yet implemented", self.peek().tk_type)
            }
        }
    }

    // main function
    pub fn parse(&mut self) {
        while self.peek().tk_type != TkType::Eof {
            let node = self.parse_decl();
            self.nodes.push(node);
        }
    }
}
