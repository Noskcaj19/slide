use std::fmt::{Display, Error, Formatter};

use calc;
pub type TErrorRecovery<'input> =
    lalrpop_util::ErrorRecovery<usize, calc::Token<'input>, &'static str>;
pub type TParseError<'input> = lalrpop_util::ParseError<usize, calc::Token<'input>, &'static str>;

pub use number::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Prev,
    Ident(String),
    Number(Number),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
    Shl,
    Shr,
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Prev => write!(fmt, "#"),
            Ident(ref n) => write!(fmt, "{}", n),
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
            Pow => write!(fmt, "**"),
            Shl => write!(fmt, "<<"),
            Shr => write!(fmt, ">>"),
        }
    }
}
