//! Trigonometric functions, along with hyperbolics and inverses.

use crate::alg::{exp, ixp, ln, sqrt};
use crate::ch::Comp;

static ONE: Comp = Comp { r: 1.0, i: 0.0 };
static I: Comp = Comp { r: 0.0, i: 1.0 };
static PI: f64 = 3.1415926535;

/// sine
pub fn sin(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(-0.5) * (series - series.inv())
}
/// cosine
pub fn cos(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    0.5 * (series + series.inv())
}
/// tangent
pub fn tan(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(-1.0) * (series - series.inv()) / (series + series.inv())
}
/// cotangent
pub fn cot(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    I * (series + series.inv()) / (series - series.inv())
}
/// secant
pub fn sec(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    2.0  / (series + series.inv())
}
/// cosecant
pub fn csc(x: Comp) -> Comp {
    let series: Comp = ixp(x);
    Comp::nim(2.0) / (series - series.inv())
}

/// inverse sine, arc-sine
pub fn asin(x: Comp) -> Comp {
    Comp::nre(0.5*PI) + Comp::nim(-1.0) * ln(x - sqrt(x*x - ONE))
}
/// inverse cosine, arc-cosine
pub fn acos(x: Comp) -> Comp {
    Comp::nim(-1.0) * ln(x + sqrt(x*x - ONE))
}
/// inverse tangent, arc-tangent
pub fn atan(x: Comp) -> Comp {
    0.5*PI + Comp::nim(-0.5) * ln((I*x + ONE) / (I*x - ONE))
}
/// inverse cotangent, arc-cotangent
pub fn acot(x: Comp) -> Comp {
    Comp::nim(-0.5) * ln((x + I) / (x - I))
}
/// inverse secant, arc-secant
pub fn asec(x: Comp) -> Comp { acos(x.inv()) }
/// inverse cosecant, arc-cosecant
pub fn acsc(x: Comp) -> Comp { asin(x.inv()) }

/// hyperbolic sine
pub fn sinh(x: Comp) -> Comp {
    let series: Comp = exp(x);
    0.5 * (series - series.inv())
}
/// hyperbolic cosine
pub fn cosh(x: Comp) -> Comp {
    let series: Comp = exp(x);
    0.5 * (series + series.inv())
}
/// hyperbolic tangent
pub fn tanh(x: Comp) -> Comp {
    let series: Comp = exp(x);
    (series - series.inv()) / (series + series.inv())
}
/// hyperbolic cotangent
pub fn coth(x: Comp) -> Comp {
    let series: Comp = exp(x);
    (series + series.inv()) / (series -  series.inv())
}
/// hyperbolic secant
pub fn sech(x: Comp) -> Comp {
    let series: Comp = exp(x);
    2.0 / (series + series.inv())
}
/// hyperbolic cosecant
pub fn csch(x: Comp) -> Comp {
    let series: Comp = exp(x);
    2.0 / (series - series.inv())
}

/// inverse hyperbolic sine
pub fn asinh(x: Comp) -> Comp {
    ln(x - sqrt(x*x + 1.0))
}
/// inverse hyperbolic cosine
pub fn acosh(x: Comp) -> Comp {
    ln(x - sqrt(x*x - 1.0))
}
/// inverse hyperbolic tangent
pub fn atanh(x: Comp) -> Comp {
    Comp::nim(0.5*PI) + 0.5 * ln((x + 1.0) / (x - 1.0))
}
/// inverse hyperbolic cotangent
pub fn acoth(x: Comp) -> Comp {
    0.5 * ln((x + 1.0) / (x - 1.0))
}
/// inverse hyperbolic secant
pub fn asech(x: Comp) -> Comp {
    acosh(x.inv())
}
/// inverse hyperbolic cosecant
pub fn acsch(x: Comp) -> Comp {
    asinh(x.inv())
}