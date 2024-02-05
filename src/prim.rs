use crate::rules::*;
use crate::cc::Comp;

fn exp_raw<X: Reals>(inp: X, iterations: usize) -> X {
    let mut total: X = X::ZERO;
    let mut running: X = X::ONE;
    let mut indx: X = X::ONE;
    for _ in 0..iterations {
        total += running;
        running *= inp / indx;
        indx += X::ONE;
    }
    total
}
fn ln_raw<X: Reals>(inp: X, iterations: usize) -> X {
    let centered: X = inp - X::ONE;
    let mut total: X = X::ZERO;
    let mut running: X = centered;
    let mut indx: X = X::ONE;
    for _ in 0..iterations {
        total += running / indx;
        running *= -centered;
        indx += X::ONE;
    }
    total
}

fn exp_real_fix<X: Reals>(real: X) -> (X, isize, bool) {
    let mut neg: bool = false;
    let mut extra: isize = 0;
    let mut out: X = real;
    if out < X::ZERO { out = -out; neg = true; }
    while out > X::ONE { extra += 1; out -= X::ONE; }
    (out, extra, neg)
}
fn exp_imag_fix<X: Reals>(imag: X) -> (X, bool) {
    let mut out: X = imag;
    let mut real_flip: bool = false;
    out %= X::TAU;
    if out > X::PI { out -= X::TAU; }
    else if out <= -X::PI { out += X::TAU; }
    if out > X::HALFPI { out = X::PI - out; real_flip = true; }
    else if out < -X::HALFPI { out = -X::PI - out; real_flip = true; }
    (out, real_flip)
}
fn ln_mag_fix<X: Reals>(mag: X) -> (X, X, bool) {
    let mut out: X = mag;
    let mut extra: X = X::ZERO;
    let mut neg: bool = false;
    if out.mag2() > X::ONE { out = out.inv(); neg = true; }
    while out < X::ONE - X::E.inv() { out *= X::E; extra += X::ONE; }
    (out, extra, neg)
}
fn ln_angle_fix<X: Reals>(unit: Comp<X>) -> (Comp<X>, X) {
    let (new_real, new_imag, extra): (X, X, X) =
    if unit.r.mag2() > unit.i.mag2() {
        if unit.r < X::ZERO { (-unit.r, -unit.i, X::PI) }
        else { (unit.r, unit.i, X::ZERO) }
    } else {
        if unit.i < X::ZERO { (-unit.i, unit.r, -X::HALFPI) }
        else { (unit.i, -unit.r, X::HALFPI) }
    };
    (Comp { r: new_real, i: new_imag }, extra)
}

pub trait Exponential: Reals {
    fn exp(self, iterations: usize) -> Self {
        let (fixed, extra, neg): (Self, isize, bool) = exp_real_fix(self);
        let out: Self = exp_raw(fixed, iterations) * Self::etothe(extra);
        if neg { out.inv() } else { out }
    }
    fn lnn(self, iterations: usize) -> Self {
        if self < Self::ZERO { return Self::UNDEF; }
        let (mag_fix, extra_real, invert): (Self, Self, bool) = ln_mag_fix(self);
        if invert { ln_raw(mag_fix.inv(), iterations) + extra_real }
        else { ln_raw(mag_fix, iterations) - extra_real }
    }
    fn power(&self, rhs: Self, iterations: usize) -> Self {
        (self.lnn(iterations) * rhs).exp(iterations)
    }
}
impl Exponential for f32 {}
impl Exponential for f64 {}
impl<R: Reals> Exponential for Comp<R> {
    fn exp(self, iterations: usize) -> Self {
        let (r_fixed, extra, neg): (R, isize, bool) = exp_real_fix(self.r);
        let (i_fixed, real_flip): (R, bool) = exp_imag_fix(self.i);
        let mut out: Comp<R> = exp_raw(Comp { r: r_fixed, i: i_fixed }, iterations) * Comp::etothe(extra);
        if neg { out = out.inv(); out.i = -out.i; }
        if real_flip { out.r = -out.r; }
        out
    }
    fn lnn(self, iterations: usize) -> Self {
        let mag: Self = self.mag1(Self::order_of(-6) * self.mag2());
        let unit: Self = self / mag;
        let mag: R = mag.r;
        let (mag_fix, extra_real, invert): (R, R, bool) = ln_mag_fix(mag);
        let mag_fix = Self::nre(mag_fix);
        let (ang_fix, extra_imag): (Self, R) = ln_angle_fix(unit);
        if invert { ln_raw(ang_fix / mag_fix, iterations) + Self::new(extra_real, extra_imag) }
        else { ln_raw(ang_fix * mag_fix, iterations) + Self::new(-extra_real, extra_imag) }
    }
}

