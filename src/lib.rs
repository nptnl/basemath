pub mod rules;
pub mod cc;
pub mod alg;
pub mod prim;


use crate::rules::*;
use crate::cc::*;
use crate::alg::*;
use crate::prim::*;




#[cfg(test)]
mod test {
   use super::*;
   #[test]
   fn mogus() {
      let result: f64 = f64::etothe(3);
      println!("{}", result);
   }
}