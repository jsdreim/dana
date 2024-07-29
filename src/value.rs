//! Module for the [`Value`] marker trait.

use std::fmt::{Debug, Display};
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
