use crate::units::{Unit, UnitConcrete};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Time {
    MilliSecond,
    Second,
    Minute,
    Hour,
}

impl Unit for Time {
    type Dim = crate::dimension::Time;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MilliSecond => 1e-3,
            Self::Second => 1e0,
            Self::Minute => 6e+1,
            Self::Hour => 36e+2,
        }
    }
}

impl UnitConcrete for Time {
    const BASE: Self = Self::Second;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MilliSecond => "ms",
            Self::Second => "s",
            Self::Minute => "min",
            Self::Hour => "h",
        }
    }
}
