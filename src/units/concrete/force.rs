use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Force {
    Newton,
    Kilonewton,
}

impl Unit for Force {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::Newton => 1e0,
            Self::Kilonewton => 1e+3,
        }
    }
}

impl UnitConcrete for Force {
    fn symbol(&self) -> &'static str {
        match self {
            Self::Newton => "N",
            Self::Kilonewton => "kN",
        }
    }
}

impl Default for Force {
    fn default() -> Self { Self::Newton }
}
