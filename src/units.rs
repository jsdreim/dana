pub mod compound;
pub mod concrete;
pub mod traits;
pub mod transform;

pub use compound::*;
pub use concrete::*;
pub use traits::{Unit, UnitCompound, UnitConcrete};


pub type Speed = utype!(Distance / Time);
pub type Accel = utype!(Speed / Time);
pub type Momentum = utype!(Mass * Speed);

pub type Area = utype!(Distance^2);
pub type Volume = utype!(Distance^3);

pub type Pressure = utype!(Force / Area);
pub type Density = utype!(Mass / Volume);
pub type Torque = utype!(Distance * Force);

pub type GravParam = utype!(Distance^3 / Time^2);


//  TODO: These should probably not ALL have shortcuts. The possibility of units
//      (or aliases) being defined downstream must be considered.
#[macro_export]
macro_rules! utype_from_symbol {
    //  Concrete.
    (l) => { $crate::units::concrete::Length };
    (m) => { $crate::units::concrete::Mass };
    (t) => { $crate::units::concrete::Time };

    (F) => { $crate::units::concrete::Force };
    (E) => { $crate::units::concrete::Energy };
    (P) => { $crate::units::concrete::Power };
    (I) => { $crate::units::concrete::Current };
    (V) => { $crate::units::concrete::Voltage };
    (R) => { $crate::units::concrete::Resistance };

    //  Compound.
    (v) => { $crate::units::Speed };
    (a) => { $crate::units::Accel };
    (p) => { $crate::units::Momentum };

    // (A) => { $crate::units::Area };
    // (V) => { $crate::units::Volume }; // This or Voltage?
    // (P) => { $crate::units::Pressure }; // This or Power?
    // (D) => { $crate::units::Density };

    ($t:tt) => { $t };
}


#[test]
pub fn test_macros() {
    let _accel_1: utype!(l/t/t) = unit!((m/s)/s);
    let _accel_2: Accel = unit!(m/s/s);
}
