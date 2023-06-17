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
    /// easy z * z
    pub fn square(self) -> Self {
        self * self
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
    /// Shortcut for only real and i parts.
    pub fn nco(r: f64, i: f64) -> Self {
        Self { r, i, j: 0.0, k: 0.0 }
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
    /// easy h * h
    pub fn square(self) -> Self {
        self * self
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
impl cmp::PartialOrd for Quat {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.r.partial_cmp(&other.r)
    }
}
impl ops::Neg for Quat {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            r: -self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

impl std::str::FromStr for Comp {
    type Err = ();
    fn from_str(slice: &str) -> Result<Self, Self::Err> {
        let last: usize = slice.len() - 1;
        if let Ok(r) = slice.parse::<f64>() {
            return Ok(Comp { r, i: 0.0 })
        }
        if let Ok(i) = slice[..last].parse::<f64>() {
        if &slice[last..] == "i" {
            return Ok(Comp { r: 0.0, i })
        }
        }
        if let Some(v) = slice.rfind('+') {
            if let (Ok(r), Ok(i)) = (slice[..v].parse::<f64>(), slice[v+1..last].parse::<f64>()) {
                return Ok(Comp { r, i });
            }
        }
        if let Some(v) = slice.rfind('-') {
            if let (Ok(r), Ok(i)) = (slice[..v].parse::<f64>(), slice[v..last].parse::<f64>()) {
                return Ok(Comp { r, i });
            }
        }
        Err(())
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

fn get_last(slice: &str, last: usize) -> (Result<f64, ()>, usize) {
    if let Some(indx) = slice.rfind('+') {
        match slice[indx+1..last].parse::<f64>() {
            Err(_) => return (Err(()), last),
            Ok(v) => return (Ok(v), indx),
        }
    }
    if let Some(indx) = slice.rfind('-') {
        match slice[indx..last].parse::<f64>() {
            Err(_) => return (Err(()), last),
            Ok(v) => return (Ok(v), indx),
        }
    }
    (Err(()), last)
}
impl std::str::FromStr for Quat {
    type Err = ();
    fn from_str(slice: &str) -> Result<Self, Self::Err> {
        let mut running: &str = slice;
        let mut out: Quat = Quat { r: 0.0, i: 0.0, j: 0.0, k: 0.0 };
        let mut last: usize = slice.len() - 1;
        loop {
            if let Ok(v) = running.parse::<f64>() {
                out.r = v; return Ok(out);
            }
            let (potval, indx): (Result<f64, ()>, usize) = get_last(slice, last);
            match &running[last..] {
                "i" => {
                    match potval {
                        Ok(v) => { out.i = v; },
                        _ => return Err(()),
                    }
                },
                "j" => {
                    match potval {
                        Ok(v) => { out.j = v; },
                        _ => return Err(()),
                    }
                },
                "k" => {
                    match potval {
                        Ok(v) => { out.k = v; },
                        _ => return Err(()),
                    }
                },
                _ => {return Err(())},
            }
            running = &running[..indx];
            last = indx - 1;
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
impl ops::Add<Quat> for f64 {
    type Output = Quat;
    fn add(self, other: Quat) -> Quat {
        Quat {
            r: self + other.r,
            i: other.i,
            j: other.j,
            k: other.k,
        }
    }
}
impl ops::Sub<Quat> for f64 {
    type Output = Quat;
    fn sub(self, other: Quat) -> Quat {
        Quat {
            r: self - other.r,
            i: -other.i,
            j: -other.j,
            k: -other.k,
        }
    }
}
impl ops::Mul<Quat> for f64 {
    type Output = Quat;
    fn mul(self, other: Quat) -> Quat {
        Quat {
            r: self * other.r,
            i: self * other.i,
            j: self * other.j,
            k: self * other.k,
        }
    }
}
impl ops::Div<Quat> for f64 {
    type Output = Quat;
    fn div(self, other: Quat) -> Quat {
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
impl ops::Add<Quat> for Comp {
    type Output = Quat;
    fn add(self, other: Quat) -> Quat {
        Quat {
            r: self.r + other.r,
            i: self.i + other.i,
            j: other.j,
            k: other.k,
        }
    }
}
impl ops::Sub<Quat> for Comp {
    type Output = Quat;
    fn sub(self, other: Quat) -> Quat {
        Quat {
            r: self.r - other.r,
            i: self.i - other.i,
            j: -other.j,
            k: -other.k,
        }
    }
}
impl ops::Mul<Quat> for Comp {
    type Output = Quat;
    fn mul(self, other: Quat) -> Quat {
        Quat {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
            j: self.r * other.j - self.i * other.k,
            k: self.r * other.k + self.i * other.j,
        }
    }
}
impl ops::Div<Quat> for Comp {
    type Output = Quat;
    fn div(self, other: Quat) -> Quat {
        self * other.inv()
    }
}

impl ops::Add<f64> for Quat {
    type Output = Quat;
    fn add(self, other: f64) -> Quat {
        Quat {
            r: self.r + other,
            i: self.i,
            j: self.j,
            k: self.k,
        }
    }
}
impl ops::Sub<f64> for Quat {
    type Output = Quat;
    fn sub(self, other: f64) -> Quat {
        Quat {
            r: self.r - other,
            i: self.i,
            j: self.j,
            k: self.k,
        }
    }
}
impl ops::Mul<f64> for Quat {
    type Output = Quat;
    fn mul(self, other: f64) -> Quat {
        Quat {
            r: self.r * other,
            i: self.r * other,
            j: self.j * other,
            k: self.k * other,
        }
    }
}
impl ops::Div<f64> for Quat {
    type Output = Quat;
    fn div(self, other: f64) -> Quat {
        self * (1.0 / other)
    }
}
impl ops::Add<Comp> for Quat {
    type Output = Quat;
    fn add(self, other: Comp) -> Quat {
        Quat {
            r: self.r + other.r,
            i: self.i + other.i,
            j: self.j,
            k: self.k,
        }
    }
}
impl ops::Sub<Comp> for Quat {
    type Output = Quat;
    fn sub(self, other: Comp) -> Quat {
        Quat {
            r: self.r - other.r,
            i: self.i - other.i,
            j: self.j,
            k: self.k,
        }
    }
}
impl ops::Mul<Comp> for Quat {
    type Output = Quat;
    fn mul(self, other: Comp) -> Quat {
        Quat {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
            j: self.j * other.r + self.k * other.i,
            k: self.k * other.r - self.j * other.i,
        }
    }
}
impl ops::Div<Comp> for Quat {
    type Output = Quat;
    fn div(self, other: Comp) -> Quat {
        self * other.inv()
    }
}
impl ops::Add<Quat> for Quat {
    type Output = Quat;
    fn add(self, other: Quat) -> Quat {
        Quat {
            r: self.r + other.r,
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}
impl ops::Sub<Quat> for Quat {
    type Output = Quat;
    fn sub(self, other: Quat) -> Quat {
        Quat {
            r: self.r - other.r,
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}
impl ops::Mul<Quat> for Quat {
    type Output = Quat;
    fn mul(self, other: Quat) -> Quat {
        Quat {
            r: self.r * other.r - self.i * other.i - self.j * other.j - self.k * other.k,
            i: self.r * other.i + self.i * other.r + self.j * other.k - self.k * other.j,
            j: self.r * other.j + self.j * other.r + self.k * other.i - self.i * other.k,
            k: self.r * other.k + self.k * other.r + self.i * other.j - self.j * other.i,
        }
    }
}
impl ops::Div<Quat> for Quat {
    type Output = Quat;
    fn div(self, other: Quat) -> Quat {
        self * other.inv()
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

impl ops::AddAssign<f64> for Quat {
    fn add_assign(&mut self, other: f64) {
        *self = *self + other
    }
}
impl ops::SubAssign<f64> for Quat {
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other
    }
}
impl ops::MulAssign<f64> for Quat {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other
    }
}
impl ops::DivAssign<f64> for Quat {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other
    }
}
impl ops::AddAssign<Comp> for Quat {
    fn add_assign(&mut self, other: Comp) {
        *self = *self + other
    }
}
impl ops::SubAssign<Comp> for Quat {
    fn sub_assign(&mut self, other: Comp) {
        *self = *self - other
    }
}
impl ops::MulAssign<Comp> for Quat {
    fn mul_assign(&mut self, other: Comp) {
        *self = *self * other
    }
}
impl ops::DivAssign<Comp> for Quat {
    fn div_assign(&mut self, other: Comp) {
        *self = *self / other
    }
}
impl ops::AddAssign<Quat> for Quat {
    fn add_assign(&mut self, other: Quat) {
        *self = *self + other
    }
}
impl ops::SubAssign<Quat> for Quat {
    fn sub_assign(&mut self, other: Quat) {
        *self = *self - other
    }
}
impl ops::MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, other: Quat) {
        *self = *self * other
    }
}
impl ops::DivAssign<Quat> for Quat {
    fn div_assign(&mut self, other: Quat) {
        *self = *self / other
    }
}
