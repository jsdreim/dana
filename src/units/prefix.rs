//! Module for experimental SI prefix enum.

use crate::units::{traits::*, unit_prefixed::UnitPrefixed};


/// A [metric prefix](https://en.wikipedia.org/wiki/Metric_prefix), used to
///     scale a unit.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum Prefix {
    /// Prefix with a scale of `10^-30`.
    Quecto = -30,
    /// Prefix with a scale of `10^-27`.
    Ronto = -27,
    /// Prefix with a scale of `10^-24`.
    Yocto = -24,
    /// Prefix with a scale of `10^-21`.
    Zepto = -21,
    /// Prefix with a scale of `10^-18`.
    Atto = -18,
    /// Prefix with a scale of `10^-15`.
    Femto = -15,
    /// Prefix with a scale of `10^-12`.
    Pico = -12,
    /// Prefix with a scale of `10^-9`.
    Nano = -9,
    /// Prefix with a scale of `10^-6`.
    Micro = -6,
    /// Prefix with a scale of `10^-3`.
    Milli = -3,
    /// Prefix with a scale of `10^-2`.
    Centi = -2,
    /// Prefix with a scale of `10^-1`.
    Deci = -1,

    // /// No prefix.
    // Empty = 0, // TODO?

    /// Prefix with a scale of `10^1`.
    Deca = 1,
    /// Prefix with a scale of `10^2`.
    Hecto = 2,
    /// Prefix with a scale of `10^3`.
    Kilo = 3,
    /// Prefix with a scale of `10^6`.
    Mega = 6,
    /// Prefix with a scale of `10^9`.
    Giga = 9,
    /// Prefix with a scale of `10^12`.
    Tera = 12,
    /// Prefix with a scale of `10^15`.
    Peta = 15,
    /// Prefix with a scale of `10^18`.
    Exa = 18,
    /// Prefix with a scale of `10^21`.
    Zetta = 21,
    /// Prefix with a scale of `10^24`.
    Yotta = 24,
    /// Prefix with a scale of `10^27`.
    Ronna = 27,
    /// Prefix with a scale of `10^30`.
    Quetta = 30,
}

impl Prefix {
    /// Return the scaling factor of this prefix, to be multiplied by the scale
    ///     of the attached unit.
    pub const fn factor(&self) -> f64 {
        match self {
            Self::Quecto => 1e-30,
            Self::Ronto  => 1e-27,
            Self::Yocto  => 1e-24,
            Self::Zepto  => 1e-21,
            Self::Atto   => 1e-18,
            Self::Femto  => 1e-15,
            Self::Pico   => 1e-12,
            Self::Nano   => 1e-9,
            Self::Micro  => 1e-6,
            Self::Milli  => 1e-3,
            Self::Centi  => 1e-2,
            Self::Deci   => 1e-1,

            Self::Deca   => 1e1,
            Self::Hecto  => 1e2,
            Self::Kilo   => 1e3,
            Self::Mega   => 1e6,
            Self::Giga   => 1e9,
            Self::Tera   => 1e12,
            Self::Peta   => 1e15,
            Self::Exa    => 1e18,
            Self::Zetta  => 1e21,
            Self::Yotta  => 1e24,
            Self::Ronna  => 1e27,
            Self::Quetta => 1e30,
        }
    }

