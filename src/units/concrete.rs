use super::traits::*;


/// # New Unit Checklist
/// 1. Source file.
/// 2. Add to `concrete_types!` and `impl_scale!` calls below.
/// 3. Add to [`crate::symbols`].
struct _Notes;


macro_rules! concrete_types {
    ($($module:ident::$unit:ident),+$(,)?) => {
        $(
        pub mod $module;
        pub use $module::$unit;
        )+

        impl_unit_concrete!($($unit),+);

        $(
        /// Unit division.
        impl<U: $crate::Unit> ::std::ops::Div<U> for $unit where
            $crate::dimension::$unit: ::std::ops::Div<<U as $crate::Unit>::Dim>,
            <$crate::dimension::$unit as ::std::ops::Div<<U as $crate::Unit>::Dim>>::Output:
                $crate::dimension::DimType,
        {
            type Output = $crate::units::UnitDiv<Self, U>;

            fn div(self, rhs: U) -> Self::Output {
                $crate::units::UnitDiv(self, rhs)
            }
        }

        /// Unit multiplication.
        impl<U: $crate::Unit> ::std::ops::Mul<U> for $unit where
            $crate::dimension::$unit: ::std::ops::Mul<<U as $crate::Unit>::Dim>,
            <$crate::dimension::$unit as ::std::ops::Mul<<U as $crate::Unit>::Dim>>::Output:
                $crate::dimension::DimType,
        {
            type Output = $crate::units::UnitMul<Self, U>;

            fn mul(self, rhs: U) -> Self::Output {
                $crate::units::UnitMul(self, rhs)
            }
        }

        impl ::num_traits::Inv for $unit where
            $crate::dimension::$unit: ::num_traits::Inv,
        {
            type Output = $crate::units::compound::PerUnit<Self>;

            fn inv(self) -> Self::Output {
                $crate::units::compound::PerUnit(self)
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

        /*impl<__E: $crate::units::Exp $($(, $tv: $t0 $(+ $t1)*)+)?>
        $crate::units::traits::CanPow<__E>
        for $unit$(<$($tv),+>)? where
        {
            type Output = $crate::units::UnitPow<Self, __E>;
            fn pow(self) -> Self::Output { $crate::units::UnitPow::new(self) }
        }*/
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

pub type Distance = Length;


impl_scale! {
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
}
