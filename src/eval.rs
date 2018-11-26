use std::collections::HashMap;
use std::f64::consts::PI;

use rug::ops::Pow;

use crate::ast::{Node, Number};

#[derive(Debug, Clone)]
struct Function {
    args: Vec<String>,
    body: Vec<Node>,
}

pub struct EvalContext {
    pub last_result: Option<Number>,
    values: HashMap<String, Number>,
    functions: HashMap<String, Function>,
    local_values: Vec<HashMap<String, Number>>,
}

impl EvalContext {
    pub fn new() -> EvalContext {
        let values = vec![("pi".to_string(), rug::Float::with_val(53, PI).into())]
            .into_iter()
            .collect();
        EvalContext {
            last_result: None,
            values,
            functions: HashMap::new(),
            local_values: vec![],
        }
    }

    fn eval_op(&mut self, lh: Node, op: String, rh: Node) -> Number {
        match op.as_str() {
            "+" => self.eval_internal(lh) + self.eval_internal(rh),
            "-" => self.eval_internal(lh) - self.eval_internal(rh),
            "*" => self.eval_internal(lh) * self.eval_internal(rh),
            "/" => self.eval_internal(lh) / self.eval_internal(rh),
            "**" => self.eval_internal(lh).pow(self.eval_internal(rh)),
            "<<" => self.eval_internal(lh) << self.eval_internal(rh),
            ">>" => self.eval_internal(lh) >> self.eval_internal(rh),
            _ => unimplemented!(),
        }
    }

    fn lookup_ident(&self, key: &str) -> Number {
        for local in &self.local_values {
            if let Some(val) = local.get(key) {
                return val.clone();
            }
        }
        self.values.get(key).cloned().unwrap_or_default()
    }

    fn eval_internal(&mut self, node: Node) -> Number {
        use crate::ast::Node::*;
        match node {
            Prev => self.last_result.clone().unwrap_or_default(),
            Ident(key) => self.lookup_ident(&key),
            Number(num) => num,
            Infix { lhs, op, rhs } => self.eval_op(*lhs, op, *rhs),
            Let(key, node) => {
                let value = self.eval_internal(*node);
                self.values.insert(key, value.clone());
                value
            }
            FunctionDef { name, args, body } => {
                self.functions.insert(name, Function { args, body });
                Default::default()
            }
            FunctionCall { name, args } => {
                let func = match self.functions.get(&name) {
                    Some(func) => func.clone(),
                    None => return Default::default(),
                };
                let new_local = {
                    args.iter()
                        .zip(func.args.clone())
                        .map(|(a, p)| (p.to_owned(), self.eval_internal(a.clone())))
                        .collect()
                };
                self.local_values.push(new_local);

                for node in &func.body[..func.body.len() - 1] {
                    self.eval_internal(node.clone());
                }
                let ret = self.eval_internal(func.body[func.body.len() - 1].clone());

                self.local_values.pop();

                ret
            }
            Error => panic!("Evaluation of invalid ast"),
        }
    }

    pub fn eval(&mut self, node: Node) -> &Number {
        self.last_result = Some(self.eval_internal(node));
        self.last_result.as_ref().unwrap() // Safe, we just set the value
    }
}
