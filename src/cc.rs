use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};
use crate::rules::*;

#[derive(Clone, Copy, Debug)]
pub struct Comp<R: Arithmetic> {
    pub r: R,
    pub i: R,
}
impl<R: Arithmetic> Comp<R> {
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
    pub fn inv(self) -> Self {
        let divisor: R = self.r * self.r + self.i * self.i;
        Self {
            r: self.r / divisor,
            i: self.i / divisor
        }
    }
}

impl<R> std::fmt::Display for Comp<R>
where R: Arithmetic + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.i < R::ZERO {
            write!(f, "{}-{}i", self.r, -self.i)
        } else {
            write!(f, "{}+{}i", self.r, self.i)
        }
    }
}
impl<R: Arithmetic> Identity for Comp<R> {
    const ZERO: Self = Self { r: R::ZERO, i: R::ZERO, };
    const ONE: Self = Self { r: R::ONE, i: R::ZERO };
    const SEED: Self = Self { r: R::ONE, i: R::ONE };
}
impl<R: Arithmetic> PowersOfTen for Comp<R> {
    fn order_of(power: isize) -> Self {
        Self { r: R::order_of(power), i: R::ZERO }
    }
}
impl<R: Arithmetic> Magnitude for Comp<R> {
    fn mag2(self) -> Self {
        Self::nre(self.r * self.r + self.i * self.i)
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



impl<R: Arithmetic> Neg for Comp<R> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { r: -self.r, i: -self.i }
    }
}
impl<R: Arithmetic> Add for Comp<R> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { r: self.r + rhs.r, i: self.i + rhs.i }
    }
}
impl<R: Arithmetic> Sub for Comp<R> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { r: self.r - rhs.r, i: self.i - rhs.i }
    }
}
impl<R: Arithmetic> Mul for Comp<R> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.i * rhs.r + self.r * rhs.i,
        }
    }
}
impl<R: Arithmetic> Div for Comp<R> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let divisor: R = rhs.r * rhs.r + rhs.i * rhs.i;
        Self {
            r: (self.r * rhs.r - self.i * rhs.i) / divisor,
            i: (self.r * rhs.i + self.i * rhs.r) / divisor,
        }
    }
}
impl<R: Arithmetic> Rem for Comp<R> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let factor: Self = Self::nre((self.r * rhs.r + self.i * rhs.i) / rhs.mag2().r);
        self - rhs * factor
    }
}
impl<R: Arithmetic> AddAssign for Comp<R> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<R: Arithmetic> SubAssign for Comp<R> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<R: Arithmetic> MulAssign for Comp<R> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<R: Arithmetic> DivAssign for Comp<R> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl<R: Arithmetic> RemAssign for Comp<R> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<R: Arithmetic> PartialEq for Comp<R> {
    fn eq(&self, rhs: &Self) -> bool {
        self.r == rhs.r && self.i == rhs.i
    }
}
impl<R: Arithmetic> PartialOrd for Comp<R> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.mag2().r.partial_cmp(&rhs.mag2().r)
    }
}

impl<R: Arithmetic> Arithmetic for Comp<R> {}