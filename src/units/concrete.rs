use super::traits::*;


macro_rules! concrete_types {
    ($($module:ident::$unit:ident),+$(,)?) => {
        $(
        pub mod $module;
        pub use $module::$unit;
        )+

        impl_unit_ops!($($unit),+);
        impl_unit_pow!($($unit),+);
        impl_unit_pow_n!($($unit),+);
        impl_unit_concrete!($($unit),+);
    };
}

concrete_types!(
    distance::Distance,
    energy::Energy,
    force::Force,
    mass::Mass,
    time::Time,
);
