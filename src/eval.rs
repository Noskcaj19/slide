use std::collections::HashMap;
use std::f64::consts::PI;

use rug::ops::Pow;

use crate::ast::{Node, Number};

struct Function {
    args: Vec<String>,
    body: Vec<Node>,
}

pub struct EvalContext {
    pub last_result: Option<Number>,
    values: HashMap<String, Number>,
    functions: HashMap<String, Function>,
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

    fn eval_internal(&mut self, node: Node) -> Number {
        use crate::ast::Node::*;
        match node {
            Prev => self.last_result.clone().unwrap_or_default(),
            Ident(key) => self.values.get(&key).cloned().unwrap_or_default(),
            Number(num) => num,
            Error => panic!("Evaluation of invalid ast"),
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
        }
    }

    pub fn eval(&mut self, node: Node) -> &Number {
        self.last_result = Some(self.eval_internal(node));
        self.last_result.as_ref().unwrap() // Safe, we just set the value
    }
}
