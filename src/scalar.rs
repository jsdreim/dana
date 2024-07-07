use std::fmt::{Debug, Display};
use num_traits::{FromPrimitive, Num};


pub trait Scalar: FromPrimitive + Num + Clone + Debug + Display {}
impl<N: FromPrimitive + Num + Clone + Debug + Display> Scalar for N {}
