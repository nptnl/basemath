use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};

use crate::rules::*;

#[derive(Clone, Debug)]
pub struct Poly<T: Arithmetic> {
    pub co: Vec<T>,
    pub le: usize,
}
impl<T: Arithmetic> Poly<T> {
    pub fn new(co: Vec<T>) -> Self {
        let le: usize = co.len();
        Self { co, le }
    }
    pub fn dvt(&self) -> Self {
        if self.le == 0 { return self.clone() };
        let mut result: Vec<T> = Vec::new();
        let mut divisor: T = T::ONE;
        for indx in 1..self.le {
            result.push(self.co[indx] * divisor);
            divisor += T::ONE;
        }
        Self { co: result, le: self.le - 1 }
    }
    pub fn itg(&self, plus_c: T) -> Self {
        let mut result: Vec<T> = Vec::new();
        result.push(plus_c);
        let mut divisor: T = T::ONE;
        for indx in 0..self.le {
            result.push(self.co[indx] / divisor);
            divisor += T::ONE;
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
    pub fn newton(&self, error: T) -> T {
        if self.le == 2 { return -self.co[0] / self.co[1] };
        let (mut s1, mut s2): (T, T) = (T::SEED, T::SEED + T::ONE);
        let slope: Self = self.dvt();
        let mut loop_count: usize = 0;
        while (s1 - s2).mag2() > error {
            if loop_count > 100 {
                s1 += T::SEED;
                s2 += T::SEED + T::ONE;
                loop_count = 0;
            }
            s2 = s1;
            s1 -= self.eval(s2) / slope.eval(s2);
            loop_count += 1;
        }
        s1
    }
    pub fn rootdiv(self, root: T) -> (Self, T) {
        let mut running: T = T::ZERO;
        let mut quotient: Vec<T> = Vec::new();
        for subtract in 0..self.le {
            running = running * root + self.co[self.le-subtract-1];
            quotient.push(running);
        }
        quotient = vec_flip(quotient);
        let remainder: T = quotient.remove(0);
        (Self { co: quotient, le: self.le - 1 }, remainder)
    }
    pub fn solve(self, error: T) -> Vec<T> {
        let mut running: Self = self;
        let mut sols: Vec<T> = Vec::new();
        while running.le > 1 {
            let next_root: T = running.newton(error);
            sols.push(next_root);
            running = running.rootdiv(next_root).0;
        }
        sols
    }
}

fn vec_flip<T: Copy>(original: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let le: usize = original.len();
    for indx in 0..le {
        result.push(original[le-indx-1]);
    }
    result
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