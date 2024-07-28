//! Module for shortcut labels to dimensions and units, based on SI symbols.
//!
//! These serve to make complex definitions clearer to read and easier to write.
//!     Examples include `m` for [`Length::Meter`] and `E` for [`Energy`]. Also
//!     included are symbols for standard compound units, such as [`kWh`] for
//!     Kilowatt-hours (energy as the product of power and time).
//!
//! Symbols for dimensions are implemented as type aliases, and those for units
//!     are implemented as constants. A convenient side effect of this is that
//!     IDEs with robust analysis may render them in different colors, for even
//!     more clarity.

#![allow(non_camel_case_types, non_upper_case_globals)]

#[macro_use]
mod macros;

use crate::units::*;


pub mod dim {
    //! Symbols for specific [`Dimension`] types.

    use crate::dimension::*;

    //region Fundamental dimensions.
    pub type L = Length;
    pub type M = Mass;
    pub type T = Time;
    pub type I = Current;
    pub type K = Temp;
    pub type Θ = Temp;
    pub type N = Amount;
    pub type J = Intensity;
    //endregion

    //region Derived dimensions.
    //  TODO: How many of these should be provided (if any at all)?
    /*pub type f = Frequency;
    pub type v = Velocity;
    pub type a = Accel;
    pub type F = Force;
    // pub type p = Pressure;
    // pub type A = Area;
    // pub type V = Volume;
    pub type D = Density;
    pub type ρ = Density;

    pub type Q = Charge;
    pub type τ = Torque;
    pub type E = Energy;
    pub type P = Power;
    pub type V = Voltage;
    // pub type U = Voltage;
    pub type R = Resistance;
    pub type C = Capacitance;*/
    //endregion
}


pub mod common {
    pub use super::{
        length_si::{L, m, km},
        mass_si::{M, kg},
        time::{T, s, h},
    };
}


define_groups! {
    /// Unit symbols for basic dimensions: Length, mass, time, and speed.
    pub mod basic(
        [L],       [M],     [T],  /*[v],*/
        length_si, mass_si, time, speed,
    );

    /// Unit symbols for units often used in chemistry.
    pub mod chemical(
        [M],     [T],  [E],    [K,Θ], [N],
        mass_si, time, energy, temp,  amount,
    );

    /// Unit symbols for units often used in physics.
    pub mod physical(
        (basic),
        [E],    [f],       [F],                [K,Θ], [J],
        energy, frequency, force_si, pressure, temp,  intensity,
    );

    /// Unit symbols related to electricity.
    pub mod electrical(
        [P],   [Q],    [I],     [U,V],   [R],
        power, charge, current, voltage, resistance,
    );

    /// Unit symbols for the ISQ base quantities.
    pub mod isq(
        [L],       [M],     [T],  [I],     [K,Θ], [N],    [J],
        length_si, mass_si, time, current, temp,  amount, intensity,
    );

    /// Unit symbols for the Imperial system.
    pub mod imperial(
        [L],             [M],           [T],  [F],
        length_imperial, mass_imperial, time, force_imperial,
    );
}


//  TODO: Greek letters acceptable?
define_symbols! {
    pub mod speed for type Speed /*as v*/ {
        const kph = (km/h);
        const mph = (mi/h);
        // const fps = (ft/s);
    }

    pub mod length_si for type Length as L {
        const nm = NanoMeter;
        const μm = MicroMeter;
        const um = MicroMeter;
        const mm = MilliMeter;
        const cm = CentiMeter;
        const  m = Meter;
        const km = KiloMeter;
    }

    pub mod length_imperial for type Length in mod length_si {
        // const in = Inch; // ...Hmm.
        const inch = Inch;
        const ft = Foot;
        const yd = Yard;
        const mi = Mile;
    }

    pub mod length_space for type Length in mod length_si {
        use length_si;

        const AU = AstroUnit;
        const ls = LightSec;
        const ly = LightYear;

        const  pc = Parsec;
        const kpc = KiloParsec;
        const Mpc = MegaParsec;
        const Gpc = GigaParsec;
    }

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

    pub mod mass_imperial for type Mass in mod mass_si {
        const oz = Ounce;
        const lb = Pound;
    }

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

    pub mod force_imperial for type Force in mod force_si {
        const ozf = Ounce;
        const lbf = Pound;
    }

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

    pub mod volume_us for type Volume in mod volume_si {
        const fl_dr = Dram;
        const fl_oz = FlOunce;
        const c     = Cup;
        const pt    = Pint;
        const qt    = Quart;
        const gal   = Gallon;
    }

    pub mod pressure for type Pressure {
        const μPa = MicroPascal;
        const uPa = MicroPascal;
        const mPa = MilliPascal;
        const  Pa = Pascal;
        const kPa = KiloPascal;
        const MPa = MegaPascal;
        const GPa = GigaPascal;
        const TPa = TeraPascal;

        const psi: (Force / Area) = (lbf / inch^2);
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
