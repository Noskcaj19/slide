use ast;

use rug::ops::Pow;

pub struct EvalContext {
    pub last_result: Option<ast::Number>,
}

impl EvalContext {
    pub fn new() -> EvalContext {
        EvalContext { last_result: None }
    }

    fn eval_op(&mut self, lh: ast::Expr, op: ast::Opcode, rh: ast::Expr) -> ast::Number {
        use ast::Opcode::*;
        match op {
            Add => self.eval_internal(lh) + self.eval_internal(rh),
            Sub => self.eval_internal(lh) - self.eval_internal(rh),
            Mul => self.eval_internal(lh) * self.eval_internal(rh),
            Div => self.eval_internal(lh) / self.eval_internal(rh),
            Pow => self.eval_internal(lh).pow(self.eval_internal(rh)),
            Shl => self.eval_internal(lh) << self.eval_internal(rh),
            Shr => self.eval_internal(lh) >> self.eval_internal(rh),
        }
    }

    fn eval_internal(&mut self, expr: ast::Expr) -> ast::Number {
        use ast::Expr::*;
        match expr {
            Prev => self.last_result.clone().unwrap(), // TODO: Remove unwrap once expressions are failible
            Number(num) => num,
            Error => panic!("Error handling not yet implemented"),
            Op(l, o, r) => self.eval_op(*l.clone(), o, *r.clone()),
        }
    }

    pub fn eval(&mut self, expr: ast::Expr) -> &ast::Number {
        self.last_result = Some(self.eval_internal(expr));
        return self.last_result.as_ref().unwrap(); // Safe, we just set the value
    }
}
