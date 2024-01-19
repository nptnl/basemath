use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};
use crate::rules::*;
use std::fmt;

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
        if n < R::ZERO { positive = !positive; n = -n; }
        if d < R::ZERO { positive = !positive; d = -d; }
        let factor: R = gcf(n, d);
        n /= factor;
        d /= factor;
        if !positive { n = -n; }
        Self { n, d }
    }
    pub fn whole(n: R) -> Self {
        Self { n, d: R::ONE }
    }
}

fn gcf<R: Arithmetic>(inp1: R, inp2: R) -> R {
    let (mut n1, mut n2): (R, R) = (inp1, inp2);
    if n1 < R::ZERO || n2 < R::ZERO { panic!("cannot GCF negative numbers") };
    loop {
        if n1 == R::ZERO { return n2 };
        if n2 == R::ZERO { return n1 };
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
        let quantity: R = self.n * rhs.d - self.d * rhs.n;
        quantity.partial_cmp(&R::ZERO)
    }
}
impl<R: Arithmetic> Identity for Rat<R> {
    const ZERO: Self = Self { n: R::ZERO, d: R::ONE };
    const ONE: Self = Self { n: R::ONE, d: R::ONE };
}
impl<R: Arithmetic> Inverse for Rat<R> {
    fn inv(self) -> Self {
        Self::new(self.d, self.n)
    }
}
impl<R: Arithmetic + PowersOfTen> PowersOfTen for Rat<R> {
    fn order_of(power: isize) -> Self {
        if power < 0 {
            Self { n: R::ONE, d: R::order_of(-power) }
        } else {
            Self { n: R::order_of(power), d: R::ONE }
        }
    }
}
impl<R: Arithmetic> Magnitude for Rat<R> {}
impl<R: Arithmetic> Arithmetic for Rat<R> {}
impl<R: Arithmetic + fmt::Display> fmt::Display for Rat<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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