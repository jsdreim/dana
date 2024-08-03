//! Module of aliases for dimensions and units, based on [SI] symbols.
//!
//! [SI]: https://en.wikipedia.org/wiki/International_System_of_Units
//!
//! These serve to make complex definitions clearer to read and easier to write.
//!     Examples include [`m`] for [`Length::Meter`] and [`E`] for [`Energy`].
//!     Also included are symbols for standard compound units, such as [`kWh`]
//!     for Kilowatt-hours (energy as the product of power and time).
//!
//! Symbols for dimensions are implemented as type aliases, and those for units
//!     are implemented as constants. A convenient side effect of this is that
//!     IDEs with robust analysis may render them in different colors, for even
//!     more clarity.
//!
//! ---
//!
//! Contains two broad classes of submodules:
//! - **Symbol Modules** define type aliases and unit constants. Inside each
//!     Symbol Module, type aliases are defined in `types`, and unit constants
//!     are defined in `units`. The Symbol Module itself also re-exports
//!     everything from both.
//! - **Group Modules** re-export the contents of multiple Symbol Modules that
//!     are often used together.
//!
//! There are also two unique submodules:
//! - [`dimensions`] defines type aliases for [`Dimension`]s.
//! - [`types`] re-exports every type alias defined in every Symbol Module.
//!
//! [`Dimension`]: crate::dimension::Dimension
//!
//! ---
//!
//! This module also re-exports every type alias and unit constant from every
//!     Symbol Module, so you can import everything at once:
//! ```rust
//! use dana::symbols::*;
//! ```
//! This is not recommended, as there are quite a lot of symbols.
//  TODO: Consider removing top level re-exports. Forcing every alias into the
//      same namespace makes it untenable to have `V` for volume without getting
//      rid of `V` for voltage.

#![allow(non_camel_case_types, non_upper_case_globals)]

#[macro_use]
mod macros;

use crate::units::*;


pub use dimensions::*;
pub mod dimensions {
    //! Symbols for specific [`Dimension`] types.

    use crate::dimension::*;

    //region Fundamental dimensions.
    /// [`Dimension`] alias for [`Length`].
    pub type _L = Length;
    /// [`Dimension`] alias for [`Mass`].
    pub type _M = Mass;
    /// [`Dimension`] alias for [`Time`].
    pub type _T = Time;
    /// [`Dimension`] alias for [`Current`].
    pub type _I = Current;
    /// [`Dimension`] alias for [`Temp`].
    pub type _K = Temp;
    /// [`Dimension`] alias for [`Temp`].
    pub type _Θ = Temp;
    /// [`Dimension`] alias for [`Amount`].
    pub type _N = Amount;
    /// [`Dimension`] alias for [`Intensity`].
    pub type _J = Intensity;
    //endregion

    //region Derived dimensions.
    //  TODO: How many of these should be provided (if any at all)?
    /*pub type _f = Frequency;
    pub type _v = Velocity;
    pub type _a = Accel;
    pub type _F = Force;
    // pub type _p = Pressure;
    // pub type _A = Area;
    // pub type _V = Volume;
    pub type _D = Density;
    pub type _ρ = Density;

    pub type _Q = Charge;
    pub type _τ = Torque;
    pub type _E = Energy;
    pub type _P = Power;
    pub type _V = Voltage;
    // pub type _U = Voltage;
    pub type _R = Resistance;
    pub type _C = Capacitance;*/
    //endregion
}


/// Group module for a minimal set of the most common units.
pub mod common {
    pub use super::{
        length_si::{L, m, km},
        mass_si::{M, kg},
        time::{T, s, h},
    };
}


define_groups! {
    /// Group module for basic dimensions: Length, mass, time, and speed.
    pub mod basic(
        [L],       [M],     [T],  /*[v],*/
        length_si, mass_si, time, speed,
    );

    /// Group module for units often used in chemistry.
    pub mod chemistry(
        [M],     [T],  [E],    [K,Θ], [N],
        mass_si, time, energy, temp,  amount,
    );

    /// Group module for units often used in physics.
    pub mod physics(
        (basic),
        [E],    [f],       [F],                [K,Θ], [J],
        energy, frequency, force_si, pressure, temp,  intensity,
    );

    /// Group module for units related to electricity.
    pub mod electrical(
        [P],   [Q],    [I],     [U,V],   [R],
        power, charge, current, voltage, resistance,
    );

    /// Group module for the ISQ base quantities.
    pub mod isq(
        [L],       [M],     [T],  [I],     [K,Θ], [N],    [J],
        length_si, mass_si, time, current, temp,  amount, intensity,
    );

    /// Group module for the [US Customary] system.
    ///
    /// [US Customary]: https://en.wikipedia.org/wiki/United_States_customary_units
    pub mod us_customary(
        [L],       [M],     /*[V],*/
        length_us, mass_us, volume_us,
    );
}


