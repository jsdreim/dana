pub mod compound;
pub mod concrete;
pub mod traits;

pub use compound::*;
pub use concrete::*;
pub use traits::{Unit, UnitCompound, UnitConcrete};


pub type Speed = UnitDiv<Distance, Time>;
pub type Acceleration = UnitDiv<Speed, Time>;
pub type Momentum = UnitMul<Mass, Speed>;

// pub type Area = UnitSquared<Distance>;
// pub type Volume = UnitCubed<Distance>;

// pub type Pressure = UnitDiv<Force, Area>;
// pub type Density = UnitDiv<Mass, Volume>;
pub type Torque = UnitMul<Distance, Force>;

// pub type GravParam = UnitDiv<Volume, UnitSquared<Time>>;
