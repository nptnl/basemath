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
      let inp: f64 = f64::TWO;
      println!("x = {}", inp);
      let out: f64 = inp.xsin(9);
      println!("sin(x) = {}", out);
      let inp: f64 = out.xasin(9);
      println!("asin( sin(x) ) = {}", inp);
   }
}