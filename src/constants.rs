//! Constant values for use in calculations.

use crate::{symbols::*, units::*};


/// Speed of light travelling through a perfect vacuum.
///
/// Unit: [m]/[s]
///
/// [m]: Length::Meter
/// [s]: Time::Second
pub const CONST_C: qtype!(Speed) = qty!(299_792_458.0 m/s);


/// Speed of light, squared; Relationship between mass and energy.
///
/// Unit: ([m]/[s])²
///
/// [m]: Length::Meter
/// [s]: Time::Second
pub const CONST_C2: qtype!(Speed^2) = qty!({CONST_C.value * CONST_C.value} (m/s)^2);


/// Elementary charge; Electrical charge of a single proton.
///
/// Unit: [C]
///
/// [C]: Charge::Coulomb
pub const CONST_E: qtype!(Charge) = qty![1.602_176_634_e-19 C];


/// Gravitational constant.
///
/// Unit: ([m]³/[s]²)/[kg]
///
/// [m]: Length::Meter
/// [s]: Time::Second
/// [kg]: Mass::KiloGram
pub const CONST_G: qtype!(GravParam / Mass) = qty!(6.6743e-11 m^3/s^2/kg);


/// One "G"; The average acceleration due to gravity at the surface of Earth.
///
/// Unit: [m]/[s]/[s]
///
/// [m]: Length::Meter
/// [s]: Time::Second
pub const GFORCE: qtype!(Accel) = qty!(9.80665 m/s/s);


/// Planck constant; Used to find the energy of a photon.
///
/// Unit: [eV]/[Hz]
///
/// [eV]: Energy::ElectronVolt
/// [Hz]: Frequency::Hertz
pub const CONST_H: qtype!(Energy / Frequency) = qty![4.135_667_696_e-15 eV/Hz];


/// Boltzmann constant; Relationship between thermal energy and temperature.
///
/// Unit: [eV]/[K]
///
/// [eV]: Energy::ElectronVolt
/// [K]: Temp::Kelvin
pub const CONST_K: qtype!(Energy / Temp) = qty![8.617_333_262_e-5 eV/K];


/// Gas constant; Relationship between energy, temperature, and substance amount.
///
/// Unit: [J]/[K]/[mol]
///
/// [J]: Energy::Joule
/// [K]: Temp::Kelvin
/// [mol]: Amount::Mole
pub const CONST_R: qtype!(E/K/N) = qty![8.314_462_618_153_24 J/K/mol];


#[test]
fn test_constants() {
    assert_eq!(format!("{CONST_C:.3e}"), "2.998e8 m/s");
    assert_eq!(format!("{CONST_G:.3e}"), "6.674e-11 (m^3/s^2)/kg");
    assert_eq!(format!("{GFORCE:.2}"), "9.81 (m/s)/s");

    assert_eq!(CONST_C.squared(), CONST_C2);
}
