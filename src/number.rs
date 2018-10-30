use rug::{Float as RFloat, Integer};

use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(Integer),
    Float(RFloat),
}

impl Display for Number {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Number::*;
        match *self {
            Int(ref i) => i.fmt(fmt),
            Float(ref f) => f.fmt(fmt),
        }
    }
}

macro_rules! impl_op {
    ($op:ident, $fn:ident, $sym:tt) => {
        impl $op for Number {
            type Output = Self;

            fn $fn(self, other: Self) -> Self {
                use self::Number::*;
                match (self, other) {
                    (Int(l), Int(r)) => Int(l $sym r),
                    (Int(l), Float(r)) => {
                        Number::Float(RFloat::with_val(r.prec(), l $sym r))
                    },
                    (Float(l), Int(r)) => {
                        Number::Float(RFloat::with_val(l.prec(), l $sym r))
                    },
                    (Float(l), Float(r)) => {
                        Number::Float(RFloat::with_val(l.prec().max(r.prec()), l $sym r))
                    }
                }
            }
        }
    };
}

impl Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        use self::Number::*;
        match (self, other) {
            // TODO: Automatic precision?
            (Int(l), Int(r)) => Number::Float(RFloat::with_val(
                53,
                RFloat::with_val(53, l) / RFloat::with_val(53, r),
            )),
            (Int(l), Float(r)) => Number::Float(RFloat::with_val(r.prec(), l / r)),
            (Float(l), Int(r)) => Number::Float(RFloat::with_val(l.prec(), l / r)),
            (Float(l), Float(r)) => Number::Float(RFloat::with_val(l.prec().max(r.prec()), l / r)),
        }
    }
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