    /// Return the standard SI symbol for this prefix.
    pub const fn symbol(&self) -> &'static str {
        match self {
            Self::Quecto => "q",
            Self::Ronto  => "r",
            Self::Yocto  => "y",
            Self::Zepto  => "z",
            Self::Atto   => "a",
            Self::Femto  => "f",
            Self::Pico   => "p",
            Self::Nano   => "n",
            Self::Micro  => "μ",
            Self::Milli  => "m",
            Self::Centi  => "c",
            Self::Deci   => "d",

            Self::Deca   => "da",
            Self::Hecto  => "h",
            Self::Kilo   => "k",
            Self::Mega   => "M",
            Self::Giga   => "G",
            Self::Tera   => "T",
            Self::Peta   => "P",
            Self::Exa    => "E",
            Self::Zetta  => "Z",
            Self::Yotta  => "Y",
            Self::Ronna  => "R",
            Self::Quetta => "Q",
        }
    }

    /// Return a single-`char` symbol for this prefix.
    pub const fn symbol_char(&self) -> char {
        match self {
            Self::Quecto => 'q',
            Self::Ronto  => 'r',
            Self::Yocto  => 'y',
            Self::Zepto  => 'z',
            Self::Atto   => 'a',
            Self::Femto  => 'f',
            Self::Pico   => 'p',
            Self::Nano   => 'n',
            Self::Micro  => 'μ',
            Self::Milli  => 'm',
            Self::Centi  => 'c',
            Self::Deci   => 'd',

            Self::Deca   => 'D',
            Self::Hecto  => 'h',
            Self::Kilo   => 'k',
            Self::Mega   => 'M',
            Self::Giga   => 'G',
            Self::Tera   => 'T',
            Self::Peta   => 'P',
            Self::Exa    => 'E',
            Self::Zetta  => 'Z',
            Self::Yotta  => 'Y',
            Self::Ronna  => 'R',
            Self::Quetta => 'Q',
        }
    }

    /// Return the provided unit, modified by this prefix.
    pub const fn unit<U: Unit>(self, unit: U) -> UnitPrefixed<U> {
        UnitPrefixed::new(unit, Some(self))
    }
}

impl Prefix {
    /// Return the next prefix down in the scale, or `None` if this is already
    ///     the smallest variant.
    pub const fn step_down(&self) -> Option<Self> {
        match self {
            Self::Quecto => None,
            Self::Ronto  => Some(Self::Quecto),
            Self::Yocto  => Some(Self::Ronto),
            Self::Zepto  => Some(Self::Yocto),
            Self::Atto   => Some(Self::Zepto),
            Self::Femto  => Some(Self::Atto),
            Self::Pico   => Some(Self::Femto),
            Self::Nano   => Some(Self::Pico),
            Self::Micro  => Some(Self::Nano),
            Self::Milli  => Some(Self::Micro),
            Self::Centi  => Some(Self::Milli),
            Self::Deci   => Some(Self::Centi),

            Self::Deca   => Some(Self::Deci),
            Self::Hecto  => Some(Self::Deca),
            Self::Kilo   => Some(Self::Hecto),
            Self::Mega   => Some(Self::Kilo),
            Self::Giga   => Some(Self::Mega),
            Self::Tera   => Some(Self::Giga),
            Self::Peta   => Some(Self::Tera),
            Self::Exa    => Some(Self::Peta),
            Self::Zetta  => Some(Self::Exa),
            Self::Yotta  => Some(Self::Zetta),
            Self::Ronna  => Some(Self::Yotta),
            Self::Quetta => Some(Self::Ronna),
        }
    }

    /// Return the next prefix up in the scale, or `None` if this is already the
    ///     largest variant.
    pub const fn step_up(&self) -> Option<Self> {
        match self {
            Self::Quecto => Some(Self::Ronto),
            Self::Ronto  => Some(Self::Yocto),
            Self::Yocto  => Some(Self::Zepto),
            Self::Zepto  => Some(Self::Atto),
            Self::Atto   => Some(Self::Femto),
            Self::Femto  => Some(Self::Pico),
            Self::Pico   => Some(Self::Nano),
            Self::Nano   => Some(Self::Micro),
            Self::Micro  => Some(Self::Milli),
            Self::Milli  => Some(Self::Centi),
            Self::Centi  => Some(Self::Deci),
            Self::Deci   => Some(Self::Deca),

            Self::Deca   => Some(Self::Hecto),
            Self::Hecto  => Some(Self::Kilo),
            Self::Kilo   => Some(Self::Mega),
            Self::Mega   => Some(Self::Giga),
            Self::Giga   => Some(Self::Tera),
            Self::Tera   => Some(Self::Peta),
            Self::Peta   => Some(Self::Exa),
            Self::Exa    => Some(Self::Zetta),
            Self::Zetta  => Some(Self::Yotta),
            Self::Yotta  => Some(Self::Ronna),
            Self::Ronna  => Some(Self::Quetta),
            Self::Quetta => None,
        }
    }
}

/*impl<U: Unit> FnOnce<(U,)> for Prefix {
    type Output = UnitPrefixed<U>;

    extern "rust-call" fn call_once(self, args: (U,)) -> Self::Output {
        args.0.prefixed(Some(self))
    }
}*/
