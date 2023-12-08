use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};

use crate::cc::{Arithmetic, Identity};

#[derive(Debug, Clone, Copy)]
pub struct Rat<R: Arithmetic> {
    pub n: R,
    pub d: R,
}
impl<R: Arithmetic> Rat<R> {
    pub fn raw(n: R, d: R) -> Self {
        Self { n, d }
    }
    pub fn new(n: R, d: R) -> Self {
        let (mut n, mut d): (R, R) = (n, d);
        let mut positive: bool = true;
        if n < R::zero() { positive = !positive; n = -n; }
        if d < R::zero() { positive = !positive; d = -d; }
        let factor: R = gcf(n, d);
        n /= factor;
        d /= factor;
        if !positive { n = -n; }
        Self { n, d }
    }
    pub fn whole(n: R) -> Self {
        Self { n, d: R::one() }
    }
}

fn gcf<R: Arithmetic>(inp1: R, inp2: R) -> R {
    let (mut n1, mut n2): (R, R) = (inp1, inp2);
    if n1 < R::zero() || n2 < R::zero() { panic!("cannot GCF negative numbers") };
    loop {
        if n1 == R::zero() { return n2 };
        if n2 == R::zero() { return n1 };
        if n1 > n2 { n1 %= n2; }
        else if n2 > n1 { n2 %= n1; }
        else { return n1 };
    }
}

impl<R: Arithmetic> PartialEq for Rat<R> {
    fn eq(&self, rhs: &Self) -> bool {
        self.n * rhs.d == self.d * rhs.n
    }
}
impl<R: Arithmetic> PartialOrd for Rat<R> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        let quantity: R = self.d * rhs.n - self.n * rhs.d;
        quantity.partial_cmp(&R::zero())
    }
}
impl<R: Arithmetic> Identity for Rat<R> {
    fn zero() -> Self {
        Self { n: R::zero(), d: R::one() }
    }
    fn one() -> Self {
        Self { n: R::one(), d: R::one() }
    }
}
impl<R: Arithmetic + std::fmt::Display> std::fmt::Display for Rat<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}/{})", self.n, self.d)
    }
}




impl<R: Arithmetic> Neg for Rat<R> {
    type Output = Self;
    fn neg(self) -> Self {
        Self { n: -self.n, d: self.d }
    }
}
impl<R: Arithmetic> Add for Rat<R> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.n * rhs.d + rhs.n * self.d,
            self.d * rhs.d
        )
    }
}
impl<R: Arithmetic> Sub for Rat<R> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.n * rhs.d - rhs.n * self.d,
            self.d * rhs.d
        )
    }
}
impl<R: Arithmetic> Mul for Rat<R> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.n * rhs.n, self.d * rhs.d)
    }
}
impl<R: Arithmetic> Div for Rat<R> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self::new(self.n * rhs.d, self.d * rhs.n)
    }
}
impl<R: Arithmetic> Rem for Rat<R> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        Self::new(
            (self.n * rhs.d) % (self.d * rhs.n),
            self.d * rhs.d,
        )
    }
}
impl<R: Arithmetic> AddAssign for Rat<R> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<R: Arithmetic> SubAssign for Rat<R> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<R: Arithmetic> MulAssign for Rat<R> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<R: Arithmetic> DivAssign for Rat<R> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl<R: Arithmetic> RemAssign for Rat<R> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}