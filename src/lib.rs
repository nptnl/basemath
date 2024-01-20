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
      let inp: c64 = Comp::nim(-1.0);
      println!("ln(-i) = {}", inp.lnn(9));
   }
}