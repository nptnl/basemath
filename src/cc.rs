use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};
use crate::rules::*;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Comp<R: RealArithmetic> {
    pub r: R,
    pub i: R,
}
impl<R: RealArithmetic> Comp<R> {
    pub fn new(r: R, i: R) -> Self {
        Self { r, i }
    }
    pub fn nre(r: R) -> Self {
        Self { r, i: R::ZERO }
    }
    pub fn nim(i: R) -> Self {
        Self { r: R::ZERO, i }
    }
    pub fn conj(self) -> Self {
        Self { r: self.r, i: -self.i }
    }
}

#[allow(non_camel_case_types)]
pub type c32 = Comp<f32>;
#[allow(non_camel_case_types)]
pub type c64 = Comp<f64>;
#[allow(non_camel_case_types)]
pub type g8 = Comp<i8>;
#[allow(non_camel_case_types)]
pub type g16 = Comp<i16>;
#[allow(non_camel_case_types)]
pub type g32 = Comp<i32>;
#[allow(non_camel_case_types)]
pub type g64 = Comp<i64>;

impl<R: RealArithmetic> Neg for Comp<R> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { r: -self.r, i: -self.i }
    }
}
impl<R: RealArithmetic> Add for Comp<R> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { r: self.r + rhs.r, i: self.i + rhs.i }
    }
}
impl<R: RealArithmetic> Sub for Comp<R> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { r: self.r - rhs.r, i: self.i - rhs.i }
    }
}
impl<R: RealArithmetic> Mul for Comp<R> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.i * rhs.r + self.r * rhs.i,
        }
    }
}
impl<R: RealArithmetic> Div for Comp<R> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let divisor: R = rhs.r * rhs.r + rhs.i * rhs.i;
        Self {
            r: (self.r * rhs.r + self.i * rhs.i) / divisor,
            i: (self.i * rhs.r - self.r * rhs.i) / divisor,
        }
    }
}
impl<R: RealArithmetic> Rem for Comp<R> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let factor: Self = Self::nre((self.r * rhs.r + self.i * rhs.i) / rhs.mag2().r);
        self - rhs * factor
    }
}
impl<R: RealArithmetic> AddAssign for Comp<R> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<R: RealArithmetic> SubAssign for Comp<R> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<R: RealArithmetic> MulAssign for Comp<R> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<R: RealArithmetic> DivAssign for Comp<R> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl<R: RealArithmetic> RemAssign for Comp<R> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<R: RealArithmetic> PartialEq for Comp<R> {
    fn eq(&self, rhs: &Self) -> bool {
        self.r == rhs.r && self.i == rhs.i
    }
}
impl<R: RealArithmetic> PartialOrd for Comp<R> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.mag2().r.partial_cmp(&rhs.mag2().r)
    }
}

impl<R: RealArithmetic + fmt::Display> fmt::Display for Comp<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.i < R::ZERO {
            write!(f, "{}-{}i", self.r, -self.i)
        } else {
            write!(f, "{}+{}i", self.r, self.i)
        }
    }
}
impl<R: RealArithmetic> Identity for Comp<R> {
    const ZERO: Self = Self { r: R::ZERO, i: R::ZERO };
    const ONE: Self = Self { r: R::ONE, i: R::ZERO };
    const SEED: Self = Self { r: R::ONE, i: R::ONE };
} 
impl<R: RealArithmetic> RealArithmetic for Comp<R> {}
impl<R: RealArithmetic> Inverse for Comp<R> {
    fn inv(self) -> Self {
        let divisor: R = self.r * self.r + self.i * self.i;
        Self {
            r: self.r / divisor,
            i: -self.i / divisor,
        }
    }
}
impl<R: RealArithmetic + PowersOfTen> PowersOfTen for Comp<R> {
    fn order_of(power: isize) -> Self {
        Self { r: R::order_of(power), i: R::ZERO }
    }
}
impl<R: RealArithmetic + PowersOfE> PowersOfE for Comp<R> {
    fn etothe(power: isize) -> Self {
        Self { r: R::etothe(power), i: R::ZERO }
    }
}
impl<R: RealArithmetic + UsefulReals> UsefulReals for Comp<R> {
    const TWO: Self = Comp { r: R::TWO, i: R::ZERO };
    const E: Self = Comp { r: R::E, i: R::ZERO };
    const TAU: Self = Comp { r: R::TAU, i: R::ZERO };
    const PI: Self = Comp { r: R::PI, i: R::ZERO };
    const HALFPI: Self = Comp { r: R::HALFPI, i: R::ZERO };
    const QTRPI: Self = Comp { r: R::QTRPI, i: R::ZERO };
}
impl<R: RealArithmetic> MagSquare for Comp<R> {
    fn mag2(self) -> Self {
        Self { r: self.r * self.r + self.i * self.i, i: R::ZERO }
    }
}
impl<R: Reals> Magnitude for Comp<R> {}
impl<R: Reals> Reals for Comp<R> {}