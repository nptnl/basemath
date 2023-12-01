pub mod cc;
use crate::cc::*;

pub mod rat;
use crate::rat::*;

fn main() {
    let cum: Rat<i32> = Rat::new(6, 8) + Rat::new(1, 12);
    println!("{}", cum);
}