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
        impl_unit_pow_n!($($unit),+);
        impl_unit_concrete!($($unit),+);
    };
}

concrete_types!(
    // one::One,

    distance::Distance,
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

pub type Length = Distance;
