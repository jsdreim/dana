//! This module defines symbols: Shortcut labels to dimensions and units. These
//!     serve to make definitions clearer to read and easier to write. Examples
//!     include `m` for [`Length::Meter`] and `E` for [`Energy`].
//!
//! Also included are symbols for standard compound units, such as [`kWh`] for
//!     Kilowatt-hours (energy as the product of power and time).
//!
//! Symbols for dimensions are implemented as type aliases, and those for units
//!     are implemented as constants. A convenient side effect of this is that
//!     IDEs with robust analysis may render them in different colors, for even
//!     more clarity.

#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::units::*;


macro_rules! define_symbols {
    //  Define only type symbols.
    ($($vis:vis type $unit:ident as $($uname:ident),+;)*) => {
        $($($vis type $uname = $unit;)+)*
    };

    //  Define type modules, with symbols for types and their variants.
    ($($vis:vis mod $module:ident for type $unit:tt $(as $($uname:ident),+)? {
        $(const $alias:ident $(: $atype:tt)? = $val:tt);*   $(;)?
    })*) => {
        $($vis use self::$module::*;
        $vis mod $module {
            #[allow(unused_imports)]
            use crate::units::concrete::*;
            $($(pub type $uname = super::$unit;)+)?
            $(define_symbols!(@ super::$unit; $alias $(: $atype)? = $val);)*
        })*
    };

    //  Define type modules, and then import everything from all of them.
    (use; $($vis:vis mod $module:ident for type $u:tt $(as $($n:ident),+)? {$($b:tt)*})*) => {
        $(
        $vis use self::$module::*;
        define_symbols!($vis mod $module for type $u $(as $($n),+)? {$($b)*});
        )*
    };

    //  Define a group module, containing symbols from other modules.
    ($vis:vis mod $module:ident ($($import:ident),* $(,)?)) => {
        $vis mod $module {$(pub use super::$import::*;)*}
    };

    //  Internal: Definitions for consts.
    (@ $utype:ty; $alias:ident            = $variant:ident) => {
        pub const $alias: $utype = <$utype>::$variant;
    };
    (@ $utype:ty; $alias:ident            = [$($unit:tt)*]) => {
        pub const $alias: $utype = {
            use super::*;
            $crate::unit!($($unit)*)
        };
    };
    (@ $utype:ty; $alias:ident            = $value:expr) => {
        pub const $alias: $utype = $value;
    };
    (@ $utype:ty; $alias:ident: $atype:tt = $($unit:tt)*) => {
        pub const $alias: $crate::utype!($atype) = {
            use super::*;
            $crate::unit!($($unit)*)
        };
    };
}


pub mod common {
    pub use super::{
        accel::{a},
        length::{L, m, km},
        mass::{M, kg},
        time::{T, s, h},
    };
}


define_symbols!(pub mod basic(length, mass, time, speed, accel));
// define_symbols!(pub mod geometric(length, area, volume));
define_symbols!(pub mod physical(basic, energy, frequency, force, momentum, temp));
define_symbols!(pub mod electrical(power, charge, current, voltage, resistance));


// define_symbols! {
//     pub mod area for type Area as A {}
//     pub mod volume for type Volume as V {}
//     pub mod density for type Density as D {}
//     pub mod pressure for type Pressure as P {
//         const  Pa = [ N / m^2];
//         const kPa = [kN / m^2];
//         const MPa = [MN / m^2];
//         const GPa = [GN / m^2];
//     }
// }


define_symbols! {
    // use;

    pub mod speed for type Speed as v {
        const kph: (Length / Time) = (km/h);
    }

    pub mod accel for type Accel as a {}
    pub mod momentum for type Momentum as p {}

    pub mod length for type Length as l, L {
        const nm = NanoMeter;
        const um = MicroMeter;
        const mm = MilliMeter;
        const cm = CentiMeter;
        const  m = Meter;
        const km = KiloMeter;
    }

    pub mod mass for type Mass as m, M {
        const mg = MilliGram;
        const  g = Gram;
        const kg = KiloGram;
        const  T = MetricTon;
    }

    pub mod time for type Time as t, T {
        const ms   = MilliSecond;
        const  s   = Second;
        const  min = Minute;
        const  h   = Hour;
    }

    pub mod frequency for type Frequency as f {
        const uHz = MicroHertz;
        const mHz = MilliHertz;
        const  Hz = Hertz;
        const kHz = KiloHertz;
        const MHz = MegaHertz;
        const GHz = GigaHertz;
        const THz = TeraHertz;
    }

    pub mod temp for type Temperature as K, Θ {
        const uK = MicroKelvin;
        const mK = MilliKelvin;
        const  K = Kelvin;
        const kK = KiloKelvin;
        const MK = MegaKelvin;
        const GK = GigaKelvin;
        const TK = TeraKelvin;
    }

    pub mod force for type Force as F {
        const  N = Newton;
        const kN = KiloNewton;
        const MN = MegaNewton;
        const GN = GigaNewton;
    }

    pub mod energy for type Energy as E {
        const eV = ElectronVolt;

        const uJ = MicroJoule;
        const mJ = MilliJoule;
        const  J = Joule;
        const kJ = KiloJoule;
        const MJ = MegaJoule;
        const GJ = GigaJoule;
        const TJ = TeraJoule;

        const Wh: (Power * Time) = (W * h);
        const kWh: (Power * Time) = (kW * h);
    }

    pub mod power for type Power as P {
        const uW = MicroWatt;
        const mW = MilliWatt;
        const  W = Watt;
        const kW = KiloWatt;
        const MW = MegaWatt;
        const GW = GigaWatt;
        const TW = TeraWatt;
    }

    pub mod charge for type Charge as q, Q {
        const uC = MicroCoulomb;
        const mC = MilliCoulomb;
        const  C = Coulomb;
        const kC = KiloCoulomb;
        const MC = MegaCoulomb;
        const GC = GigaCoulomb;
        const TC = TeraCoulomb;

        const Ah: (Current * Time) = (A * h);
        const mAh: (Current * Time) = (mA * h);
    }

    pub mod current for type Current as I {
        const uA = MicroAmp;
        const mA = MilliAmp;
        const  A = Amp;
        const kA = KiloAmp;
        const MA = MegaAmp;
        const GA = GigaAmp;
        const TA = TeraAmp;
    }

    pub mod voltage for type Voltage as V {
        const uV = MicroVolt;
        const mV = MilliVolt;
        const  V = Volt;
        const kV = KiloVolt;
        const MV = MegaVolt;
        const GV = GigaVolt;
        const TV = TeraVolt;
    }

    //  TODO: Greek letters acceptable?
    //  TODO: Keep `u` as prefix for micro? Switch to `μ`? Allow both?
    pub mod resistance for type Resistance as R {
        const uΩ = MicroOhm;
        const mΩ = MilliOhm;
        const  Ω = Ohm;
        const kΩ = KiloOhm;
        const MΩ = MegaOhm;
        const GΩ = GigaOhm;
        const TΩ = TeraOhm;
    }
}
