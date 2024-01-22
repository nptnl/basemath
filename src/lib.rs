pub mod rules;
pub mod cc;
pub mod alg;
pub mod rat;
pub mod prim;

#[allow(unused_imports)]
use crate::rules::*;
#[allow(unused_imports)]
use crate::cc::*;
#[allow(unused_imports)]
use crate::alg::*;
#[allow(unused_imports)]
use crate::rat::*;
#[allow(unused_imports)]
use crate::prim::*;

#[cfg(test)]
mod test {
   use super::*;
   #[test]
   fn mogus() {
      let inp: c64 = -c64::PI * Comp::nre(0.66666667);
      println!("sin(π/3) = {}\ncos(π/3) = {}", inp.xsin(8), inp.xcos(8));
   }
}