use crate::units::{*, symbols::*};


/// Speed of light travelling through a perfect vacuum.
///
/// Unit: [m]/[s]
///
/// [m]: Length::Meter
/// [s]: Time::Second
pub const C: qtype!(Speed) = qty!(299_792_458.0 m/s);


/// Gravitational constant.
///
/// Unit: ([m]³/[s]²)/[kg]
///
/// [m]: Length::Meter
/// [s]: Time::Second
/// [kg]: Mass::KiloGram
pub const G: qtype!(GravParam / Mass) = qty!(6.6743e-11 m^3/s^2/kg);


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
pub const H: qtype!(Energy / Frequency) = qty![4.135_667_696_e-15 eV/Hz];


#[test]
fn test_constants() {
    assert_eq!(format!("{C:.3e}"), "2.998e8 m/s");
    assert_eq!(format!("{G:.3e}"), "6.674e-11 (m^3/s^2)/kg");
    assert_eq!(format!("{GFORCE:.2}"), "9.81 (m/s)/s");
}
