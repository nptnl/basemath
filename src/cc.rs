use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};

pub trait Identity {
    const ZERO: Self;
    const ONE: Self;
}

pub trait Arithmetic: Identity + Copy + Neg<Output = Self> + PartialEq + PartialOrd
+ Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self>
+ AddAssign + SubAssign + MulAssign + DivAssign + RemAssign {}

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
    pub fn mag2(self) -> R {
        self.r * self.r + self.i * self.i
    }
    pub fn conj(self) -> Self {
        Self { r: self.r, i: -self.i }
    }
    pub fn inv(self) -> Self {
        let divisor: R = self.mag2();
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
}

impl Identity for i8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for i16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for i32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for i64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}
impl Identity for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

impl Arithmetic for f32 {}
impl Arithmetic for f64 {}
impl Arithmetic for i8 {}
impl Arithmetic for i16 {}
impl Arithmetic for i32 {}
impl Arithmetic for i64 {}

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
        let divisor: R = rhs.mag2();
        Self {
            r: (self.r * rhs.r - self.i * rhs.i) / divisor,
            i: (self.r * rhs.i + self.i * rhs.r) / divisor,
        }
    }
}
impl<R: Arithmetic> Rem for Comp<R> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let factor: Self = Self::nre((self.r * rhs.r + self.i * rhs.i) / rhs.mag2());
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
        self.mag2().partial_cmp(&rhs.mag2())
    }
}

impl<R: Arithmetic> Arithmetic for Comp<R> {}