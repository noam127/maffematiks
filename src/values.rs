
use crate::values::Value::*;
use std::fmt::{Debug, Formatter, Pointer};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub enum Value {
    Natural(i32),
    Imaginary(i32),
    Complex(i32, i32),
    Undefined
}
impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Natural(n) => write!(f, "{n}"),
            Imaginary(n) => write!(f, "{n}i"),
            Complex(r, i) => write!(f, "{r} + {i}i"),
            Undefined => write!(f, "undefined"),
        }
    }
}

impl Value {
    fn complex(self) -> Self {
        match self {
            Natural(n) => Complex(n, 0),
            Imaginary(n) => Complex(0, n),
            Complex(_, _) => self,
            _ => Undefined,
        }
    }
    fn simple(self) -> Self {
        if let Complex(r, i) = self {
            if i == 0 {
                Natural(r)
            } else if r == 0 {
                Imaginary(i)
            } else {
                self
            }
        } else {
            self
        }
    }
}
impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (left, right) = (self.complex(), rhs.complex());
        match (left, right) {
            (Complex(lr, li), Complex(rr, ri)) => {
                Complex(lr + rr, li + ri).simple()
            }
            _ => Undefined
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let (left, right) = (self.complex(), rhs.complex());
        match (left, right) {
            (Complex(lr, li), Complex(rr, ri)) => {
                Complex(lr - rr, li - ri).simple()
            }
            _ => Undefined
        }
    }
}
impl Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Natural(0).complex() - self
    }
}
impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let (left, right) = (self.complex(), rhs.complex());
        match (left, right) {
            (Complex(lr, li), Complex(rr, ri)) => {
                Complex(lr * rr - li * ri, lr * ri + rr * li).simple()
            }
            _ => Undefined
        }
    }
}
impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        // to be upgraded

        let (left, right) = (self.complex(), rhs);
        match (left, right) {
            (Complex(real, imaginary), Natural(n)) => {
                if n == 0 { return Undefined }
                Complex(real / n, imaginary / n).simple()
            }
            _ => Undefined
        }
    }
}