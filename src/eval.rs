use std::collections::HashMap;
use std::f64::consts::PI;

use rug::ops::Pow;

use crate::ast::{Node, Number};

macro_rules! builtin_fns {
    ($($func:tt),*) => {
        vec![
            $(
                ($func.to_string(), Function::Builtin($func.to_string())),
            )*
        ]
    };
}

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
        let functions =
            builtin_fns!("sin", "cos", "tan", "asin", "acos", "atan", "csc", "sec", "cot")
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
            None => {
                // Really hacky
                if let Number::Int(ptr) = self.lookup_ident(name) {
                    if ptr > 0xFFFF {
                        let fn_ptr =
                            ptr.to_usize().expect("Pointer deref too big") as *const Function;
                        unsafe { (&*fn_ptr).clone() }
                    } else {
                        return Default::default();
                    }
                } else {
                    return Default::default();
                }
            }
        };
        match func {
            Function::Builtin(name) => match name.as_str() {
                "sin" => self.eval_internal(args[0].clone()).sin(),
                "cos" => self.eval_internal(args[0].clone()).cos(),
                "tan" => self.eval_internal(args[0].clone()).tan(),
                "asin" => self.eval_internal(args[0].clone()).asin(),
                "acos" => self.eval_internal(args[0].clone()).acos(),
                "atan" => self.eval_internal(args[0].clone()).atan(),
                "csc" => self.eval_internal(args[0].clone()).csc(),
                "sec" => self.eval_internal(args[0].clone()).sec(),
                "cot" => self.eval_internal(args[0].clone()).cot(),
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
            Prev => self.last_result.take().unwrap_or_default(),
            Ident(key) => self.lookup_ident(&key),
            Number(num) => num,
            String(str) => self::Number::Int(str.bytes().map(usize::from).sum::<usize>().into()),
            Infix { lhs, op, rhs } => self.eval_op(*lhs, op, *rhs),
            Let(key, node) => {
                let value = self.eval_internal(*node);
                self.values.insert(key, value.clone());
                value
            }
            FunctionDef { name, params, body } => {
                let func = self
                    .functions
                    .entry(name)
                    .or_insert(Function::UserDefined { params, body });
                self::Number::Int(rug::Integer::from(func as *const Function as usize))
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
