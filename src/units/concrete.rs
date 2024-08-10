//! Module for [concrete unit](UnitConcrete) types.
//!
//! Concrete units are defined absolutely, and are not reducible, but they may
//!     have multiple variants with different scale factors. Every concrete unit
//!     type has a constant [base unit](UnitConcrete::BASE), and every concrete
//!     unit has a unique [symbol](UnitConcrete::symbol).

#![allow(missing_docs)]

use core::ops::{Div, Mul};
use crate::{
    dimension::{DimPowType, ExpHack, HasTypenum},
    units::{compound::*, traits::*},
};


/// # New Unit Checklist
/// 1. Source file.
/// 2. Add to `concrete_types!` and `impl_scale!` calls below.
/// 3. Add to [`crate::symbols`].
struct _Notes;


macro_rules! concrete_mod {
    //  Doc comment provided, use it directly.
    ($(#[$attr:meta])+ $vis:vis use $module:ident::$unit:ident;) => {
        $(#[$attr])+
        $vis mod $module;
        $vis use $module::$unit;
    };
    //  Doc comment not provided, generate one.
    ($vis:vis use $module:ident::$unit:ident;) => {
        #[doc = concat!(
            "Module for the ",
            "[`", stringify!($unit), "`]",
            "(", stringify!($module), "::", stringify!($unit), ")",
            " concrete unit type.",
        )]
        $vis mod $module;
        $vis use $module::$unit;
    };
}

macro_rules! concrete_types {
    ($(
    $(#[$attr:meta])*
    $module:ident::$unit:ident
    ),+$(,)?) => {
        $(concrete_mod! {
            $(#[$attr])*
            pub use $module::$unit;
        })+

        /// Module to re-export all concrete unit types.
        pub mod types {
            pub use super::{$($module::$unit,)+};
        }

        $(
        impl Default for $unit {
            fn default() -> Self { Self::BASE }
        }

        impl ::core::fmt::Display for $unit {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <str as core::fmt::Display>::fmt(self.symbol(), f)
            }
        }

        //  Unit division.
        impl<U: Unit> Div<U> for $unit where Self: CanUnitDiv<U> {
            type Output = UnitDiv<Self, U>;

            fn div(self, rhs: U) -> Self::Output {
                UnitDiv::new(self, rhs)
            }
        }

        //  Unit multiplication.
        impl<U: Unit> Mul<U> for $unit where Self: CanUnitMul<U> {
            type Output = UnitMul<Self, U>;

            fn mul(self, rhs: U) -> Self::Output {
                UnitMul::new(self, rhs)
            }
        }

        //  Unit inversion.
        impl ::num_traits::Inv for $unit where Self: CanUnitInv {
            type Output = PerUnit<Self>;

            fn inv(self) -> Self::Output {
                PerUnit::new(self)
            }
        }

        //  Unit exponentiation.
        impl<const E: i32> CanPow<E> for $unit where
            ExpHack<E>: HasTypenum,
            Self::Dim: DimPowType<<ExpHack<E> as HasTypenum>::Typenum>,
        {
            type Output = UnitPow<Self, <ExpHack<E> as HasTypenum>::Typenum>;

            fn pow(self) -> Self::Output {
                UnitPow::new(self)
            }
        }
        )+
    };
}

concrete_types!(
    one::One,

    length::Length,
    mass::Mass,
    time::Time,
    frequency::Frequency,

    temp::Temp,
    amount::Amount,
    intensity::Intensity,

    force::Force,
    volume::Volume,
    pressure::Pressure,

    energy::Energy,
    power::Power,
    charge::Charge,
    current::Current,
    voltage::Voltage,
    resistance::Resistance,
);


/*impl_scale! {
    for Length impl (Pico, Nano, Micro, Milli, Kilo) Meter;
    // for Mass impl (Milli, Kilo) Gram;
    for Time impl (Pico, Nano, Micro, Milli) Second;
    for Frequency impl (Micro, Milli, Kilo, Mega, Giga, Tera) Hertz;

    for Temp impl (Micro, Milli, Kilo, Mega, Giga, Tera) Kelvin;
    for Amount impl (Micro, Milli, Kilo, Mega, Giga, Tera) Mole;
    for Intensity impl (Micro, Milli, Kilo, Mega, Giga, Tera) Candela;

    for Force impl (Kilo, Mega, Giga) Newton;
    for Volume impl (Micro, Milli, Kilo, Mega, Giga, Tera) Liter;
    for Pressure impl (Micro, Milli, Kilo, Mega, Giga, Tera) Pascal;

    for Energy impl (Micro, Milli, Kilo, Mega, Giga, Tera) Joule;
    for Power impl (Micro, Milli, Kilo, Mega, Giga, Tera) Watt;
    for Charge impl (Micro, Milli, Kilo, Mega, Giga, Tera) Coulomb;
    for Current impl (Micro, Milli, Kilo, Mega, Giga, Tera) Amp;
    for Voltage impl (Micro, Milli, Kilo, Mega, Giga, Tera) Volt;
    for Resistance impl (Micro, Milli, Kilo, Mega, Giga, Tera) Ohm;
}*/
