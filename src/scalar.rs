use std::fmt::{Debug, Display};
use num_traits::{FromPrimitive, Num};


/// Marker trait for a type that can be used as the scalar component of a
///     [`Quantity`](crate::Quantity). Implemented automatically for any type
///     that implements all of the following:
/// - [`FromPrimitive`]
/// - [`Num`]
/// - [`Clone`]
/// - [`Debug`]
/// - [`Display`]
pub trait Scalar: FromPrimitive + Num + Clone + Debug + Display + 'static {}
impl<N: FromPrimitive + Num + Clone + Debug + Display + 'static> Scalar for N {}
