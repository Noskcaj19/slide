use rug::{Float as RFloat, Integer};

use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
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
                    (Int(i1), Int(i2)) => Int(i1 $sym i2),
                    (Int(i1), Float(f1)) | (Float(f1), Int(i1)) => {
                        Number::Float(RFloat::with_val(f1.prec(), i1 $sym f1))
                    }
                    (Float(f1), Float(f2)) => {
                        Number::Float(RFloat::with_val(f1.prec().max(f2.prec()), f1 $sym f2))
                    }
                }
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);
