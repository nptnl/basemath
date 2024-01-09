pub mod cc;
pub mod rat;
pub mod alg;
pub mod rules;

use crate::cc::*;
use crate::alg::*;
use crate::rat::*;
use crate::rules::*;



#[cfg(test)]
mod test {
   use super::*;
   #[test]
   fn m() {
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
}