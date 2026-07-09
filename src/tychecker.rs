#![allow(unused)]

use std::{clone, collections::HashMap};

use crate::{ast::*, lexer::TkType};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Type {
    Int, 
    Float,
    Str,
    Bool,
    Void,
    Unknown
}

pub struct TyChecker<'a> {
    envs: Vec<HashMap<&'a str, Type>>,
    funcs: Vec<(&'a str, Vec<(&'a str, Type)>, Type, &'a Option<Box<Node>>)>, // (name, [name, type], return type, body (None if the function is extern))
    nodes: &'a Vec<Node>,
    curr_func: Option<(&'a str, Vec<(&'a str, Type)>, Type, &'a Option<Box<Node>>)>,
    src: &'a str
}

impl<'a> TyChecker<'a> {
    pub fn new(nodes: &'a Vec<Node>, src: &'a str) -> Self {
        return TyChecker {
            envs: vec![HashMap::new()], // global env
            funcs: Vec::new(),
            nodes,
            curr_func: None,
            src
        };
    }

    fn tktype_2_type(&self, ty: TkType) -> Type {
        match ty {
            TkType::IntT => Type::Int,
            TkType::FloatT => Type::Float,
            TkType::StringT => Type::Str,
            TkType::BoolT => Type::Bool,
            TkType::VoidT => Type::Bool,
            TkType::VaArgs => Type::Unknown,
            _ => {
                eprintln!("[ERROR] cannot use token`{ty:?}` as a type");
                panic!();
            }
        }
    }

    fn populate_funcs(&mut self) {
        for node in self.nodes {
            match node {
                Node::FuncDecl { name_s, name_e, args, rtype, body, is_extern } => {
                    self.funcs.push((
                        &self.src[*name_s..*name_e],
                        args.iter().map(|a| (&self.src[a.0..a.1], self.tktype_2_type(a.2))).collect(),
                        self.tktype_2_type(*rtype),
                        body
                    ));
                }
                _ => continue
            }
        }
    }

