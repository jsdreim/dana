use num_traits::Inv;
use dana::dimension::*;


#[test]
fn test_dimensions() {
    assert_eq!(format!("{}", Length::new()), "L");
    assert_eq!(format!("{}", Velocity::new()), "L*T^-1");
    assert_eq!(format!("{}", Accel::new()), "L*T^-2");

    let _: Accel = Velocity::new() / Time::new();
    let _: Accel = Velocity::new() * Time::new().inv();
    let _: Length = Velocity::new() * Time::new();
    let _: Torque = Length::new() * Force::new();
}
