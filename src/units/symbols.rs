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

    //  Define a group module, containing symbols from other modules.
    ($vis:vis mod $module:ident ($($import:ident),* $(,)?)) => {
        $vis mod $module {$(pub use super::$import::*;)*}
    };

    //  Internal: Definitions for consts.
    (@ $utype:ty; $alias:ident            = $variant:ident) => {
        pub const $alias: $utype = <$utype>::$variant;
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
define_symbols!(pub mod physics(basic, energy, frequency, force, momentum));
define_symbols!(pub mod electricity(power, current, voltage, resistance));


define_symbols! {
    pub mod speed for type Speed as v {
        const kph: (Length / Time) = (km/h);
    }

    pub mod accel for type Accel as a {}
    pub mod momentum for type Momentum as p {}

    pub mod length for type Length as l, L {
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

    pub mod force for type Force as F {
        const  N = Newton;
        const kN = KiloNewton;
        const MN = MegaNewton;
        const GN = GigaNewton;
    }

    pub mod energy for type Energy as E {
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
