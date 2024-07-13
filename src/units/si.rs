use crate::units::traits::*;


macro_rules! def_scale {
    ($(
    $(#[$attr:meta])*
    $id_trait:ident::$id_const:ident = $scale:literal;
    )*) => {
        $($(#[$attr])*
        pub trait $id_trait: UnitConcrete {
            const $id_const: Self;
            const SCALE: f64 = $scale;
        })*
    };
}

def_scale! {
    SiFemto::FEMTO = 1e-15;
    SiPico ::PICO  = 1e-12;
    SiNano ::NANO  = 1e-9;
    SiMicro::MICRO = 1e-6;
    SiMilli::MILLI = 1e-3;

    SiKilo ::KILO  = 1e+3;
    SiMega ::MEGA  = 1e+6;
    SiGiga ::GIGA  = 1e+9;
    SiTera ::TERA  = 1e+12;
    SiPeta ::PETA  = 1e+15;
    SiExa  ::EXA   = 1e+18;
}
