use crate::rules::*;

pub fn exp_raw<X: Reals>(inp: X, iterations: usize) -> X {
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
pub fn ln_raw<X: Reals>(inp: X, iterations: usize) -> X {
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

pub fn exp_real_fix<X: Reals>(real: X) -> (X, isize) {
    let mut neg: bool = false;
    let mut extra: isize = 1;
    let mut out: X = real;
    if out < X::ZERO { out = -out; neg = true; }
    while out > X::ONE { extra += 1; out -= X::ONE; }
    if neg { extra = -extra; }
    (out, extra)
}
pub fn exp_imag_fix<X: Reals>(imag: X) -> (X, bool) {
    let mut out: X = imag;
    let mut real_flip: bool = false;
    out %= X::TAU;
    if out > X::PI { out -= X::TAU; }
    else if out <= -X::PI { out += X::TAU; }
    if out > X::HALFPI { out = -out - X::PI; real_flip = true; }
    (out, real_flip)
}

pub fn exp<X: Reals>(inp: X, iterations: usize) -> X {
    unimplemented!()
}