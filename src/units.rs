pub mod _experimental;
pub mod compound;
pub mod concrete;
pub mod dim;
pub mod si;
pub mod symbols;
pub mod traits;
pub mod transform;

pub use compound::*;
pub use concrete::*;
pub use traits::{Unit, UnitCompound, UnitConcrete};


pub type Speed = utype!(Length / Time);
pub type Accel = utype!(Speed / Time);
pub type Momentum = utype!(Mass * Speed);

pub type Area = utype!(Length^2);
pub type Volume = utype!(Length^3);

pub type Pressure = utype!(Force / Area);
pub type Density = utype!(Mass / Volume);
pub type Torque = utype!(Length * Force);

pub type GravParam = utype!(Length^3 / Time^2);


#[test]
pub fn test_macros() {
    use symbols::common::*;

    let _accel_1: utype!(L/T/T) = unit!((m/s)/s);
    let _accel_2: Accel = unit!(m/s/s);
}
