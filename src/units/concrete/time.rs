use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, /*Eq, Ord*/)]
pub enum Time {
    Millisecond,
    Second,
    Minute,
    Hour,
}

impl Unit for Time {
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::Millisecond => 1e-3,
            Self::Second => 1e0,
            Self::Minute => 6e+1,
            Self::Hour => 36e+2,
        }
    }
}

impl UnitConcrete for Time {}

impl Default for Time {
    fn default() -> Self { Self::Second }
}
