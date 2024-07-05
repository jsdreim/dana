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


#[macro_export]
macro_rules! unit_from_symbol {
    ($t:ident) => {{
        #[allow(non_snake_case)]
        let $t = $crate::unit_from_symbol!(@$t);
        $t
    }};

    (@mm) => { $crate::units::Distance::Millimeter };
    (@cm) => { $crate::units::Distance::Centimeter };
    (@m) => { $crate::units::Distance::Meter };
    (@km) => { $crate::units::Distance::Kilometer };

    (@J) => { $crate::units::Energy::Joule };
    (@kJ) => { $crate::units::Energy::Kilojoule };

    (@N) => { $crate::units::Force::Newton };
    (@kN) => { $crate::units::Force::Kilonewton };

    (@g) => { $crate::units::Mass::Gram };
    (@kg) => { $crate::units::Mass::Kilogram };

    (@s) => { $crate::units::Time::Second };

    (@$t:tt) => { $t };
    ($t:tt) => { $t };
}
