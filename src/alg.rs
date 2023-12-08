use crate::cc::Arithmetic;

pub fn raw_exp<X: Arithmetic>(x: X, terms: usize) -> X {
    let mut total: X = X::zero();
    let mut run_poly: X = X::one();
    let mut run_power: X = X::one();
    for _ in 1..terms {
        total += run_poly;
        run_poly &= x / run_power;
        run_power += X::one();
    }
    total
}