//! Module for the [`Value`] marker trait.

use core::fmt::{Debug, Display};
use num_traits::{FromPrimitive, Num, NumCast};


dummy! {
    /// Marker trait for a type that can be used as the dimensionless component
    ///     of a [`Quantity`](crate::Quantity).
    ///
    /// Implemented automatically for any type that implements  all of the
    ///     following:
    /// - [`Clone`]
    /// - [`Debug`]
    /// - [`Display`]
    /// - [`FromPrimitive`]
    /// - [`Num`]
    /// - [`NumCast`]
    pub trait Value: Clone
        + Debug
        + Display
        + FromPrimitive
        + Num + NumCast
        + 'static
}


/// Convert an `f64` to any [`Value`] type.
///
/// TODO: This function is a stand-in to concentrate every conversion in one
///     place until the best way to do this is decided. It should be infallible
///     if at all possible, but
///         1. Will `from_f64` succeed for every `V`? *Probably* not.
///         2. Is there a better trait than [`FromPrimitive`] to use here?
pub(crate) fn _conv_f64<V: Value>(v: f64) -> V {
    V::from_f64(v).unwrap()
}


/// Convert an `i32` to any [`Value`] type.
///
/// TODO: See [`_conv_f64`].
pub(crate) fn _conv_i32<V: Value>(v: i32) -> V {
    V::from_i32(v).unwrap()
}
