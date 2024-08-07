use num_traits::Inv;
use dana::{assert_qty_approx, prelude::*, symbols::basic::*};


#[test]
pub fn test_macros() {
    //  Start with a basic length unit, and ensure `unit!` correctly
    //      produces one.
    let u: Length = m;
    assert_eq!(u.dimension(), u.inv().inv().dimension());

    //  Confirm that `utype!` produces types that agree.
    let _: utype!(L) = u;

    //  Check implicitly-positive exponents.
    let _: utype!(L^ 1) = u;
    let _: utype!(L^ 2) = u.squared();
    let _: utype!(L^ 3) = u.cubed();
    // let _: utype!(L^ 4) = u.pow(4.0);
    // let _: utype!(L^ 2.0) = u.pow(2.0);

    //  Check explicitly-positive exponents.
    let _: utype!(L^+1) = u;
    let _: utype!(L^+2) = u.squared();
    let _: utype!(L^+3) = u.cubed();
    // let _: utype!(L^+4) = u.pow(4.0);
    // let _: utype!(L^+2.0) = u.pow(2.0);

    //  Check explicitly-negative exponents.
    let _: utype!(L^-1) = u.inv();
    let _: utype!(L^-2) = u.squared().inv();
    let _: utype!(L^-3) = u.cubed().inv();
    // let _: utype!(L^-4) = u.pow(4.0).inv();
    // let _: utype!(L^-2.0) = u.pow(2.0).inv();


    //  Use that unit for a quantity, and ensure the `qty!` macro correctly
    //      produces one.
    let q: Quantity<Length> = qty!(2.0 u);
    assert_eq!(q, q.inv().inv());

    //  Confirm that `qtype!` produces types that agree.
    let _: qtype!(L) = q;

    //  Check implicitly-positive exponents.
    let _: qtype!(L^ 1) = q;
    let _: qtype!(L^ 2) = q.squared();
    let _: qtype!(L^ 3) = q.cubed();
    let _: qtype!(L^ 4) = q.squared().squared();
    // let _: qtype!(L^ 4) = q.pow(4.0);
    // let _: qtype!(L^ 2.0) = q.pow(2.0);

    //  Check explicitly-positive exponents.
    let _: qtype!(L^+1) = q;
    let _: qtype!(L^+2) = q.squared();
    let _: qtype!(L^+3) = q.cubed();
    let _: qtype!(L^+4) = q.squared().squared();
    // let _: qtype!(L^+4) = q.pow(4.0);
    // let _: qtype!(L^+2.0) = q.pow(2.0);

    //  Check explicitly-negative exponents.
    let _: qtype!(L^-1) = q.inv();
    let _: qtype!(L^-2) = q.squared().inv();
    let _: qtype!(L^-3) = q.cubed().inv();
    let _: qtype!(L^-4) = q.squared().squared().inv();
    // let _: qtype!(L^-4) = q.pow(4.0).inv();
    // let _: qtype!(L^-2.0) = q.pow(2.0).inv();

    //  Check powers and roots.
    assert_eq!(q.squared(), q.pow::<2>());
    assert_eq!(q.squared(), q.pow::<8>().root::<4>());
    assert_eq!(q.pow::<6>(), q.pow::<2>().pow::<3>());
}


#[test]
fn test_quantity_norm() {
    fn test<U: UnitStep>(q1: Quantity<U>) {
        let q2 = q1.normalize();

        // eprintln!("{q1:>9e} -> {q2:>8.3}");
        assert_eq!(q2, q1, "inconsistent precision loss");
        assert_qty_approx!(<= 1e-6, q1, q2);
    }

    test(qty![4.321_e+9  mm/s]);
    test(qty![4.321_e+8  mm/s]);
    test(qty![4.321_e+7  mm/s]);
    test(qty![4.321_e+6  mm/s]);
    // eprintln!();
    test(qty![4.321_e+5  mm/s]);
    test(qty![4.321_e+4  mm/s]);
    test(qty![4.321_e+3  mm/s]);
    // eprintln!();
    test(qty![4.321_e+2  mm/s]);
    test(qty![4.321_e+1  mm/s]);
    test(qty![4.321_e00  mm/s]);
    // eprintln!();
    test(qty![4.321_e-1  mm/s]);
    test(qty![4.321_e-2  mm/s]);
    test(qty![4.321_e-3  mm/s]);
    // eprintln!();
    test(qty![4.321_e-4  mm/s]);
    test(qty![4.321_e-5  mm/s]);
    test(qty![4.321_e-6  mm/s]);
    // eprintln!();
    test(qty![4.321_e-7  mm/s]);
    test(qty![4.321_e-8  mm/s]);
    test(qty![4.321_e-9  mm/s]);
    // eprintln!();
    test(qty![4.321_e-10 mm/s]);
    test(qty![4.321_e-11 mm/s]);
    test(qty![4.321_e-12 mm/s]);
}


#[test]
fn test_unit_step() {
    fn test<U: UnitStep + PartialOrd + core::fmt::Debug>() {
        let base = U::base();

        assert_eq!(
            1.0, base.scale(),
            "base unit `{base:?}` has the wrong scale",
        );

        assert!(base >= base.step_to_bottom());
        assert!(base <= base.step_to_top());
    }

    test::<Length>();
    test::<Mass>();
    test::<Time>();
    test::<Current>();
    test::<Temp>();
    test::<Amount>();
    test::<Intensity>();

    test::<Charge>();
    test::<Energy>();
    test::<Force>();
    test::<Frequency>();
    test::<One>();
    test::<Power>();
    test::<Pressure>();
    test::<Resistance>();
    test::<Voltage>();
}


#[test]
fn test_scale() {
    let dist = Length::MilliMeter.quantity(50.0);

    let as_mm = dist.with_unit(Length::MilliMeter).value;
    let as_cm = dist.with_unit(Length::CentiMeter).value;

    assert_eq!(as_mm, as_cm * 10.0);
}
