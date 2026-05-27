
use crate::{ast::*, lexer::TkType};

pub struct Backend {
    code: String,
    padding: usize,
    source: String, // original source
    buff: String
}

impl Backend {
    pub fn new(source: String) -> Self {
        return Backend {
            code: String::new(),
            padding: 0,
            source,
            buff: String::new()
        }
    }

    fn push_buff(&mut self) {
        self.code.push_str(format!(
            "{}{}\n",
            " ".repeat(self.padding * 4),
            self.buff,
            ).as_str()
        );
        self.buff.clear();
    }

    fn tk_type_to_ctype(&self, ttype: &TkType, is_main_func: bool) -> String {
        match ttype {
            TkType::IntT => "int".to_string(),
            TkType::FloatT => "double".to_string(),
            TkType::StringT => if is_main_func { "char**".to_string() } else { "const char*".to_string() },
            TkType::BoolT => "unsigned char".to_string(),
            TkType::VoidT => "void".to_string(),
            _ => panic!("[ERROR] trying to use token of type {ttype:?} as a type")
        }
    }

    fn emit_expr_c(&self, node: &Node) -> String {
        match node {
            Node::Integer(i) => i.to_string(),
            Node::Float(f) => f.to_string(),
            Node::Bool(b) => if *b { "1".to_string() } else { "0".to_string() },
            Node::Str(start, end) => format!("\"{}\"", self.source.get(*start..*end).unwrap()),
            Node::Id(start, end) => self.source.get(*start..*end).unwrap().to_string(),
            Node::FuncCall { name, args } => {
                let mut rargs = Vec::with_capacity(args.len()); // raw arguments
                for arg in args {
                    rargs.push(format!(
                        "{}",
                        self.emit_expr_c(arg)
                    ));
                }
                let args = rargs.join(",");

                return format!(
                    "{}({})",
                    self.emit_expr_c(name),
                    args
                );
            }
            Node::BinOp { left, right, op } => {
                let (left, right) = (
                    self.emit_expr_c(left),
                    self.emit_expr_c(right)
                );

                return match op {
                    TkType::Plus => format!("({left} + {right})"),
                    TkType::Minus => format!("({left} - {right})"),
                    TkType::Star => format!("({left} * {right})"),
                    TkType::Slash => format!("({left} / {right})"),
                    TkType::Lt => format!("({left} < {right})"),
                    TkType::Gt => format!("({left} > {right})"),
                    TkType::Le => format!("({left} <= {right})"),
                    TkType::Ge => format!("({left} >= {right})"),
                    TkType::Eq => format!("({left} == {right})"),
                    TkType::Ne => format!("({left} != {right})"),
                    TkType::And => format!("({left} && {right})"),
                    TkType::Or => format!("({left} || {right})"),
                    _ => unreachable!()
                }
            }
            Node::Unary { right, op } => {
                let rhs = self.emit_expr_c(right);

                return match op {
                    TkType::Not => format!("!({rhs})"),
                    TkType::Minus => format!("-(rhs)"),
                    _ => unreachable!()
                }
            }
            _ => panic!("[INTERNAL ERROR] trying to treat a statement as an expretion. note: this is an internal error of the compiler")
        }
    }

    fn emit_stat_c(&mut self, node: &Node) {
        match node {
            Node::VarDecl { name_s, name_e, vtype, value } => {
                self.buff.push_str(format!(
                    "{} {} = {};",
                    self.tk_type_to_ctype(vtype, false),
                    self.source.get(*name_s..*name_e).unwrap(),
                    self.emit_expr_c(value)
                ).as_str());
                self.push_buff();
            }
            Node::VarReassing { name_s, name_e, value } => {
                let expr = self.emit_expr_c(value);

                self.buff.push_str(format!(
                    "{} = {};",
                    self.source.get(*name_s..*name_e).unwrap(),
                    expr
                ).as_str());
                self.push_buff();
            }
            Node::Block(nodes) => {
                for n in nodes {
                    self.emit_stat_c(n);
                }
            }
            Node::FuncDecl { name_s, name_e, args, rtype, body, is_extern } => {
                let mut rparams = Vec::with_capacity(args.len()); // raw params
                for param_pair in args {
                    if param_pair.2 == TkType::VaArgs {
                        rparams.push(format!("..."));
                    } else {
                        rparams.push(format!(
                            "{} {}",
                            self.tk_type_to_ctype(&param_pair.2,true),
                            self.source.get(param_pair.0..param_pair.1).unwrap()
                        ));
                    }
                }
                let params = format!("({})", rparams.join(","));

                if *is_extern {
                    self.buff.push_str(format!(
                        "extern {} {}{};\n",
                        self.tk_type_to_ctype(rtype, false),
                        self.source.get(*name_s..*name_e).unwrap(),
                        params,
                    ).as_str());
                    self.push_buff();
                } else {
                    self.buff.push_str(format!(
                        "{} {}{} {{",
                        self.tk_type_to_ctype(rtype, false),
                        self.source.get(*name_s..*name_e).unwrap(),
                        params,
                    ).as_str());
                    self.push_buff();

                    self.padding += 1;

                    self.emit_stat_c(&body.clone().unwrap());

                    self.padding -= 1;

                    self.buff.push_str("}\n");
                    self.push_buff();
                }
                
            }
            Node::Return(expr) => {
                self.buff.push_str(format!(
                    "return {};",
                    self.emit_expr_c(expr)
                ).as_str());
                self.push_buff();
            }
            Node::FuncCall { name, args } => {
                let mut rargs = Vec::with_capacity(args.len()); // raw arguments
                for arg in args {
                    rargs.push(format!(
                        "{}",
                        self.emit_expr_c(arg)
                    ));
                }
                let args = rargs.join(",");

                self.buff.push_str(format!(
                    "{}({});",
                    self.emit_expr_c(name),
                    args
                ).as_str());
                self.push_buff();
            }
            Node::While { cond, body } => {
                let expr = self.emit_expr_c(cond);

                self.buff.push_str(format!("while ({}) {{\n",expr).as_str());
                self.push_buff();

                self.padding += 1;

                self.emit_stat_c(body);

                self.padding -= 1;

                self.buff.push_str("}\n"); 
                self.push_buff();
            }
            Node::If { cond, then_br, else_br } => {
                let expr = self.emit_expr_c(cond);

                self.buff.push_str(format!("if ({}) {{\n",expr).as_str());
                self.push_buff();

                self.padding += 1;

                self.emit_stat_c(then_br);

                self.padding -= 1;

                self.buff.push_str("} ");

                if let Some(else_br) = else_br {
                    self.buff.push_str("else {\n");
                    self.push_buff();

                    self.padding += 1;

                    self.emit_stat_c(else_br);

                    self.padding -= 1;

                    self.buff.push_str("}\n");
                    self.push_buff();
                } else {
                    self.push_buff();
                }
            }
            _ => panic!("[INTERNAL ERROR] trying to treat a expretion as an statement. note: this is an internal error of the compiler")
        }
    }

    pub fn emit_c(&mut self, parser: &Parser) -> String {
        for node in &parser.nodes {
            self.emit_stat_c(node);
        }
        return self.code.clone()
    }
}
