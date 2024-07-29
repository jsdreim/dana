use crate::units::traits::{Unit, UnitConcrete, UnitStep};


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Force {
    MicroNewton,
    MilliNewton,
    Ounce,
    Newton,
    Pound,
    KiloNewton,
    MegaNewton,
    GigaNewton,
    TeraNewton,
}

impl Unit for Force {
    type Dim = crate::dimension::Force;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroNewton => 1e-6,
            Self::MilliNewton => 1e-3,
            Self::Newton => 1e0,
            Self::KiloNewton => 1e+3,
            Self::MegaNewton => 1e+6,
            Self::GigaNewton => 1e+9,
            Self::TeraNewton => 1e+12,

            Self::Ounce => 0.278_013_9,
            Self::Pound => 4.448_222,
        }
    }
}

impl UnitConcrete for Force {
    const BASE: Self = Self::Newton;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroNewton => "μN",
            Self::MilliNewton => "mN",
            Self::Newton => "N",
            Self::KiloNewton => "kN",
            Self::MegaNewton => "MN",
            Self::GigaNewton => "GN",
            Self::TeraNewton => "TN",

            Self::Ounce => "ozf",
            Self::Pound => "lbf",
        }
    }
}

impl UnitStep for Force {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroNewton => None,
            Self::MilliNewton => Some(Self::MicroNewton),
            Self::Newton      => Some(Self::MilliNewton),
            Self::KiloNewton  => Some(Self::Newton),
            Self::MegaNewton  => Some(Self::KiloNewton),
            Self::GigaNewton  => Some(Self::MegaNewton),
            Self::TeraNewton  => Some(Self::GigaNewton),

            Self::Ounce       => Some(Self::MilliNewton),
            Self::Pound       => Some(Self::Ounce),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroNewton => Some(Self::MilliNewton),
            Self::MilliNewton => Some(Self::Newton),
            Self::Newton      => Some(Self::KiloNewton),
            Self::KiloNewton  => Some(Self::MegaNewton),
            Self::MegaNewton  => Some(Self::GigaNewton),
            Self::GigaNewton  => Some(Self::TeraNewton),
            Self::TeraNewton  => None,

            Self::Ounce       => Some(Self::Pound),
            Self::Pound       => Some(Self::KiloNewton),
        }
    }
}
