use std::ops::{
    Neg, Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::cmp::{PartialEq, PartialOrd};

pub trait Identity: Copy {
    const ZERO: Self;
    const ONE: Self;
    const SEED: Self = Self::ONE;
}
pub trait UsefulReals: Arithmetic {
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
+ MulAssign + Mul<Output = Self> {
    fn etothe(power: isize) -> Self {
        let mut running: Self = Self::ONE;
        for _ in 0..power { running *= Self::E; }
        for _ in power..0 { running *= Self::E; }
        if power < 0 { running.inv() } else { running }
    }
}
pub trait Magnitude: Identity + Mul<Output = Self> {
    fn mag2(self) -> Self { self * self }
}


pub trait Arithmetic:
  Identity 
+ PowersOfTen
+ Magnitude
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

impl Magnitude for u8 {}
impl Magnitude for u16 {}
impl Magnitude for u32 {}
impl Magnitude for u64 {}
impl Magnitude for usize {}
impl Magnitude for i8 {}
impl Magnitude for i16 {}
impl Magnitude for i32 {}
impl Magnitude for i64 {}
impl Magnitude for isize {}
impl Magnitude for f32 {}
impl Magnitude for f64 {}

impl Arithmetic for i8 {}
impl Arithmetic for i16 {}
impl Arithmetic for i32 {}
impl Arithmetic for i64 {}
impl Arithmetic for isize {}
impl Arithmetic for f32 {}
impl Arithmetic for f64 {}

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