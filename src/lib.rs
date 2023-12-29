pub mod cc;
pub mod rat;
pub mod alg;

use crate::cc::*;
use crate::rat::*;
use crate::alg::*;


#[cfg(test)]
mod test {
   use super::*;
   #[test]
   fn m() {
      let mut cum: Poly<f32> = Poly::new(vec![-2.0, 0.0, 1.0]);
      println!("{:?}", cum.newton(1.0, 0.001));
   }
}