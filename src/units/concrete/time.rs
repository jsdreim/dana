use crate::units::traits::{Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Time {
    PicoSecond,
    NanoSecond,
    MicroSecond,
    MilliSecond,
    Second,
    Minute,
    Hour,
    Day,
}

impl Unit for Time {
    type Dim = crate::dimension::Time;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::PicoSecond  => 1e-12,
            Self::NanoSecond  => 1e-9,
            Self::MicroSecond => 1e-6,
            Self::MilliSecond => 1e-3,
            Self::Second => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3_600.0,
            Self::Day => 86_400.0,
        }
    }
}

impl UnitConcrete for Time {
    const BASE: Self = Self::Second;

    fn symbol(&self) -> &'static str {
        match self {
            Self::PicoSecond  => "ps",
            Self::NanoSecond  => "ns",
            Self::MicroSecond => "μs",
            Self::MilliSecond => "ms",
            Self::Second => "s",
            Self::Minute => "min",
            Self::Hour => "h",
            Self::Day => "d",
        }
    }
}


impl UnitStep for Time {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::PicoSecond => None,
            Self::NanoSecond => Some(Self::PicoSecond),
            Self::MicroSecond => Some(Self::NanoSecond),
            Self::MilliSecond => Some(Self::MicroSecond),
            Self::Second => Some(Self::MilliSecond),
            Self::Minute => Some(Self::Second),
            Self::Hour => Some(Self::Minute),
            Self::Day => Some(Self::Hour),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::PicoSecond => Some(Self::NanoSecond),
            Self::NanoSecond => Some(Self::MicroSecond),
            Self::MicroSecond => Some(Self::MilliSecond),
            Self::MilliSecond => Some(Self::Second),
            Self::Second => Some(Self::Minute),
            Self::Minute => Some(Self::Hour),
            Self::Hour => Some(Self::Day),
            Self::Day => None,
        }
    }
}
