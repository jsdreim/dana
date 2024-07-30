use crate::units::traits::*;


macro_rules! def_scale {
    ($(
    $(#[$attr:meta])*
    $id_trait:ident::$id_const:ident = $scale:literal;
    )*) => {
        $($(#[$attr])*
        pub trait $id_trait: UnitConcrete {
            /// The scaled unit of this type.
            const $id_const: Self;
            /// The scale factor of the prefix.
            const SCALE: f64 = $scale;
        })*
    };
}

def_scale! {
    /// The SI prefix "Femto-" scales a unit by 1e-15.
    SiFemto::FEMTO = 1e-15;
    /// The SI prefix "Pico-" scales a unit by 1e-12.
    SiPico ::PICO  = 1e-12;
    /// The SI prefix "Nano-" scales a unit by 1e-9.
    SiNano ::NANO  = 1e-9;
    /// The SI prefix "Micro-" scales a unit by 1e-6.
    SiMicro::MICRO = 1e-6;
    /// The SI prefix "Milli-" scales a unit by 1e-3.
    SiMilli::MILLI = 1e-3;

    /// The SI prefix "Kilo-" scales a unit by 1e+3.
    SiKilo ::KILO  = 1e+3;
    /// The SI prefix "Mega-" scales a unit by 1e+6.
    SiMega ::MEGA  = 1e+6;
    /// The SI prefix "Giga-" scales a unit by 1e+9.
    SiGiga ::GIGA  = 1e+9;
    /// The SI prefix "Tera-" scales a unit by 1e+12.
    SiTera ::TERA  = 1e+12;
    /// The SI prefix "Peta-" scales a unit by 1e+15.
    SiPeta ::PETA  = 1e+15;
    /// The SI prefix "Exa-" scales a unit by 1e+18.
    SiExa  ::EXA   = 1e+18;
}
