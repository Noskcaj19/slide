use rug::float::Round;
use rug::{self, Float as RFloat, Integer};

use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul, Shl, Shr, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(Integer),
    Float(RFloat),
}

impl Default for Number {
    fn default() -> Number {
        Number::Int(Integer::from(0))
    }
}

impl From<RFloat> for Number {
    fn from(x: RFloat) -> Number {
        Number::Float(x)
    }
}

impl From<Integer> for Number {
    fn from(x: Integer) -> Number {
        Number::Int(x)
    }
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

impl rug::ops::Pow<Number> for Number {
    type Output = Self;

    fn pow(self, other: Self) -> Self {
        use self::Number::*;
        match (self, other) {
            // TODO: Correct these, why are there missing implementations?
            (Int(l), Int(r)) => Number::Int(l.pow(r.to_u32().unwrap_or(std::u32::MAX))),
            (Int(l), Float(r)) => Number::Float(RFloat::with_val(
                r.prec(),
                l.pow(
                    r.to_u32_saturating_round(Round::Nearest)
                        .unwrap_or(std::u32::MAX),
                ),
            )),
            (Float(l), Int(r)) => Number::Float(RFloat::with_val(l.prec(), l.pow(r))),
            (Float(l), Float(r)) => {
                Number::Float(RFloat::with_val(l.prec().max(r.prec()), l.pow(r)))
            }
        }
    }
}

impl Shl<Number> for Number {
    type Output = Self;

    fn shl(self, other: Self) -> Self {
        use self::Number::*;
        match (self, other) {
            (Int(l), Int(r)) => Int(l << r.to_u32().unwrap_or(std::u32::MAX)),
            (Int(l), Float(r)) => Number::Float(RFloat::with_val(
                r.prec(),
                l << r
                    .to_u32_saturating_round(Round::Nearest)
                    .unwrap_or(std::u32::MAX),
            )),
            (Float(l), Int(r)) => Number::Float(RFloat::with_val(
                l.prec(),
                l << r.to_u32().unwrap_or(std::u32::MAX),
            )),
            (Float(l), Float(r)) => Number::Float(RFloat::with_val(
                l.prec().max(r.prec()),
                l << r
                    .to_u32_saturating_round(Round::Nearest)
                    .unwrap_or(std::u32::MAX),
            )),
        }
    }
}

impl Shr<Number> for Number {
    type Output = Self;

    fn shr(self, other: Self) -> Self {
        use self::Number::*;
        match (self, other) {
            (Int(l), Int(r)) => Int(l >> r.to_u32().unwrap_or(std::u32::MAX)),
            (Int(l), Float(r)) => Number::Float(RFloat::with_val(
                r.prec(),
                l >> r
                    .to_u32_saturating_round(Round::Nearest)
                    .unwrap_or(std::u32::MAX),
            )),
            (Float(l), Int(r)) => Number::Float(RFloat::with_val(
                l.prec(),
                l >> r.to_u32().unwrap_or(std::u32::MAX),
            )),
            (Float(l), Float(r)) => Number::Float(RFloat::with_val(
                l.prec().max(r.prec()),
                l >> r
                    .to_u32_saturating_round(Round::Nearest)
                    .unwrap_or(std::u32::MAX),
            )),
        }
    }
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
