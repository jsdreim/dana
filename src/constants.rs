use crate::units::*;

use Distance::Meter;
use Time::Second;
use Mass::Kilogram;


/// Speed of light travelling through a perfect vacuum.
///
/// Unit: [m]/[s]
///
/// [m]: Meter
/// [s]: Second
pub const C: qtype!(Speed)
= quantity!(299_792_458.0   (Meter / Second));


/// Gravitational constant.
///
/// Unit: ([m]³/[s]²)/[kg]
///
/// [m]: Meter
/// [s]: Second
/// [kg]: Kilogram
pub const G: qtype!(GravParam / Mass)
= quantity!(6.6743e-11      (Meter^3 / Second^2) / Kilogram);


/// One "G"; The average acceleration due to gravity at the surface of Earth.
///
/// Unit: [m]/[s]/[s]
///
/// [m]: Meter
/// [s]: Second
pub const GFORCE: qtype!(Acceleration)
= quantity!(9.80665         (Meter / Second) / Second);


#[test]
fn test_constants() {
    assert_eq!(format!("{C:.3e}"), "2.998e8 m/s");
    assert_eq!(format!("{G:.3e}"), "6.674e-11 (m^3/s^2)/kg");
    assert_eq!(format!("{GFORCE:.2}"), "9.81 (m/s)/s");
}
