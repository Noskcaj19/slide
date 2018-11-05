use rug::{Float as RFloat, Integer};
use slide::{
    ast::{
        self,
        Node::{self, *},
        Number,
    },
    eval::EvalContext,
    token,
};

fn int(v: isize) -> Number {
    Number::Int(Integer::from(v))
}

fn float(prec: u32, v: f64) -> Number {
    Number::Float(RFloat::with_val(prec, v))
}

fn wrapped_int(v: isize) -> Node {
    Node::Number(int(v))
}

fn wrapped_float(prec: u32, v: f64) -> Node {
    Node::Number(float(prec, v))
}

fn boxed_int(v: isize) -> Box<Node> {
    Box::new(wrapped_int(v))
}

fn boxed_float(prec: u32, v: f64) -> Box<Node> {
    Box::new(wrapped_float(prec, v))
}

fn parse_str(input: &str) -> Node {
    let mut errors = Vec::new();
    let tokens = token::tokenize(input).unwrap();
    let mut lalr_tokens = Vec::new();
    for token in tokens {
        lalr_tokens.push(Ok((token.1.start as usize, token.0, token.1.end as usize)))
    }
    ast::parse_single(&mut errors, lalr_tokens).unwrap()
}

#[test]
fn add() {
    let result = parse_str("1+ 1");
    let mut eval_ctx = EvalContext::new();
    assert_eq!(*eval_ctx.eval(result.clone()), int(2));
    assert_eq!(
        result,
        Infix {
            lhs: boxed_int(1),
            op: "+".to_string(),
            rhs: boxed_int(1)
        }
    );

    let result = parse_str("1.25 + 1.75");
    assert_eq!(*eval_ctx.eval(result.clone()), float(53, 3.0));
    assert_eq!(
        result,
        Infix {
            lhs: boxed_float(53, 1.25),
            op: "+".to_string(),
            rhs: boxed_float(53, 1.75)
        }
    );
}

#[test]
fn hex() {
    let result = parse_str("0xFF");
    let mut eval_ctx = EvalContext::new();
    assert_eq!(result, wrapped_int(255));
    assert_eq!(*eval_ctx.eval(result), int(255));

    let result = parse_str("27h");
    assert_eq!(result, wrapped_int(39));
    assert_eq!(*eval_ctx.eval(result), int(39));
}

#[test]
fn binary() {
    let result = parse_str("0b1010101");
    let mut eval_ctx = EvalContext::new();
    assert_eq!(result, wrapped_int(85));
    assert_eq!(*eval_ctx.eval(result), int(85));

    let result = parse_str("0b00000011");
    assert_eq!(result, wrapped_int(3));
    assert_eq!(*eval_ctx.eval(result), int(3));
}
