use crate::rules::*;

pub fn exp_raw<X: Arithmetic>(inp: X, iterations: usize) -> X {
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
pub fn ln_raw<X: Arithmetic>(inp: X, iterations: usize) -> X {
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