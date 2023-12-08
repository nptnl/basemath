pub mod cc;
pub mod rat;
use crate::cc::*;
use crate::rat::*;

mod test {
   fn m() {
   let mut cum: c64 = Comp::new(2.0, 1.0);
   cum = raw_exp(cum, 10);
   println!("{}", cum);
   }
}