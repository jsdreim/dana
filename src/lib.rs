//! Compile-time [dimensional analysis] via generic types.
//!
//! [dimensional analysis]: https://en.wikipedia.org/wiki/Dimensional_analysis
//!
//! The core of the crate is the [`Unit`] trait, which marks a struct or enum as
//!     being a representation of a dimensional unit. Units may have multiple
//!     variants, each with a different scaling factor.
//!
//! The most important exported type is [`Quantity`]. A `Quantity` is generic
//!     over `Unit` types and numeric [`Value`] types, and serves to pair a
//!     dimensionless value with a dimensional unit. The default scalar type is
//!     `f64`, but [`Value`] is automatically implemented for any type that
//!     implements the correct set of traits.
//!
//! # Examples
//!
//! The goal of dimensional analysis is to ensure correctness when doing
//!     calculations on physical quantities. Operations are only valid between
//!     quantities of compatible units, and units must be conserved in order to
//!     get correct results.
//!
//! The fundamental principle of this library is to represent quantities with
//!     incompatible units as different types, so that attempting to use them
//!     together results in a compiler error.
//!
//! The following function takes a distance an object has moved, as well as the
//!     amount of time it took to move that distance, and calculates the average
//!     speed of the object:
//! ```
//! use dana::{Quantity, units::{Length, Speed, Time}};
//!
//! fn speed(dist: Quantity<Length>, time: Quantity<Time>) -> Quantity<Speed> {
//!     dist / time
//! }
//! ```
//!
//! This calculation is correctly performed by dividing the distance moved by
//!     the time taken. Attempting to perform the wrong operation will produce
//!     the wrong type, resulting in a "mismatched types" error:
//! ```compile_fail
//! # use dana::{Quantity, units::{Length, Speed, Time}};
//! #
//! fn speed(dist: Quantity<Length>, time: Quantity<Time>) -> Quantity<Speed> {
//!     time / dist
//! }
//! ```
//!
//! ## Defining Quantities
//!
//  TODO: Focus less heavily on `qty!` here. Link to it and recommend it, but
//      then describe non-macro definition.
//! Using the full syntax is verbose to the point of near-unreadability:
//! ```
//! use dana::{Quantity, units::*};
//!
//! let grav: Quantity<UnitDiv<Length, UnitSquared<Time>>> = Quantity {
//!     unit: UnitDiv::new(Length::Meter, UnitSquared::new(Time::Second)),
//!     value: 9.81,
//! };
//! ```
//!
//! This can be reduced somewhat by using type inference, [`Quantity::new`] or
//!     [`Unit::quantity`], standard library math operators, and the methods of
//!     unit traits. The result is better, but still difficult to read for more
//!     complex expressions:
//! ```
//! use dana::{Quantity, units::{concrete::*, traits::CanSquare}};
//!
//! let grav = Quantity::new(
//!     Length::Meter / Time::Second.squared(),
//!     9.81,
//! );
//! ```
//!
//! To make large units more workable, the [`qty`] macro interprets combinations
//!     of units using a wider range of operators than the standard library
//!     traits provide:
//! ```
//! use dana::{qty, units::concrete::*};
//!
//! let grav = qty![9.81 Length::Meter / (Time::Second ^ 2)];
//! ```
//!
//! Finally, the [`symbols`] module provides standard SI unit symbols as
//!     constants and type aliases, bringing the syntax very close to a pure
//!     mathematical form:
//! ```
//! use dana::{qty, symbols::*};
//!
//! let grav = qty![9.81 m/s^2];
//! ```

#![no_std]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![warn(missing_docs)]

//  NOTE: Hack to allow proc macros to work both inside and outside the crate.
extern crate self as dana;

#[macro_use]
pub mod macros;
pub mod prelude;

pub mod constants;
#[allow(missing_docs)]
pub mod dimension;
pub mod equations;
pub mod quantity;
pub mod symbols;
pub mod units;
pub mod value;

pub mod error;

#[cfg(feature = "simd")]
#[allow(missing_docs)]
pub mod simd;

pub use quantity::Quantity;
pub use units::Unit;
pub use value::Value;
use value::{_conv_f64, _conv_i32};
