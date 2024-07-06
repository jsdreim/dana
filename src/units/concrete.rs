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
    distance::Distance,
    mass::Mass,
    time::Time,

    force::Force,
    energy::Energy,
    power::Power,
    current::Current,
    voltage::Voltage,
    resistance::Resistance,
);

pub type Length = Distance;


#[macro_export]
macro_rules! unit_from_symbol {
    // ($t:ident) => { $crate::unit_from_symbol!(@$t) };
    ($t:ident) => {{
        //  NOTE: This hints to code analysis tools that the ident has a type.
        #[allow(non_snake_case)]
        let $t = $crate::unit_from_symbol!(@$t);
        $t
    }};

    (@mm) => { $crate::units::concrete::Length::Millimeter };
    (@cm) => { $crate::units::concrete::Length::Centimeter };
    (@m) => { $crate::units::concrete::Length::Meter };
    (@km) => { $crate::units::concrete::Length::Kilometer };

    (@g) => { $crate::units::concrete::Mass::Gram };
    (@kg) => { $crate::units::concrete::Mass::Kilogram };

    (@ms) => { $crate::units::concrete::Time::Millisecond };
    (@s) => { $crate::units::concrete::Time::Second };
    // (@M) => { $crate::units::concrete::Time::Minute };
    (@h) => { $crate::units::concrete::Time::Hour };

    (@N) => { $crate::units::concrete::Force::Newton };
    (@kN) => { $crate::units::concrete::Force::KiloNewton };

    (@kWh) => { $crate::units::concrete::Energy::KWH };
    (@mJ) => { $crate::units::concrete::Energy::MilliJoule };
    (@J) => { $crate::units::concrete::Energy::Joule };
    (@kJ) => { $crate::units::concrete::Energy::KiloJoule };

    (@mW) => { $crate::units::concrete::Power::MilliWatt };
    (@W) => { $crate::units::concrete::Power::Watt };
    (@kW) => { $crate::units::concrete::Power::KiloWatt };

    (@mA) => { $crate::units::concrete::Current::MilliAmp };
    (@A) => { $crate::units::concrete::Current::Amp };
    (@kA) => { $crate::units::concrete::Current::KiloAmp };

    (@mV) => { $crate::units::concrete::Voltage::MilliVolt };
    (@V) => { $crate::units::concrete::Voltage::Volt };
    (@kV) => { $crate::units::concrete::Voltage::KiloVolt };

    //  TODO: Greek letters acceptable? "Ohm" will not compile and using "O" is
    //      just not right.
    (@mΩ) => { $crate::units::concrete::Resistance::MilliOhm };
    (@Ω) => { $crate::units::concrete::Resistance::Ohm };
    (@kΩ) => { $crate::units::concrete::Resistance::KiloOhm };

    (@$t:tt) => { $t };
    ($t:tt) => { $t };
}
