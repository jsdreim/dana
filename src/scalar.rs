//! Module for the [`Scalar`] marker trait.

use std::fmt::{Debug, Display};
use num_traits::{FromPrimitive, Num, NumCast};


dummy! {
    /// Marker trait for a type that can be used as the scalar component of a
    ///     [`Quantity`](crate::Quantity).
    ///
    /// Implemented automatically for any type that implements  all of the
    ///     following:
    /// - [`Clone`]
    /// - [`Debug`]
    /// - [`Display`]
    /// - [`FromPrimitive`]
    /// - [`Num`]
    /// - [`NumCast`]
    pub trait Scalar: Clone
        + Debug
        + Display
        + FromPrimitive
        + Num + NumCast
        + 'static
}
