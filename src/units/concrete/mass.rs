use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
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

impl UnitConcrete for Mass {
    const BASE: Self = Self::Kilogram;

    fn symbol(&self) -> &'static str {
        match self {
            Self::Gram => "g",
            Self::Kilogram => "kg",
            Self::MetricTon => "T",
        }
    }
}
