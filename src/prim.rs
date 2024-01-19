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
fn _ln_raw<X: Reals>(inp: X, iterations: usize) -> X {
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
    if out > X::HALFPI { out = -out - X::PI; real_flip = true; }
    (out, real_flip)
}

pub trait Exponential: Reals {
    fn exp(self, iterations: usize) -> Self {
        let (fixed, extra, neg): (Self, isize, bool) = exp_real_fix(self);
        let out: Self = exp_raw(fixed, iterations) * Self::etothe(extra);
        if neg { out.inv() } else { out }
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
}