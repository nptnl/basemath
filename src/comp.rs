//! Complex numbers and trait implementations.

use std::{ops, cmp};
use crate::alg::{exp, ln, real_sqrt};

/// Basic complex number struct, constructed from two `f64`s, one real, one imaginary in form (a + bi).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comp {
    /// The real part of the number.
    pub r: f64,
    /// The imaginary part of the number.
    pub i: f64,
}
impl Comp {
    /// New complex number instance from two `f64`s, real and imaginary.
    pub fn new(r: f64, i: f64) -> Self {
        Self { r, i }
    }
    /// Shortcut for no imaginary part.
    pub fn nre(r: f64) -> Self {
        Self { r, i: 0.0 }
    }
    /// Shortcut for no real part.
    pub fn nim(i: f64) -> Self {
        Self { r: 0.0, i }
    }
    /// Simple complex conjugate, (a + bi) -> (a - bi).
    pub fn conj(self) -> Self {
        Self { r: self.r, i: -self.i }
    }
    /// Uses the complex conjugate to compute the inverse of a `Comp`.
    pub fn inv(self) -> Self {
        let divisor: f64 = 1.0 / (self.r*self.r + self.i*self.i);
        Self {
            r: self.r * divisor,
            i: -self.i * divisor
        }
    }
    /// Cheap square of the magnitude, or absolute value of the number.
    pub fn mag_square(self) -> f64 {
        self.r * self.r + self.i * self.i
    }
    /// More expensive magnitude function, because it uses a `sqrt()` function, but usually nothing to worry about.
    pub fn mag(self) -> f64 {
        real_sqrt(self.r * self.r + self.i * self.i)
    }
    /// Uses exp() and ln() to compute exponentiation for any `Comp`s.
    pub fn pow(self, other: Self) -> Self {
        exp( ln(self) * other )
    }
}

/// Struct for quaternion numbers, using four `f64`s in form (a + bi + cj + dk).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub r: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
}
impl Quat {
    /// New quaternion instance from four `f64`s.
    pub fn new(r: f64, i: f64, j: f64, k: f64) -> Self {
        Self { r, i, j, k }
    }
    /// Shortcut for all imaginary parts == 0.
    pub fn nre(r: f64) -> Self {
        Self { r, i: 0.0, j: 0.0, k: 0.0 }
    }
    /// Simple quaternion conjugate, (a + bi + cj + dk) -> (a - bi  - cj - dk)
    pub fn conj(self) -> Self {
        Self { r: self.r, i: -self.i, j: -self.j, k: -self.k }
    }
    /// Uses a quaternion conjugate to compute the inverse of a quaternion.
    pub fn inv(self) -> Self {
        let divisor: f64 = 1.0 / (self.r*self.r + self.i*self.i + self.j*self.j + self.k*self.k);
        Self {
            r: self.r * divisor,
            i: self.i * divisor,
            j: self.j * divisor,
            k: self.k * divisor,
        }
    }
    /// Cheap square of magnitude, or absolute value of thw quaternion.
    pub fn mag_square(self) -> f64 {
        self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k
    }
    /// More expensive magnitude function, as it uses sqrt(), but usually nothing to worry about.
    pub fn mag(self) -> f64 {
        real_sqrt( self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k )
    }
}

impl cmp::PartialOrd for Comp {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.r.partial_cmp(&other.r)
    }
}
impl ops::Neg for Comp {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            r: -self.r,
            i: -self.i,
        }
    }
}

impl ops::Add<Comp> for Comp {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { r: self.r + other.r, i: self.i + other.i }
    }
}
impl ops::Sub<Comp> for Comp {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { r: self.r - other.r, i: self.i - other.i }
    }
}
impl ops::Mul<Comp> for Comp {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r - self.i * other.i,
            i: self.i * other.r + self.r * other.i
        }
    }
}
impl ops::Div<Comp> for Comp {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl ops::Add<f64> for Comp {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Self {
            r: self.r + other,
            i: self.i,
        }
    }
}
impl ops::Sub<f64> for Comp {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self {
            r: self.r - other,
            i: self.i,
        }
    }
}
impl ops::Mul<f64> for Comp {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            i: self.i * other,
        }
    }
}
impl ops::Div<f64> for Comp {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            r: self.r / other,
            i: self.i / other,
        }
    }
}

impl ops::Add<Comp> for f64 {
    type Output = Comp;
    fn add(self, other: Comp) -> Comp {
        Comp {
            r: self + other.r,
            i: other.i,
        }
    }
}
impl ops::Sub<Comp> for f64 {
    type Output = Comp;
    fn sub(self, other: Comp) -> Comp {
        Comp {
            r: self - other.r,
            i: -other.i,
        }
    }
}
impl ops::Mul<Comp> for f64 {
    type Output = Comp;
    fn mul(self, other: Comp) -> Comp {
        Comp {
            r: self * other.r,
            i: self * other.i,
        }
    }
}
impl ops::Div<Comp> for f64 {
    type Output = Comp;
    fn div(self, other: Comp) -> Comp {
        self * other.inv()
    }
}

impl ops::AddAssign<Comp> for Comp {
    fn add_assign(&mut self, other: Comp) {
        *self = *self + other
    }
}
impl ops::SubAssign<Comp> for Comp {
    fn sub_assign(&mut self, other: Comp) {
        *self = *self - other
    }
}
impl ops::MulAssign<Comp> for Comp {
    fn mul_assign(&mut self, other: Comp) {
        *self = *self * other
    }
}
impl ops::DivAssign<Comp> for Comp {
    fn div_assign(&mut self, other: Comp) {
        *self = *self / other
    }
}

impl ops::AddAssign<f64> for Comp {
    fn add_assign(&mut self, other: f64) {
        *self = *self + other
    }
}
impl ops::SubAssign<f64> for Comp {
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other
    }
}
impl ops::MulAssign<f64> for Comp {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other
    }
}
impl ops::DivAssign<f64> for Comp {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other
    }
}

impl std::str::FromStr for Comp {
    type Err = ();
    fn from_str(slice: &str) -> Result<Self, Self::Err> {
        let mut chlist = slice.chars();
        let last = chlist.clone().count() - 1;
        if chlist.nth(last).unwrap() == 'i' {
            match slice.rfind('+') {
                Some(v) => Ok( Comp {
                    r: slice[..v].parse::<f64>().unwrap(),
                    i: slice[v+1..last].parse::<f64>().unwrap()
                } ),
                None => match slice.rfind('-') {
                    Some(v) => Ok( Comp {
                        r: slice[..v].parse::<f64>().unwrap(),
                        i: -slice[v+1..last].parse::<f64>().unwrap()
                    } ),
                    None => Ok( Comp {
                        r: 0.0,
                        i: slice[..last].parse::<f64>().unwrap()
                    } ),
                },
            }
        } else {
            match slice.parse::<f64>() {
                Ok(v) => Ok(Comp {r: v, i: 0.0 }),
                Err(_) => Err(()),
            }
        }
    }
}
impl std::fmt::Display for Comp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.i < 0.0 {
            write!(f, "{}-{}i", self.r, -self.i)
        } else if self.i > 0.0 {
            write!(f, "{}+{}i", self.r, self.i)
        } else {
            write!(f, "{}", self.r)
        }
    }
}