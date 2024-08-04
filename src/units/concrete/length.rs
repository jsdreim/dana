use crate::units::traits::{Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
//  NOTE: Keep these in ascending order so that derived `Ord` is correct.
pub enum Length {
    PicoMeter,
    NanoMeter,
    MicroMeter,
    MilliMeter,
    CentiMeter,
    Inch,

    Foot,
    Yard,
    Meter,
    KiloMeter,
    Mile,

    /// The distance travelled by light in one [second](super::Time::Second).
    LightSec,
    /// The average orbital radius of Earth.
    AstroUnit,
    /// The distance travelled by light in one year.
    LightYear,

    /// The distance to an object at 1 arcsecond of parallax.
    Parsec,
    KiloParsec,
    MegaParsec,
    GigaParsec,
}

impl Unit for Length {
    type Dim = crate::dimension::Length;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::PicoMeter => 1e-12,
            Self::NanoMeter => 1e-9,
            Self::MicroMeter => 1e-6,
            Self::MilliMeter => 1e-3,
            Self::CentiMeter => 1e-2,
            Self::Meter => 1e0,
            Self::KiloMeter => 1e+3,

            Self::Inch   => 0.025_4, // Defined as exactly 25.4mm.
            Self::Foot   => 0.304_8,
            Self::Yard   => 0.914_4,
            Self::Mile   => 1_609.344,

            Self::LightSec   => crate::constants::CONST_C.value,
            Self::AstroUnit  => 149_597_870_700.0,
            Self::LightYear  => 009_460_700_000_000_000.0,
            Self::Parsec     => 030_857_e+12,
            Self::KiloParsec => 030_857_e+15,
            Self::MegaParsec => 030_857_e+18,
            Self::GigaParsec => 030_857_e+21,
        }
    }
}

impl UnitConcrete for Length {
    const BASE: Self = Self::Meter;

    fn symbol(&self) -> &'static str {
        match self {
            Self::PicoMeter  => "pm",
            Self::NanoMeter  => "nm",
            Self::MicroMeter => "μm",
            Self::MilliMeter => "mm",
            Self::CentiMeter => "cm",
            Self::Meter      =>  "m",
            Self::KiloMeter  => "km",

            Self::Inch => "in",
            Self::Foot => "ft",
            Self::Yard => "yd",
            Self::Mile => "mi",

            Self::AstroUnit  => "AU",
            Self::LightSec   => "ls",
            Self::LightYear  => "ly",
            Self::Parsec     =>  "pc",
            Self::KiloParsec => "kpc",
            Self::MegaParsec => "Mpc",
            Self::GigaParsec => "Gpc",
        }
    }
}


impl UnitStep for Length {
    fn step_down(&self) -> Option<Self> {
        match self {
            //region Metric scale.
            Self::PicoMeter => None,
            Self::NanoMeter => Some(Self::PicoMeter),
            Self::MicroMeter => Some(Self::NanoMeter),
            Self::MilliMeter => Some(Self::MicroMeter),
            Self::CentiMeter => Some(Self::MilliMeter),
            Self::Meter => Some(Self::MilliMeter),
            Self::KiloMeter => Some(Self::Meter),
            //endregion

            //region Imperial scale.
            Self::Inch => Some(Self::MilliMeter),
            Self::Foot => Some(Self::Inch),
            Self::Yard => Some(Self::Foot),
            Self::Mile => Some(Self::Yard),
            //endregion

            //region Space scale.
            Self::LightSec => Some(Self::KiloMeter),
            Self::AstroUnit => Some(Self::LightSec),
            Self::LightYear => Some(Self::AstroUnit),
            Self::Parsec => Some(Self::LightYear),
            Self::KiloParsec => Some(Self::Parsec),
            Self::MegaParsec => Some(Self::KiloParsec),
            Self::GigaParsec => Some(Self::MegaParsec),
            //endregion
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            //region Metric scale.
            Self::PicoMeter => Some(Self::NanoMeter),
            Self::NanoMeter => Some(Self::MicroMeter),
            Self::MicroMeter => Some(Self::MilliMeter),
            Self::MilliMeter => Some(Self::Meter),
            Self::CentiMeter => Some(Self::Meter),
            Self::Meter => Some(Self::KiloMeter),
            Self::KiloMeter => Some(Self::LightSec),
            //endregion

            //region Imperial scale.
            Self::Inch => Some(Self::Foot),
            Self::Foot => Some(Self::Mile),
            Self::Yard => Some(Self::Mile),
            Self::Mile => Some(Self::LightSec),
            //endregion

            //region Space scale.
            Self::LightSec => Some(Self::AstroUnit),
            Self::AstroUnit => Some(Self::LightYear),
            Self::LightYear => Some(Self::Parsec),
            Self::Parsec => Some(Self::KiloParsec),
            Self::KiloParsec => Some(Self::MegaParsec),
            Self::MegaParsec => Some(Self::GigaParsec),
            Self::GigaParsec => None,
            //endregion
        }
    }
}
