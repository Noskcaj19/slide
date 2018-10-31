use ast;

use rug::ops::Pow;

fn eval_op(lh: ast::Expr, op: ast::Opcode, rh: ast::Expr) -> ast::Number {
    use ast::Opcode::*;
    match op {
        Add => eval(lh) + eval(rh),
        Sub => eval(lh) - eval(rh),
        Mul => eval(lh) * eval(rh),
        Div => eval(lh) / eval(rh),
        Pow => eval(lh).pow(eval(rh)),
    }
}

pub fn eval(expr: ast::Expr) -> ast::Number {
    use ast::Expr::*;
    match expr {
        Number(num) => num,
        Error => panic!("Error handling not yet implemented"),
        Op(l, o, r) => eval_op(*l.clone(), o, *r.clone()),
    }
}
