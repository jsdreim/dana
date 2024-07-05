use num_traits::{FromPrimitive, Num};


pub trait Scalar: FromPrimitive + Num + Clone {}
impl<N: FromPrimitive + Num + Clone> Scalar for N {}
