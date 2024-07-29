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

    pub type Speed = utype!(Length / Time);
    pub type Accel = utype!(Speed / Time);
    pub type Momentum = utype!(Mass * Speed);

    pub type Area = utype!(Length^2);

    pub type Density = utype!(Mass / Volume);
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