//  TODO: Greek letters acceptable?
define_symbols! {
    /// Symbol module for [`Speed`].
    pub mod speed for type Speed /*as v*/ {
        /// Unit alias for [`KiloMeter`s](Length::KiloMeter) per
        ///     [`Hour`](Time::Hour).
        const kph = (km/h);

        /// Unit alias for [`Mile`s](Length::Mile) per [`Hour`](Time::Hour).
        const mph = (mi/h);

        // /// Unit alias for [feet](Length::Foot) per [`Second`](Time::Second).
        // const fps = (ft/s);
    }

    /// Symbol module for [`Length`] units in the SI.
    pub mod length_si for type Length as L {
        const nm = NanoMeter;
        const μm = MicroMeter;
        const um = MicroMeter;
        const mm = MilliMeter;
        const cm = CentiMeter;
        const  m = Meter;
        const km = KiloMeter;
    }

    /// Symbol module for [`Length`] units in the US Customary system.
    pub mod length_us for type Length in mod length_si {
        // const in = Inch; // ...Hmm.
        const inch = Inch;
        const ft = Foot;
        const yd = Yard;
        const mi = Mile;
    }

    /// Symbol module for extremely large [`Length`] units used in astronomy.
    pub mod length_huge for type Length in mod length_si {
        use length_si;

        const AU = AstroUnit;
        const ls = LightSec;
        const ly = LightYear;

        const  pc = Parsec;
        const kpc = KiloParsec;
        const Mpc = MegaParsec;
        const Gpc = GigaParsec;
    }

    /// Symbol module for [`Mass`] units in the SI.
    pub mod mass_si for type Mass as M {
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

    /// Symbol module for [`Mass`] units in the US Customary system.
    pub mod mass_us for type Mass in mod mass_si {
        const gr = Grain;
        const oz = Ounce;
        const lb = Pound;
    }

    /// Symbol module for [`Time`].
    pub mod time for type Time as T {
        const ps   = PicoSecond;
        const ns   = NanoSecond;
        const μs   = MicroSecond;
        const us   = MicroSecond;
        const ms   = MilliSecond;
        const  s   = Second;
        const  min = Minute;
        const  h   = Hour;
    }

    /// Symbol module for [`Frequency`].
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

    /// Symbol module for [`Temp`].
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

    /// Symbol module for [`Amount`].
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

    /// Symbol module for [`Intensity`].
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

    /// Symbol module for [`Force`] units in the SI.
    pub mod force_si for type Force as F {
        const  N = Newton;
        const kN = KiloNewton;
        const MN = MegaNewton;
        const GN = GigaNewton;
    }

    /// Symbol module for [`Force`] units in the US Customary system.
    pub mod force_us for type Force in mod force_si {
        const ozf = Ounce;
        const lbf = Pound;
    }

    // pub mod area_us for type Area as A /*, S*/ {
    //     const sq_in = (inch^2);
    //     const sq_ft = (ft^2);
    // }

    /// Symbol module for [`Volume`] units in the SI.
    pub mod volume_si for type Volume {
        // type V = Volume; // TODO

        const μL = MicroLiter;
        const uL = MicroLiter;
        const mL = MilliLiter;
        const  L = Liter;
        const kL = KiloLiter;
        const ML = MegaLiter;
        const GL = GigaLiter;
        const TL = TeraLiter;
    }

    /// Symbol module for [`Volume`] units in the US Customary system.
    pub mod volume_us for type Volume in mod volume_si {
        const fl_dr = Dram;
        const fl_oz = FlOunce;
        const c     = Cup;
        const pt    = Pint;
        const qt    = Quart;
        const gal   = Gallon;
    }

    /// Symbol module for [`Pressure`].
    pub mod pressure for type Pressure {
        const μPa = MicroPascal;
        const uPa = MicroPascal;
        const mPa = MilliPascal;
        const  Pa = Pascal;
        const kPa = KiloPascal;
        const MPa = MegaPascal;
        const GPa = GigaPascal;
        const TPa = TeraPascal;

        const  psi = Psi;
        const kpsi = KiloPsi;
        const Mpsi = MegaPsi;
    }

    /// Symbol module for [`Energy`].
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

        /// Unit alias for [`Watt`s](Power::Watt) multiplied by
        ///     [`Hour`s](Time::Hour).
        const  Wh: (Power * Time) = (W * h);

        /// Unit alias for [`KiloWatt`s](Power::KiloWatt) multiplied by
        ///     [`Hour`s](Time::Hour).
        const kWh: (Power * Time) = (kW * h);
    }

    /// Symbol module for [`Power`].
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

    /// Symbol module for [`Charge`].
    pub mod charge for type Charge as q, Q {
        const μC = MicroCoulomb;
        const uC = MicroCoulomb;
        const mC = MilliCoulomb;
        const  C = Coulomb;
        const kC = KiloCoulomb;
        const MC = MegaCoulomb;
        const GC = GigaCoulomb;
        const TC = TeraCoulomb;

        /// Unit alias for [`Amps`](Current::Amp) multiplied by
        ///     [`Hours`](Time::Hour).
        const  Ah: (Current * Time) = (A * h);

        /// Unit alias for [`MilliAmps`](Current::MilliAmp) multiplied by
        ///     [`Hours`](Time::Hour).
        const mAh: (Current * Time) = (mA * h);
    }

    /// Symbol module for [`Current`].
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

    /// Symbol module for [`Voltage`].
    pub mod voltage for type Voltage as U, V {
        const μV = MicroVolt;
        const uV = MicroVolt;
        const mV = MilliVolt;
        const  V = Volt;
        const kV = KiloVolt;
        const MV = MegaVolt;
        const GV = GigaVolt;
        const TV = TeraVolt;
    }

    /// Symbol module for [`Resistance`].
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
