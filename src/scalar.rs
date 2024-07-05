use std::fmt::Display;
use num_traits::{FromPrimitive, Num};


pub trait Scalar: FromPrimitive + Num + Clone + Display {}
impl<N: FromPrimitive + Num + Clone + Display> Scalar for N {}