    fn analize_node(&mut self, node: &'a Node) -> Type {
        match node {
            Node::Integer(_) => Type::Int,
            Node::Float(_) => Type::Float,
            Node::Bool(_) => Type::Bool,
            Node::Str(_, _) => Type::Str,
            Node::Id(s, e) => {
                let id = &self.src[*s..*e];

                for env in self.envs.iter().rev() {
                    if let Some(id) = env.iter().find(|i| *i.0 == id) {
                        return *env.get(id.0).unwrap();
                    }
                }

                unreachable!()
            },
            Node::BinOp { left, right, op } => {
                let lhs = self.analize_node(left);
                let rhs = self.analize_node(right);

                if lhs != rhs {
                    eprintln!("[ERROR] type mismatch, both types in the operation must be the same");
                    panic!();
                }

                match op {
                    TkType::Plus | TkType::Minus | TkType::Star => {
                        if lhs != Type::Int && lhs != Type::Float {
                            eprintln!("[ERROR] cannot do arithmetic over values of type `{lhs:?}`");
                            panic!()
                        }

                        return lhs;
                    }
                    TkType::Slash => {
                        if lhs != Type::Int && lhs != Type::Float {
                            eprintln!("[ERROR] cannot do arithmetic over values of type `{lhs:?}`");
                            panic!()
                        }

                        return Type::Float;
                    }
                    TkType::Lt | TkType::Gt | TkType::Le | TkType::Ge => {
                        if lhs != Type::Int && lhs != Type::Float {
                            eprintln!("[ERROR] cannot do arithmetic over values of type `{lhs:?}`");
                            panic!()
                        }

                        return Type::Bool;
                    }
                    TkType::And | TkType::Or => {
                        if lhs != Type::Bool {
                            eprintln!("[ERROR] cannot do boolean operations over values of type `{lhs:?}`");
                            panic!()
                        }

                        return Type::Bool;
                    } 
                    TkType::Eq | TkType::Ne => {
                        if lhs != Type::Bool && lhs != Type::Int && lhs != Type::Float {
                            eprintln!("[ERROR] cannot compare values that are not boolean or numeric");
                            panic!()
                        }

                        return Type::Bool;
                    }
                    _ => unreachable!("{op:?}")
                }
            },
            Node::Unary { right, op } => {
                let ty = self.analize_node(right);

                if *op == TkType::Not {
                    match ty {
                        Type::Bool => ty,
                        _ => {
                            eprintln!("[ERROR] cannot apply `not` over non-boolean values");
                            panic!();
                        }
                    }
                } else {
                    match ty {
                        Type::Int | Type::Float => ty,
                        _ => {
                            eprintln!("[ERROR] cannot apply `-` over non-numeric values");
                            panic!()
                        }
                    }
                }
            },
            Node::VarDecl { name_s, name_e, vtype, value } => {
                let id = &self.src[*name_s..*name_e];

                let ty = self.analize_node(value);

                if *vtype != TkType::Err && ty != self.tktype_2_type(*vtype) {
                    eprintln!("[ERROR] expected value to be of type `{:?}`, found {ty:?}", self.tktype_2_type(*vtype));
                    panic!()
                } else {
                    self.envs.last_mut().unwrap().insert(id, ty);
                    Type::Void
                }
            },
            Node::VarReassing { name_s, name_e, value } => {
                let id = &self.src[*name_s..*name_e];
                let ty = self.analize_node(value);

                for env in self.envs.iter().rev() {
                    if env.contains_key(id) {
                        if *env.get(id).unwrap() != ty {
                            eprintln!("[ERROR] expected type `{:?}`, found {ty:?}", *env.get(id).unwrap());
                            panic!()
                        } else {
                            break;
                        }
                    }
                }

                Type::Void
            },
            Node::Block(nodes) => {
                for node in nodes {
                    self.analize_node(node);
                }
                Type::Void
            },
            Node::FuncDecl { name_s, name_e, args, rtype, body, is_extern } => {
                let id = &self.src[*name_s..*name_e];
                
                self.curr_func = Some((
                    id,
                    args.iter().map(|a| (&self.src[a.0..a.1], self.tktype_2_type(a.2))).collect(),
                    self.tktype_2_type(*rtype),
                    body
                ));

                self.analize_func();

                self.curr_func = None;

                Type::Void
            },
            Node::Return(node) => {
                let ty = self.analize_node(node);
                let curr_ret_ty = self.curr_func.as_ref().unwrap().2;

                if curr_ret_ty != ty {
                    eprintln!("[ERROR] expected return type `{curr_ret_ty:?}`, found `{ty:?}`");
                    panic!()
                }

                Type::Void
            },
            Node::FuncCall { name, args } => {
                let name = match &**name {
                    Node::Id(s, e) => &self.src[*s..*e],
                    _ => unreachable!()
                };

                let func = self.funcs.iter().find(|f| f.0 == name).unwrap().clone();

                if func.1.last().unwrap().0 == "..." {
                    // for simplicity, we let it as this
                    return func.2;
                }

                for i in 0..args.len() {
                    let ty = self.analize_node(&args[i]);
                    if ty != func.1[i].1 {
                        eprintln!("[ERROR] expected argument of type `{:?}`, found `{ty:?}`", func.1[i].1);
                        panic!()
                    }
                }

                func.2                
            },
            Node::While { cond, body } => {
                let cond_ty = self.analize_node(cond);

                if cond_ty != Type::Bool {
                    eprintln!("[ERROR] condition must be a boolean value");
                    panic!();
                }

                let body = match &**body {
                    Node::Block(nodes) => nodes,
                    _ => unreachable!()
                };

                for node in body {
                    self.analize_node(node);
                }

                Type::Void
            },
            Node::If { cond, then_br, else_br } => {
                let cond_ty = self.analize_node(cond);

                if cond_ty != Type::Bool {
                    eprintln!("[ERROR] condition must be a boolean value");
                    panic!();
                }

                let then_br = match &**then_br {
                    Node::Block(nodes) => nodes,
                    _ => unreachable!()
                };

                for node in then_br {
                    self.analize_node(node);
                }

                if let Some(else_br) = else_br {
                    self.analize_node(else_br);
                }

                Type::Void
            },
        }
    }

    fn analize_func(&mut self) {
        let Some(func) = self.curr_func.clone() else { // performance cost of cloninig is acceptable
            return;
        };

        self.envs.push(HashMap::new());

        let Some(body) = func.3 else {
            return;
        };

        for arg in &func.1 {
            self.envs.last_mut().unwrap().insert(arg.0, arg.1);
        }

        match &**body {
            Node::Block(nodes) => {
                for node in nodes {
                    self.analize_node(node);
                }
            },
            _ => unreachable!()
        }
    }

    pub fn analize(&mut self) {
        self.populate_funcs();

        for func in self.funcs.clone() {
            self.curr_func = Some(func);
            self.analize_func();
        }
    }
}