pub mod rules;
pub mod cc;
pub mod rat;
pub mod alg;
pub mod prim;


use crate::rules::*;
use crate::cc::*;
use crate::rat::*;
use crate::alg::*;
use crate::prim::*;




#[cfg(test)]
mod test {
   use super::*;
   #[test]
   fn polysolve() {
      let aarush: Poly<c32> = Poly::new(vec![
         Comp::nre(-2.0),
         Comp::ZERO,
         Comp::nre(1.0),
      ]);
      let sols = aarush.solve(Comp::order_of(-4));
      for each in sols {
         println!("{}", each);
      }
   }
   #[test]
   fn genexp() {
      let mut result: c64 = exp_raw(Comp::nre(3.0f64), 16);
      println!("{}", result);
      result = ln_raw(Comp::nre(1.01f64), 16);
      println!("{}", result);
   }
}