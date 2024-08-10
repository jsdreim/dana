//! Core module of the units system.

// pub mod _experimental;
pub mod compound;
pub mod concrete;
// pub mod si;
pub mod traits;
pub mod unit_anon;
pub mod unit_rescale;

pub use compound::*;
pub use concrete::*;
pub use derived::*;
pub use traits::{Unit, UnitCompound, UnitConcrete};
pub use unit_anon::UnitAnon;
pub use unit_rescale::UnitRescale;


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

    /// [`Length`] cubed over [`Time`] squared.
    ///
    /// The Gravitational Parameter, the product of mass and the [gravitational
    ///     constant](crate::constants::CONST_G), is used to calculate the
    ///     gravitational force between celestial bodies.
    ///
    /// Often represented as *μ*.
    pub type GravParam = utype!(Length^3 / Time^2);

    /// [`Energy`] per unit [`Temp`].
    ///
    /// Represents the amount of heat energy that must be supplied to an object
    ///     in order to increase its temperature by a certain amount.
    ///
    /// Often represented as *C*.
    pub type HeatCapacity = utype!(Energy / Temp);

    /// [Heat capacity](HeatCapacity) per unit [`Mass`].
    ///
    /// The intensive form of heat capacity.
    ///
    /// Often represented as *c*.
    pub type HeatSpecific = utype!(HeatCapacity / Mass);
}


/// Module to re-export all unit types.
pub mod types {
    pub use super::{
        compound::*,
        concrete::*,
        derived::*,
        unit_anon::UnitAnon,
        unit_rescale::UnitRescale,
    };
}
