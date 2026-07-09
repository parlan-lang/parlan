#![allow(unused)]

use crate::ast::*;

pub struct SemChecker<'a> {
    envs: Vec<Vec<(&'a str)>>, // variables
    funcs: Vec<(&'a str, Vec<&'a str>, &'a Option<Box<Node>>)>, // (name, number of parameters, body (None if the function is extern))
    nodes: &'a Vec<Node>,
    curr_func: Option<(&'a str, Vec<&'a str>, &'a Option<Box<Node>>)>, // current function
    src: &'a str
}

impl<'a> SemChecker<'a> {
    pub fn new(nodes: &'a Vec<Node>, src: &'a str) -> Self {
        return SemChecker {
            envs: vec![vec![]],
            funcs: Vec::new(),
            nodes,
            curr_func: None,
            src
        };
    }

    fn populate_funcs(&mut self) {
        for node in self.nodes {
            match node {
                Node::FuncDecl { name_s, name_e, args, rtype, body, is_extern } => {
                    self.funcs.push((
                        &self.src[*name_s..*name_e],
                        args.iter().map(|a| &self.src[a.0..a.1]).collect(),
                        body
                    ));
                }
                _ => continue
            }
        }
    }

    fn analize_node(&mut self, node: &'a Node) {
        match node {
            Node::Id(start, end) => {
                let id = &self.src[*start..*end];
                
                for env in self.envs.iter().rev() {
                    if env.iter().find(|entry| id == **entry).is_some() {
                        return;
                    } else {
                        continue;
                    }
                }

                eprintln!("[ERROR] undeclared identifier `{id}`");
                panic!();
            },
            Node::BinOp { left, right, op } => {
                self.analize_node(left);
                self.analize_node(right);
            },
            Node::Unary { right, op } => {
                self.analize_node(right);
            },
            Node::VarDecl { name_s, name_e, vtype, value } => {
                let id = &self.src[*name_s..*name_e];
                
                self.analize_node(value);

                self.envs.last_mut().unwrap().push(id);
            },
            Node::VarReassing { name_s, name_e, value } => {
                self.analize_node(value);

                let id = &self.src[*name_s..*name_e];
                
                for env in &self.envs {
                    if env.iter().find(|entry| id == **entry).is_some() {
                        return;
                    } else {
                        continue;
                    }
                }

                eprintln!("[ERROR] undeclared identifier `{id}`");
                panic!();
            },
            Node::Block(nodes) => {
                self.envs.push(Vec::new());
                for node in nodes {
                    self.analize_node(node);
                }
                self.envs.pop();
            },
            Node::FuncDecl { name_s, name_e, args, rtype, body, is_extern } => {
                if let Some(_) = self.curr_func {
                    eprintln!("[ERROR] cannot declare a function inside a function");
                    panic!();
                }
                
                self.curr_func = Some((
                    &self.src[*name_s..*name_e],
                    args.iter().map(|a| &self.src[a.0..a.1]).collect(),
                    body
                ));

                self.curr_func = None;
            },
            Node::Return(node) => {
                self.analize_node(node);
            },
            Node::FuncCall { name, args } => {
                let name = match &**name {
                    Node::Id(start, end) => &self.src[*start..*end],
                    _ => unreachable!()
                };

                let Some(func) = self.funcs.iter().find(|f| f.0 == name) else {
                    eprintln!("[ERROR] undeclared function `{name}`");
                    panic!();
                };

                let is_variadic = *func.1.last().unwrap() == "...";

                if !is_variadic && (args.len() != func.1.len()) {
                    eprintln!("[ERROR] wrong number of arguments, expected {} found {}", func.1.len(), args.len());
                    panic!();
                }
            },
            Node::While { cond, body } => {
                self.analize_node(cond);
                self.analize_node(body);
            },
            Node::If { cond, then_br, else_br } => {
                self.analize_node(cond);
                self.analize_node(then_br);
                if let Some(else_br) = else_br {
                    self.analize_node(else_br);
                }
            },
            _ => {
                return; 
            }
        }
    }

    fn analize_func(&mut self) {
        let Some(ref func) = self.curr_func else {
            return;
        };

        let Some(body) = func.2 else {
            return;
        };

        self.envs.push(Vec::new()); // enter new scope

        for arg in &func.1 {
            self.envs.last_mut().unwrap().push(arg);
        }

        match &**body {
            Node::Block(nodes) => nodes.iter().for_each(|n| self.analize_node(n)),
            _ => unreachable!("the parser ensures that this never happens")
        }

        self.envs.pop(); // leave new scope
    }

    pub fn analize(&mut self) {
        self.populate_funcs();
        
        for func in self.funcs.clone() {
            self.curr_func = Some(func);
            self.analize_func();
        }
    }
}