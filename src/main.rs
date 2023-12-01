pub mod cc;
use crate::cc::*;

fn main() {
    let cum: g32 = Comp::new(3, -4) * Comp::new(3, 4);
    println!("{}", cum);
}