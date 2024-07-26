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
            $(define_symbols!(@ super::$unit; $alias $(: $atype)? = $val);)*
        })*

        pub use types::*;
        pub mod types {
            $($($(pub type $uname = super::$unit;)+)?)*
        }

        // pub mod units {
        //     $($(define_symbols!(@ super::$unit; $alias $(: $atype)? = $val);)*)*
        // }
    };

    //  Define type modules, and then import everything from all of them.
    (use; $($vis:vis mod $module:ident for type $u:tt $(as $($n:ident),+)? {$($b:tt)*})*) => {
        $(
        $vis use self::$module::*;
        define_symbols!($vis mod $module for type $u $(as $($n),+)? {$($b)*});
        )*
    };

    //  Define a group module, containing symbols from other modules.
    ($(#[$attr:meta])*
    $vis:vis mod $module:ident ($($import:tt),* $(,)?)) => {
        $(#[$attr])*
        $vis mod $module {
            $(define_symbols!(# $import);)*
        }
    };

    ($($(#[$attr:meta])*
    $vis:vis mod $module:ident ($($import:tt),* $(,)?);)*) => {
        $(define_symbols!($(#[$attr])* $vis mod $module ($($import),*));)*
    };

    //  Internal: Imports.
    (# [$($utype:ident),* $(,)?]) => { pub use super::types::{$($utype),*}; };
    (# $module:ident) => { pub use super::$module::*; };

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
        types::{L, M, T},
        length_si::{m, km},
        mass::{kg},
        time::{s, h},
    };
}


define_symbols! {
    /// Unit symbols for basic dimensions: Length, mass, time, and speed.
    pub mod basic(
        [L],       [M],  [T],  [v],
        length_si, mass, time, speed,
    );

    // /// Unit symbols for units often used in geometry.
    // pub mod geometric(length, area, volume);

    /// Unit symbols for units often used in chemistry.
    pub mod chemical(
        [M],  [T],  [E],    [K,Θ], [N],
        mass, time, energy, temp,  amount,
    );

    /// Unit symbols for units often used in physics.
    pub mod physical(
        basic,
        [E],    [f],       [F],      [K,Θ], [J],
        energy, frequency, force_si, temp,  intensity,
    );

    /// Unit symbols related to electricity.
    pub mod electrical(
        [P],   [Q],    [I],     [V],     [R],
        power, charge, current, voltage, resistance,
    );

    /// Unit symbols for the ISQ base quantities.
    pub mod isq(
        [L],       [M],  [T],  [I],     [K,Θ], [N],    [J],
        length_si, mass, time, current, temp,  amount, intensity,
    );

    /// Unit symbols for the Imperial system.
    pub mod imperial(
        [L],             [T],  [F],
        length_imperial, time, force_imperial,
    );
}


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


//  TODO: Greek letters acceptable?
define_symbols! {
    // use;

    pub mod speed for type Speed as v {
        const kph: (Length / Time) = (km/h);
        const mph: (Length / Time) = (mi/h);
    }

    // pub mod accel for type Accel as a {}
    // pub mod momentum for type Momentum as p {}

    pub mod length_si for type Length as L {
        const nm = NanoMeter;
        const μm = MicroMeter;
        const um = MicroMeter;
        const mm = MilliMeter;
        const cm = CentiMeter;
        const  m = Meter;
        const km = KiloMeter;
    }

    pub mod length_imperial for type Length {
        // const in = Inch; // ...Hmm.
        const inch = Inch;
        const ft = Foot;
        const yd = Yard;
        const mi = Mile;
    }

    pub mod length_space for type Length {
        const AU = AstroUnit;
        const ls = LightSec;
        const ly = LightYear;

        const  pc = Parsec;
        const kpc = KiloParsec;
        const Mpc = MegaParsec;
        const Gpc = GigaParsec;
    }

    pub mod mass for type Mass as M {
        const pg = PicoGram;
        const ng = NanoGram;
        const μg = MicroGram;
        const ug = MicroGram;
        const mg = MilliGram;
        const  g = Gram;
        const kg = KiloGram;

        const  T = MetricTon;
        const kT = KiloTon;
        const MT = MegaTon;
        const GT = GigaTon;
    }

    pub mod time for type Time as T {
        const ms   = MilliSecond;
        const  s   = Second;
        const  min = Minute;
        const  h   = Hour;
    }

    pub mod frequency for type Frequency as f {
        const μHz = MicroHertz;
        const uHz = MicroHertz;
        const mHz = MilliHertz;
        const  Hz = Hertz;
        const kHz = KiloHertz;
        const MHz = MegaHertz;
        const GHz = GigaHertz;
        const THz = TeraHertz;
    }

    pub mod temp for type Temp as K, Θ {
        const μK = MicroKelvin;
        const uK = MicroKelvin;
        const mK = MilliKelvin;
        const  K = Kelvin;
        const kK = KiloKelvin;
        const MK = MegaKelvin;
        const GK = GigaKelvin;
        const TK = TeraKelvin;
    }

    pub mod amount for type Amount as N {
        const μmol = MicroMole;
        const umol = MicroMole;
        const mmol = MilliMole;
        const  mol = Mole;
        const kmol = KiloMole;
        const Mmol = MegaMole;
        const Gmol = GigaMole;
        const Tmol = TeraMole;
    }

    pub mod intensity for type Intensity as J {
        const μcd = MicroCandela;
        const ucd = MicroCandela;
        const mcd = MilliCandela;
        const  cd = Candela;
        const kcd = KiloCandela;
        const Mcd = MegaCandela;
        const Gcd = GigaCandela;
        const Tcd = TeraCandela;
    }

    pub mod force_si for type Force as F {
        const  N = Newton;
        const kN = KiloNewton;
        const MN = MegaNewton;
        const GN = GigaNewton;
    }

    pub mod force_imperial for type Force {
        const ozf = Ounce;
        const lbf = Pound;
    }

    pub mod energy for type Energy as E {
        const eV = ElectronVolt;

        const μJ = MicroJoule;
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
        const μW = MicroWatt;
        const uW = MicroWatt;
        const mW = MilliWatt;
        const  W = Watt;
        const kW = KiloWatt;
        const MW = MegaWatt;
        const GW = GigaWatt;
        const TW = TeraWatt;
    }

    pub mod charge for type Charge as q, Q {
        const μC = MicroCoulomb;
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
        const μA = MicroAmp;
        const uA = MicroAmp;
        const mA = MilliAmp;
        const  A = Amp;
        const kA = KiloAmp;
        const MA = MegaAmp;
        const GA = GigaAmp;
        const TA = TeraAmp;
    }

    pub mod voltage for type Voltage as V {
        const μV = MicroVolt;
        const uV = MicroVolt;
        const mV = MilliVolt;
        const  V = Volt;
        const kV = KiloVolt;
        const MV = MegaVolt;
        const GV = GigaVolt;
        const TV = TeraVolt;
    }

    pub mod resistance for type Resistance as R {
        const μΩ = MicroOhm;
        const uΩ = MicroOhm;
        const mΩ = MilliOhm;
        const  Ω = Ohm;
        const kΩ = KiloOhm;
        const MΩ = MegaOhm;
        const GΩ = GigaOhm;
        const TΩ = TeraOhm;

        const μO = MicroOhm;
        const uO = MicroOhm;
        const mO = MilliOhm;
        const  O = Ohm;
        const kO = KiloOhm;
        const MO = MegaOhm;
        const GO = GigaOhm;
        const TO = TeraOhm;
    }
}
