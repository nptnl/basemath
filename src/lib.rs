pub mod rules;
pub mod cc;
pub mod alg;
pub mod rat;
pub mod prim;
pub mod lin;
#[allow(unused_imports)]
use crate::{rules::*, cc::*, alg::*, rat::*, prim::*, lin::*};

#[cfg(test)]
mod test {
   use super::*;
   #[test]
   fn mogus() {
      let third: Rat<i32> = Rat::new(2, 6);
      println!("{}", third.latex());
   }
}