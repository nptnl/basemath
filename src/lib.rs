//! Math functions, except more based because I cooked them up from scratch.
//! 
//! The goal here is to build up my own library of common math functions (and uncommon but cool ones) without any dependencies.
//! 
//! Everything is subject to change, as I will continue to find more math to implement, and more ways to optimize existing functions.
//!


pub mod comp;
pub mod alg;
pub mod trig;

pub use comp::*;
pub use alg::*;
pub use trig::*;