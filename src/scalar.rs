use num_traits::{FromPrimitive, Num};


pub trait Scalar: FromPrimitive + Num {}
impl<N: FromPrimitive + Num> Scalar for N {}
