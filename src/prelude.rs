//! Prelude module re-exporting [`Quantity`], [`Scalar`], convenience macros,
//!     and [`Unit`] traits and types.

pub use crate::{
    {dim, qty, qtype, unit, utype},
    Quantity,
    Scalar,
    units::{traits::*, types::*},
};
