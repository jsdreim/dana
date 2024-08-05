//! Module for [concrete unit](UnitConcrete) types.
//!
//! Concrete units are defined absolutely, and are not reducible, but they may
//!     have multiple variants with different scale factors. Every concrete unit
//!     type has a constant [base unit](UnitConcrete::BASE), and every concrete
//!     unit has a unique [symbol](UnitConcrete::symbol).

#![allow(missing_docs)]

use super::traits::*;


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

        /// Unit division.
        impl<U: $crate::Unit> ::core::ops::Div<U> for $unit where
            $crate::dimension::$unit: ::core::ops::Div<<U as $crate::Unit>::Dim>,
            <$crate::dimension::$unit as ::core::ops::Div<<U as $crate::Unit>::Dim>>::Output:
                $crate::dimension::DimType,
        {
            type Output = $crate::units::UnitDiv<Self, U>;

            fn div(self, rhs: U) -> Self::Output {
                $crate::units::UnitDiv::new(self, rhs)
            }
        }

        /// Unit multiplication.
        impl<U: $crate::Unit> ::core::ops::Mul<U> for $unit where
            $crate::dimension::$unit: ::core::ops::Mul<<U as $crate::Unit>::Dim>,
            <$crate::dimension::$unit as ::core::ops::Mul<<U as $crate::Unit>::Dim>>::Output:
                $crate::dimension::DimType,
        {
            type Output = $crate::units::UnitMul<Self, U>;

            fn mul(self, rhs: U) -> Self::Output {
                $crate::units::UnitMul::new(self, rhs)
            }
        }

        /// Unit inversion.
        impl ::num_traits::Inv for $unit where
            $crate::dimension::$unit: ::num_traits::Inv,
        {
            type Output = $crate::units::compound::PerUnit<Self>;

            fn inv(self) -> Self::Output {
                $crate::units::compound::PerUnit::new(self)
            }
        }

        /// Unit exponentiation.
        impl<const E: i32> $crate::units::traits::CanPow<E> for $unit where
            $crate::dimension::ExpHack<E>: $crate::dimension::HasTypenum,
            $crate::dimension::$unit: $crate::dimension::DimPowType<
                <$crate::dimension::ExpHack<E> as $crate::dimension::HasTypenum>::Typenum,
            >,
        {
            type Output = $crate::units::UnitPow<Self, <$crate::dimension::ExpHack<E> as $crate::dimension::HasTypenum>::Typenum>;

            fn pow(self) -> Self::Output {
                $crate::units::UnitPow::new(self)
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
