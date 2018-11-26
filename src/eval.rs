use std::collections::HashMap;
use std::f64::consts::PI;

use rug::ops::Pow;

use crate::ast::{Node, Number};

#[derive(Debug, Clone)]
enum Function {
    Builtin(String),
    UserDefined {
        params: Vec<String>,
        body: Vec<Node>,
    },
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
        let functions = vec![
            ("sin".to_string(), Function::Builtin("sin".to_string())),
            ("cos".to_string(), Function::Builtin("cos".to_string())),
            ("tan".to_string(), Function::Builtin("tan".to_string())),
        ]
        .into_iter()
        .collect();
        EvalContext {
            last_result: None,
            values,
            functions,
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

    fn eval_function(&mut self, name: &str, args: Vec<Node>) -> Number {
        let func = match self.functions.get(name) {
            Some(func) => func.clone(),
            None => return Default::default(),
        };
        match func {
            Function::Builtin(name) => match name.as_str() {
                "sin" => self.eval_internal(args[0].clone()).sin(),
                _ => Default::default(),
            },
            Function::UserDefined { params, body } => {
                let new_local = {
                    args.iter()
                        .zip(params.clone())
                        .map(|(a, p)| (p.to_owned(), self.eval_internal(a.clone())))
                        .collect()
                };
                self.local_values.push(new_local);

                for node in &body[..body.len() - 1] {
                    self.eval_internal(node.clone());
                }
                let ret = self.eval_internal(body[body.len() - 1].clone());

                self.local_values.pop();

                ret
            }
        }
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
            FunctionDef { name, params, body } => {
                self.functions
                    .insert(name, Function::UserDefined { params, body });
                Default::default()
            }
            FunctionCall { name, args } => self.eval_function(&name, args),
            Error => panic!("Evaluation of invalid ast"),
        }
    }

    pub fn eval(&mut self, node: Node) -> &Number {
        self.last_result = Some(self.eval_internal(node));
        self.last_result.as_ref().unwrap() // Safe, we just set the value
    }
}
