use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};

pub trait Identity: Copy {
    const ZERO: Self;
    const ONE: Self;
    const SEED: Self = Self::ONE;
}
pub trait UsefulReals: RealArithmetic {
    const TWO: Self;
    const E: Self;
    const TAU: Self;
    const PI: Self;
    const HALFPI: Self;
    const QTRPI: Self;
}
pub trait Inverse {
    fn inv(self) -> Self;
}
pub trait PowersOfTen: Identity {
    fn order_of(power: isize) -> Self;
}
pub trait PowersOfE:
UsefulReals + Inverse + Identity
+ MulAssign + Mul<Output = Self>
{
    fn etothe(power: isize) -> Self {
        let mut running: Self = Self::ONE;
        for _ in 0..power { running *= Self::E; }
        for _ in power..0 { running *= Self::E; }
        if power < 0 { running.inv() } else { running }
    }
}
pub trait MagSquare: Identity + Mul<Output = Self> {
    fn mag2(self) -> Self { self * self }
}
pub trait Magnitude: RealArithmetic + MagSquare + UsefulReals {
    fn rrt(self, error: Self) -> Self {
        let (mut t1, mut t2): (Self, Self) = (Self::SEED, Self::SEED + Self::ONE);
        while (t2 - t1).mag2() > error {
            t1 = t2;
            t2 -= (t2*t2 - self) / (Self::TWO * t2);
        }
        t2
    }
    fn mag1(self, error: Self) -> Self { self.mag2().rrt(error) }
}

pub trait RealArithmetic:
  Identity
+ MagSquare
+ Copy
+ Neg<Output = Self>
+ PartialEq
+ PartialOrd
+ Add<Output = Self>
+ Sub<Output = Self>
+ Mul<Output = Self>
+ Div<Output = Self>
+ Rem<Output = Self>
+ AddAssign
+ SubAssign
+ MulAssign
+ DivAssign
+ RemAssign
{}
pub trait Reals: 
  RealArithmetic
+ Inverse
+ Magnitude
+ PowersOfTen
+ PowersOfE
+ UsefulReals
{}

impl Identity for u8 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for u16 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}
impl Identity for usize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
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
impl Identity for isize {
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

impl MagSquare for i8 {}
impl MagSquare for i16 {}
impl MagSquare for i32 {}
impl MagSquare for i64 {}
impl MagSquare for isize {}
impl MagSquare for f32 {}
impl MagSquare for f64 {}

impl RealArithmetic for i8 {}
impl RealArithmetic for i16 {}
impl RealArithmetic for i32 {}
impl RealArithmetic for i64 {}
impl RealArithmetic for isize {}
impl RealArithmetic for f32 {}
impl RealArithmetic for f64 {}

impl PowersOfTen for u8 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for u16 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for u32 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for u64 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for usize {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for i8 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for i16 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for i32 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for i64 {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for isize {
    fn order_of(power: isize) -> Self {
        if power < 0 { return 0 };
        let mut running: Self = 1;
        for _ in 0..power as usize {
            running *= 10;
        }
        running
    }
}
impl PowersOfTen for f32 {
    fn order_of(power: isize) -> Self {
        let mut running: Self = 1.0;
        if power < 0 {
            for _ in power..0 { running *= 0.1; }
        } else {
            for _ in 0..power { running *= 10.0; }
        }
        running
    }
}
impl PowersOfTen for f64 {
    fn order_of(power: isize) -> Self {
        let mut running: Self = 1.0;
        if power < 0 {
            for _ in power..0 { running *= 0.1; }
        } else {
            for _ in 0..power { running *= 10.0; }
        }
        running
    }
}

impl UsefulReals for f32 {
    const TWO: Self = 2.0;
    const E: Self = 2.718281828459045;
    const TAU: Self = 6.283185307179586;
    const PI: Self = 3.141592653589793;
    const HALFPI: Self = 1.5707963267948966;
    const QTRPI: Self = 0.7853981633974483;
}
impl UsefulReals for f64 {
    const TWO: Self = 2.0;
    const E: Self = 2.718281828459045;
    const TAU: Self = 6.283185307179586;
    const PI: Self = 3.141592653589793;
    const HALFPI: Self = 1.5707963267948966;
    const QTRPI: Self = 0.7853981633974483;
}
impl Inverse for f32 {
    fn inv(self) -> Self { 1.0 / self }
}
impl Inverse for f64 {
    fn inv(self) -> Self { 1.0 / self }
}
impl PowersOfE for f32 {}
impl PowersOfE for f64 {}
impl Magnitude for f32 {}
impl Magnitude for f64 {}
impl Reals for f32 {}
impl Reals for f64 {}