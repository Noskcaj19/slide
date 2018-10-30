use rug::{Float, Integer};

use std::fmt::{Display, Error, Formatter};

use calc;
pub type TErrorRecovery<'input> =
    lalrpop_util::ErrorRecovery<usize, calc::Token<'input>, &'static str>;
pub type TParseError<'input> = lalrpop_util::ParseError<usize, calc::Token<'input>, &'static str>;

#[derive(Debug)]
pub enum Number {
    Int(Integer),
    Float(Float),
}

#[derive(Debug)]
pub enum Expr {
    Number(Number),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(ref n) => write!(fmt, "{:?}", n),
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "[error]"),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
