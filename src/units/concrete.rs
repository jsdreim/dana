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

pub type Length = Distance;


#[macro_export]
macro_rules! unit_from_symbol {
    ($t:ident) => {{
        #[allow(non_snake_case)]
        let $t = $crate::unit_from_symbol!(@$t);
        $t
    }};

    (@mm) => { $crate::units::concrete::Length::Millimeter };
    (@cm) => { $crate::units::concrete::Length::Centimeter };
    (@m) => { $crate::units::concrete::Length::Meter };
    (@km) => { $crate::units::concrete::Length::Kilometer };

    (@J) => { $crate::units::concrete::Energy::Joule };
    (@kJ) => { $crate::units::concrete::Energy::Kilojoule };

    (@N) => { $crate::units::concrete::Force::Newton };
    (@kN) => { $crate::units::concrete::Force::Kilonewton };

    (@g) => { $crate::units::concrete::Mass::Gram };
    (@kg) => { $crate::units::concrete::Mass::Kilogram };

    (@s) => { $crate::units::concrete::Time::Second };

    (@$t:tt) => { $t };
    ($t:tt) => { $t };
}
