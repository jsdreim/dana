use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Mass {
    Gram,
    Kilogram,
    MetricTon,
}

impl Unit for Mass {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::Gram => 1e-3,
            Self::Kilogram => 1e0,
            Self::MetricTon => 1e+3,
        }
    }
}

impl UnitConcrete for Mass {}

impl Default for Mass {
    fn default() -> Self { Self::Kilogram }
}


impl_mul_unit_scalar!(Mass);