fn sin_raw<X: Reals>(inp: X, iterations: usize) -> X {
    let mut total: X = X::ZERO;
    let mut running: X = inp;
    let mut indx: X = X::TWO;
    for _ in 0..iterations {
        total += running;
        running *= -inp * inp / indx / (indx + X::ONE);
        indx += X::TWO;
    }
    total
}
fn cos_raw<X: Reals>(inp: X, iterations: usize) -> X {
    let mut total: X = X::ZERO;
    let mut running: X = X::ONE;
    let mut indx: X = X::ONE;
    for _ in 0..iterations {
        total += running;
        running *= -inp * inp / indx / (indx + X::ONE);
        indx += X::TWO;
    }
    total
}
pub trait CircularTrig: Reals + Exponential {
    fn xsin(self, iterations: usize) -> Self {
        let fixed: Self = exp_imag_fix(self).0;
        sin_raw(fixed, iterations)
    }
    fn xcos(self, iterations: usize) -> Self {
        let (fixed, neg): (Self, bool) = exp_imag_fix(self);
        if neg { -cos_raw(fixed, iterations) } else { cos_raw(fixed, iterations) }
    }
    fn xcsc(self, iterations: usize) -> Self { self.xsin(iterations).inv() }
    fn xsec(self, iterations: usize) -> Self { self.xcos(iterations).inv() }
    fn xtan(self, iterations: usize) -> Self {
        let (fixed, neg): (Self, bool) = exp_imag_fix(self);
        if neg { -sin_raw(fixed, iterations) / cos_raw(fixed, iterations) }
        else { sin_raw(fixed, iterations) / cos_raw(fixed, iterations) }
    }
    fn xcot(self, iterations: usize) -> Self {
        let (fixed, neg): (Self, bool) = exp_imag_fix(self);
        if neg { -cos_raw(fixed, iterations) / sin_raw(fixed, iterations) }
        else { cos_raw(fixed, iterations) / sin_raw(fixed, iterations) }
    }
}
pub trait HyperbolicTrig: Reals + Exponential {
    fn xsinh(self, iterations: usize) -> Self {
        let series: Self = self.exp(iterations);
        (series - series.inv()) / Self::TWO
    }
    fn xcosh(self, iterations: usize) -> Self {
        let series: Self = self.exp(iterations);
        (series + series.inv()) / Self::TWO
    }
    fn xcsch(self, iterations: usize) -> Self { self.xsinh(iterations).inv() }
    fn xsech(self, iterations: usize) -> Self { self.xcosh(iterations).inv() }
    fn xtanh(self, iterations: usize) -> Self {
        let series: Self = self.exp(iterations);
        (series - series.inv()) / (series + series.inv())
    }
    fn xcoth(self, iterations: usize) -> Self {
        let series: Self = self.exp(iterations);
        (series + series.inv()) / (series - series.inv())
    }
}

static INV_TRIG_ERR_PWR: isize = -4;
#[inline(always)]
fn root_plus<X: Reals>(inp: X) -> X {
    (X::ONE + inp * inp).rrt(X::order_of(INV_TRIG_ERR_PWR))
}
#[inline(always)]
fn root_minus<X: Reals>(inp: X) -> X {
    (X::ONE - inp * inp).rrt(X::order_of(INV_TRIG_ERR_PWR))
}

