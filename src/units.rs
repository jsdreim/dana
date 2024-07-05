pub mod compound;
pub mod concrete;
pub mod conversions;
pub mod traits;

pub use compound::*;
pub use concrete::*;
pub use traits::{Unit, UnitCompound, UnitConcrete};


pub type Speed = utype!(Distance / Time);
pub type Acceleration = utype!(Speed / Time);
pub type Momentum = utype!(Mass * Speed);

pub type Area = utype!(Distance^2);
pub type Volume = utype!(Distance^3);

pub type Pressure = utype!(Force / Area);
pub type Density = utype!(Mass / Volume);
pub type Torque = utype!(Distance * Force);

pub type GravParam = utype!(Distance^3 / Time^2);
