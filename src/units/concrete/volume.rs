use crate::units::{Length, traits::{CanRoot, Unit, UnitConcrete, UnitStep}, UnitRescale};


#[allow(dead_code)]
const GAL_IMP: f64 = 4.546_090_e-3;
#[allow(dead_code)]
const GAL_USA: f64 = 3.785_411_784_e-3;

const GAL: f64 = GAL_USA;


#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Volume {
    MicroLiter,
    MilliLiter,

    Dram,
    FlOunce,
    Cup,
    Pint,
    Quart,
    Liter,
    Gallon,

    KiloLiter,
    MegaLiter,
    GigaLiter,
    TeraLiter,
}

impl Unit for Volume {
    type Dim = crate::dimension::Volume;
    // type ScaleType = f64;

    fn scale(&self) -> f64 {
        match self {
            Self::MicroLiter => 1e-9,
            Self::MilliLiter => 1e-6,
            Self::Liter      => 1e-3,
            Self::KiloLiter  => 1e00,
            Self::MegaLiter  => 1e+3,
            Self::GigaLiter  => 1e+6,
            Self::TeraLiter  => 1e+9,

            Self::Dram       => GAL / 1_280.0,
            Self::FlOunce    => GAL / 160.0,
            Self::Cup        => GAL / 16.0,
            Self::Pint       => GAL / 8.0,
            Self::Quart      => GAL / 4.0,
            Self::Gallon     => GAL,
        }
    }
}

impl CanRoot<3> for Volume {
    type Output = UnitRescale<Length>;

    fn root(self) -> Self::Output {
        match self {
            Self::MicroLiter => Length::MilliMeter.rescale(1.0),
            Self::MilliLiter => Length::MilliMeter.rescale(10.0),
            Self::Liter      => Length::MilliMeter.rescale(100.0),
            Self::KiloLiter  => Length::     Meter.rescale(1.0),
            Self::MegaLiter  => Length::     Meter.rescale(10.0),
            Self::GigaLiter  => Length::     Meter.rescale(100.0),
            Self::TeraLiter  => Length:: KiloMeter.rescale(1.0),

            _ => Length::Meter.rescale((self.scale() * 1e-3).cbrt() * 10.0),
        }
    }
}

impl UnitConcrete for Volume {
    const BASE: Self = Self::Liter;

    fn symbol(&self) -> &'static str {
        match self {
            Self::MicroLiter => "μL",
            Self::MilliLiter => "mL",
            Self::Liter      => "L",
            Self::KiloLiter  => "kL",
            Self::MegaLiter  => "ML",
            Self::GigaLiter  => "GL",
            Self::TeraLiter  => "TL",

            Self::Dram       => "fl dr",
            Self::FlOunce    => "fl oz",
            Self::Cup        => "c",
            Self::Pint       => "pt",
            Self::Quart      => "qt",
            Self::Gallon     => "gal",
        }
    }
}

impl UnitStep for Volume {
    fn step_down(&self) -> Option<Self> {
        match self {
            Self::MicroLiter => None,
            Self::MilliLiter => Some(Self::MicroLiter),
            Self::Liter      => Some(Self::MilliLiter),
            Self::KiloLiter  => Some(Self::Liter),
            Self::MegaLiter  => Some(Self::KiloLiter),
            Self::GigaLiter  => Some(Self::MegaLiter),
            Self::TeraLiter  => Some(Self::GigaLiter),

            Self::Dram       => Some(Self::MilliLiter),
            Self::FlOunce    => Some(Self::Dram),
            Self::Cup        => Some(Self::FlOunce),
            Self::Pint       => Some(Self::Cup),
            Self::Quart      => Some(Self::Pint),
            Self::Gallon     => Some(Self::Quart),
        }
    }

    fn step_up(&self) -> Option<Self> {
        match self {
            Self::MicroLiter => Some(Self::MilliLiter),
            Self::MilliLiter => Some(Self::Liter),
            Self::Liter      => Some(Self::KiloLiter),
            Self::KiloLiter  => Some(Self::MegaLiter),
            Self::MegaLiter  => Some(Self::GigaLiter),
            Self::GigaLiter  => Some(Self::TeraLiter),
            Self::TeraLiter  => None,

            Self::Dram       => Some(Self::FlOunce),
            Self::FlOunce    => Some(Self::Cup),
            Self::Cup        => Some(Self::Pint),
            Self::Pint       => Some(Self::Quart),
            Self::Quart      => Some(Self::Gallon),
            Self::Gallon     => Some(Self::KiloLiter),
        }
    }
}
