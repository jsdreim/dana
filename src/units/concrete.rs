use super::traits::*;


/// # New Unit Checklist
/// 1. Source file.
/// 2. Add to `concrete_types!` and `impl_scale!` calls below.
/// 3. Add relevant relationships in [`crate::units::transform::convert`].
/// 4. Add to [`crate::units::symbols`].
struct _Notes;


macro_rules! concrete_types {
    ($($module:ident::$unit:ident),+$(,)?) => {
        $(
        pub mod $module;
        pub use $module::$unit;
        )+

        // impl_unit_ops!($($unit),+);
        // impl_unit_inv!($($unit),+);
        // impl_unit_pow!($($unit),+);
        impl_unit_concrete!($($unit),+);

        $(
        /// Unit division.
        impl<U: $crate::Unit> ::std::ops::Div<U> for $unit where
            $crate::dim::$unit: ::std::ops::Div<<U as $crate::Unit>::Dim>,
            <$crate::dim::$unit as ::std::ops::Div<<U as $crate::Unit>::Dim>>::Output:
                $crate::dim::DimType,
        {
            type Output = $crate::units::UnitDiv<Self, U>;

            fn div(self, rhs: U) -> Self::Output {
                $crate::units::UnitDiv(self, rhs)
            }
        }

        /// Unit multiplication.
        impl<U: $crate::Unit> ::std::ops::Mul<U> for $unit where
            $crate::dim::$unit: ::std::ops::Mul<<U as $crate::Unit>::Dim>,
            <$crate::dim::$unit as ::std::ops::Mul<<U as $crate::Unit>::Dim>>::Output:
                $crate::dim::DimType,
        {
            type Output = $crate::units::UnitMul<Self, U>;

            fn mul(self, rhs: U) -> Self::Output {
                $crate::units::UnitMul(self, rhs)
            }
        }

        impl ::num_traits::Inv for $unit where
            $crate::dim::$unit: ::num_traits::Inv,
        {
            type Output = $crate::units::compound::PerUnit<Self>;

            fn inv(self) -> Self::Output {
                $crate::units::compound::PerUnit(self)
            }
        }

        /// Unit exponentiation.
        impl<E: ::typenum::Integer> $crate::units::traits::CanPow<E> for $unit where
            $crate::dim::$unit: $crate::dim::DimPowType<E>,
        {
            type Output = $crate::units::UnitPow<Self, E>;

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

    force::Force,
    energy::Energy,
    power::Power,
    charge::Charge,
    current::Current,
    voltage::Voltage,
    resistance::Resistance,
);

pub type Distance = Length;


impl_scale! {
    for Length impl (Nano, Micro, Milli, Kilo) Meter;
    // for Mass impl (Milli, Kilo) Gram;
    for Time impl (Milli) Second;
    for Frequency impl (Micro, Milli, Kilo, Mega, Giga, Tera) Hertz;
    for Temp impl (Micro, Milli, Kilo, Mega, Giga, Tera) Kelvin;

    for Force impl (Kilo, Mega, Giga) Newton;
    for Energy impl (Micro, Milli, Kilo, Mega, Giga, Tera) Joule;

    for Power impl (Micro, Milli, Kilo, Mega, Giga, Tera) Watt;
    for Charge impl (Micro, Milli, Kilo, Mega, Giga, Tera) Coulomb;
    for Current impl (Micro, Milli, Kilo, Mega, Giga, Tera) Amp;
    for Voltage impl (Micro, Milli, Kilo, Mega, Giga, Tera) Volt;
    for Resistance impl (Micro, Milli, Kilo, Mega, Giga, Tera) Ohm;
}
