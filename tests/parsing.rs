use crate::slide::rug::{Float as RFloat, Integer};
use crate::slide::{
    ast::{
        Expr::{self, *},
        Number,
        Opcode::*,
    },
    calc,
    eval::EvalContext,
};

fn int(v: isize) -> Number {
    Number::Int(Integer::from(v))
}

fn float(prec: u32, v: f64) -> Number {
    Number::Float(RFloat::with_val(prec, v))
}

fn boxed_int(v: isize) -> Box<Expr> {
    Box::new(Expr::Number(Number::Int(Integer::from(v))))
}

fn boxed_float(prec: u32, v: f64) -> Box<Expr> {
    Box::new(Expr::Number(Number::Float(RFloat::with_val(prec, v))))
}

#[test]
fn add() {
    let mut errors = Vec::new();
    let result = calc::ExprParser::new().parse(&mut errors, "1+ 1").unwrap();
    let mut eval_ctx = EvalContext::new();
    assert_eq!(*eval_ctx.eval(*result.clone()), int(2));
    assert_eq!(*result, Op(boxed_int(1), Add, boxed_int(1)));

    let result = calc::ExprParser::new()
        .parse(&mut errors, "1.25 + 1.75")
        .unwrap();
    assert_eq!(*eval_ctx.eval(*result.clone()), float(53, 3.0));
    assert_eq!(
        *result,
        Op(boxed_float(53, 1.25), Add, boxed_float(53, 1.75))
    );
}

#[test]
fn hex() {
    let mut errors = Vec::new();
    let result = calc::ExprParser::new().parse(&mut errors, "0xFF").unwrap();
    let mut eval_ctx = EvalContext::new();
    assert_eq!(result, boxed_int(255));
    assert_eq!(*eval_ctx.eval(*result), int(255));

    let result = calc::ExprParser::new().parse(&mut errors, "27h").unwrap();
    assert_eq!(result, boxed_int(39));
    assert_eq!(*eval_ctx.eval(*result), int(39));
}

#[test]
fn binary() {
    let mut errors = Vec::new();
    let result = calc::ExprParser::new()
        .parse(&mut errors, "0b1010101")
        .unwrap();
    let mut eval_ctx = EvalContext::new();
    assert_eq!(result, boxed_int(85));
    assert_eq!(*eval_ctx.eval(*result), int(85));

    let result = calc::ExprParser::new()
        .parse(&mut errors, "0b00000011")
        .unwrap();
    assert_eq!(result, boxed_int(3));
    assert_eq!(*eval_ctx.eval(*result), int(3));
}