pub trait CircularTrigInv: Reals + Exponential {
    fn xacos(self, iterations: usize) -> Self {
        if self > Self::ONE || self < -Self::ONE { return Self::UNDEF; }
        let unit_circle: Comp<Self> = Comp { r: self, i: root_minus(self) };
        unit_circle.lnn(iterations).i
    }
    fn xasin(self, iterations: usize) -> Self { Self::HALFPI + self.xacos(iterations) }
    fn xacsc(self, iterations: usize) -> Self { Self::HALFPI + self.inv().xacos(iterations) }
    fn xasec(self, iterations: usize) -> Self { self.inv().xacos(iterations) }
    fn xatan(self, iterations: usize) -> Self { root_plus(self).inv().xacos(iterations) }
    fn xacot(self, iterations: usize) -> Self { self.inv().xatan(iterations) }
}
pub trait HyperbolicTrigInv: Reals + Exponential {
    fn xacosh(self, iterations: usize) -> Self {
        if self < Self::ONE { return Self::UNDEF; }
        (self + root_plus(self)).lnn(iterations)
    }
    fn xasinh(self, iterations: usize) -> Self { (self + root_plus(self)).lnn(iterations) }
    fn xacsch(self, iterations: usize) -> Self { self.inv().xasinh(iterations) }
    fn xasech(self, iterations: usize) -> Self { self.inv().xasinh(iterations) }
    fn xatanh(self, iterations: usize) -> Self { root_minus(self).inv().xacosh(iterations) }
    fn xacoth(self, iterations: usize) -> Self { self.inv().xatanh(iterations) }
}

impl CircularTrig for f32 {}
impl CircularTrig for f64 {}
impl HyperbolicTrig for f32 {}
impl HyperbolicTrig for f64 {}
impl CircularTrigInv for f32 {}
impl CircularTrigInv for f64 {}
impl HyperbolicTrigInv for f32 {}
impl HyperbolicTrigInv for f64 {}

impl<R: RealArithmetic> Comp<R> {
    pub fn ccw(self) -> Self {
        Self { r: -self.i, i: self.r }
    }
    pub fn cw(self) -> Self {
        Self { r: self.i, i: -self.r }
    }
}
impl<R: Reals> Comp<R> {
    pub fn ixp(self, iterations: usize) -> Self {
        self.ccw().exp(iterations)
    }
}
impl<R: Reals> CircularTrig for Comp<R> {
    fn xsin(self, iterations: usize) -> Self {
        let series: Self = self.ixp(iterations);
        ((series - series.inv()) / Self::TWO).cw()
    }
    fn xcos(self, iterations: usize) -> Self {
        let series: Self = self.ixp(iterations);
        (series + series.inv()) / Self::TWO
    }
    fn xtan(self, iterations: usize) -> Self {
        let series: Self = self.ixp(iterations);
        (series - series.inv()) / (series + series.inv()).cw()
    }
    fn xcot(self, iterations: usize) -> Self {
        let series: Self = self.ixp(iterations);
        ((series + series.inv()) / (series - series.inv())).ccw()
    }
}
impl<R: Reals> HyperbolicTrig for Comp<R> {}
impl<R: Reals> CircularTrigInv for Comp<R> {
    fn xacos(self, iterations: usize) -> Self {
        let unit_circle: Self = self + (self * self - Self::ONE).rrt(Self::order_of(-4));
        unit_circle.lnn(iterations).cw()
    }
}
impl<R: Reals> HyperbolicTrigInv for Comp<R> {
    fn xacosh(self, iterations: usize) -> Self {
        (self + root_plus(self)).lnn(iterations)
    }
}

pub trait Trigonometry:
  Reals
  + Exponential
  + CircularTrig
  + CircularTrigInv
  + HyperbolicTrig
  + HyperbolicTrigInv
{}

impl Trigonometry for f32 {}
impl Trigonometry for f64 {}
impl<R: Reals> Trigonometry for Comp<R> {}