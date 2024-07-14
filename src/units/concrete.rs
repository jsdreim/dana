use super::traits::*;


macro_rules! concrete_types {
    ($($module:ident::$unit:ident),+$(,)?) => {
        $(
        pub mod $module;
        pub use $module::$unit;
        )+

        impl_unit_ops!($($unit),+);
        impl_unit_inv!($($unit),+);
        impl_unit_pow!($($unit),+);
        impl_unit_concrete!($($unit),+);
    };
}

concrete_types!(
    // one::One,

    length::Length,
    mass::Mass,
    time::Time,
    frequency::Frequency,

    force::Force,
    energy::Energy,
    power::Power,
    current::Current,
    voltage::Voltage,
    resistance::Resistance,
);

pub type Distance = Length;


dim_macros::impl_scale! {
    for Length impl (Nano, Micro, Milli, Kilo) Meter;
    // for Mass impl (Milli, Kilo) Gram;
    for Time impl (Milli) Second;
    for Frequency impl (Micro, Milli, Kilo, Mega, Giga, Tera) Hertz;

    for Force impl (Kilo, Mega, Giga) Newton;
    for Energy impl (Micro, Milli, Kilo, Mega, Giga, Tera) Joule;

    for Power impl (Micro, Milli, Kilo, Mega, Giga, Tera) Watt;
    for Current impl (Micro, Milli, Kilo, Mega, Giga, Tera) Amp;
    for Voltage impl (Micro, Milli, Kilo, Mega, Giga, Tera) Volt;
    for Resistance impl (Micro, Milli, Kilo, Mega, Giga, Tera) Ohm;
}
