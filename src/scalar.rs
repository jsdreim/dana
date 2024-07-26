use std::fmt::{Debug, Display};
use num_traits::{FromPrimitive, Num, NumCast};


dummy! {
/// Marker trait for a type that can be used as the scalar component of a
///     [`Quantity`](crate::Quantity). Implemented automatically for any type
///     that implements all of the following:
/// - [`FromPrimitive`]
/// - [`Num`]
/// - [`NumCast`]
/// - [`Clone`]
/// - [`Debug`]
/// - [`Display`]
pub trait Scalar:
    FromPrimitive
    + Num + NumCast
    + Clone
    + Debug
    + Display
    + 'static
}
