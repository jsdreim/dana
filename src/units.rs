//! Core module of the units system.

pub mod _experimental;
pub mod compound;
pub mod concrete;
pub mod si;
pub mod traits;
pub mod unit_anon;

pub use compound::types::*;
pub use concrete::types::*;
pub use derived::*;
pub use traits::{Unit, UnitCompound, UnitConcrete};


/// Module for named compound unit types, defined as relationships between
///     concrete units.
pub mod derived {
    use super::*;

    /// [`Distance`](Length) travelled per unit [`Time`].
    ///
    /// Often represented as *v*.
    pub type Speed = utype!(Length / Time);

    /// Change in [`Speed`] per unit [`Time`].
    ///
    /// Often represented as *a*.
    pub type Accel = utype!(Speed / Time);

    /// [`Mass`] by unit [`Speed`].
    ///
    /// Often represented as *p*.
    pub type Momentum = utype!(Mass * Speed);

    /// [`Length`] squared, representing a region of two-dimensional space.
    ///
    /// Often represented as *A* or *S*.
    pub type Area = utype!(Length^2);

    /// [`Mass`] per unit [`Volume`]. More precisely, volumetric mass density.
    ///
    /// Often represented as *D* or *ρ*.
    pub type Density = utype!(Mass / Volume);

    /// [`Length`] by unit [`Force`].
    ///
    /// Often represented as *τ*.
    ///
    /// NOTE: Torque is *dimensionally* equivalent to [`Energy`] (compare the
    ///     definitions of the dimensions for [torque](crate::dimension::Torque)
    ///     and [energy](crate::dimension::Energy)), but in practice they are
    ///     **not** interchangeable, because torque is a [pseudovector].
    ///
    /// [pseudovector]: https://en.wikipedia.org/wiki/Pseudovector
    //  TODO: Should torque even be included? Is this out of scope?
    pub type Torque = utype!(Length * Force);

    pub type GravParam = utype!(Length^3 / Time^2);

    pub type HeatCapacity = utype!(Energy / Temp);
    pub type HeatSpecific = utype!(HeatCapacity / Mass);
}


/// Module to re-export all unit types.
pub mod types {
    pub use super::{
        compound::types::*,
        concrete::types::*,
        derived::*,
        unit_anon::UnitAnon,
    };
}
