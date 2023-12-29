use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};

use crate::cc::{Identity, Arithmetic};

#[derive(Clone, Debug)]
pub struct Poly<T: Arithmetic> {
    co: Vec<T>,
    le: usize,
}
impl<T: Arithmetic> Poly<T> {
    pub fn new(co: Vec<T>) -> Self {
        let le: usize = co.len();
        Self { co, le }
    }
    pub fn dvt(&self) -> Self {
        if self.le == 0 { return self.clone() };
        let mut result: Vec<T> = Vec::new();
        let mut indx: T = T::ONE;
        for term in self.co.clone().into_iter().skip(1) {
            result.push(term * indx);
            indx += T::ONE;
        }
        Self { co: result, le: self.le - 1 }
    }
    pub fn itg(&self, plus_c: T) -> Self {
        let mut result: Vec<T> = Vec::new();
        result.push(plus_c);
        let mut indx: T = T::ONE;
        for term in self.co.clone().into_iter() {
            result.push(term / indx);
            indx += T::ONE;
        }
        Self{ co: result, le: self.le + 1 }
    }
    pub fn eval(&self, input: T) -> T {
        let mut total: T = T::ZERO;
        let mut exponent: T = T::ONE;
        for indx in 0..self.le {
            total += self.co[indx] * exponent;
            exponent *= input;
        }
        total
    }
    pub fn newton(&self, seed: T, error: T) -> T {
        let (mut s1, mut s2): (T, T) = (seed, seed + T::ONE);
        let slope: Self = self.dvt();
        while s1 - s2 > error || s2 - s1 > error {
            s2 = s1;
            s1 -= self.eval(s2) / slope.eval(s2);
        }
        s1
    }
}




impl<T: Arithmetic> Neg for Poly<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut result: Vec<T> = self.co;
        for indx in 0..self.le { result[indx] = -result[indx]; }
        Self { co: result, le: self.le }
    }
}
impl<T: Arithmetic> Add for Poly<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result: Vec<T>;
        if self.le > rhs.le {
            result = self.co.clone();
            for term in 0..rhs.le {
                result[term] += rhs.co[term];
            }
        } else {
            result = rhs.co.clone();
            for term in 0..self.le {
                result[term] += self.co[term];
            }
        }
        Poly::new(result)
    }
}
impl<T: Arithmetic> Sub for Poly<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result: Vec<T>;
        if self.le > rhs.le {
            result = self.co.clone();
            for term in 0..rhs.le {
                result[term] -= rhs.co[term];
            }
        } else {
            result = rhs.co.clone();
            for term in 0..self.le {
                result[term] -= self.co[term];
            }
        }
        Poly::new(result)
    }
}
impl<T: Arithmetic> Mul for Poly<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut product: Vec<T> = Vec::new();
        let newlen: usize = self.le + rhs.le - 1;
        for _ in 0..newlen { product.push(T::ZERO); }
        for left in 0..self.le {
            for right in 0..rhs.le {
                product[left+right] += self.co[left] * rhs.co[right];
            }
        }
        Self { co: product, le: newlen }
    }
}