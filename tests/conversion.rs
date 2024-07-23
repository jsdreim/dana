use ::dimensional::{
    constants::*,
    equations,
    qty, qtype,
    Quantity,
    units::{*, symbols::*},
};


#[test]
fn test_ratios() {
    // let v = qty![5.0 m/s];

    let f = qty![1.0 kHz];
    let v = qty![CONST_C in m/s];

    let v_f = v / qty![f as 1/t];

    let fv = qty![v_f -> l];

    // let fv: qtype!((l/t) / (1/t)) = v / qty![f as 1/t];
    // let fv: qtype!((l/t) * t) = fv.simplify();
    // let fv: qtype!(l) = fv.simplify();

    assert_eq!(fv, equations::frequency_to_wavelength(f, v));

    // dbg!(qty![(f/v) in m]);
}


#[test]
fn test_f_ma() {
    let mass: Quantity<Mass> = qty![2.0 kg];
    let accel: Quantity<Accel> = qty![3.0 km/s/s];

    let force: Quantity<Force> = qty![(mass * accel) as _];
    assert_eq!(qty![*force in kN], 6.0);
}


#[test]
fn test_electrical() {
    //  3V3 across a 150Ω resistor.
    let v: Quantity<Voltage> = qty![3.3 V];
    let r: Quantity<Resistance> = qty![150.0 Ω];

    //  Should measure 22mA of current through the resistor.
    let i: Quantity<Current> = qty![(v / r) in A];
    assert_eq!(qty![*i in mA], 22.0);

    //  Resistor should be dissipating 72.6mW as heat.
    let p: Quantity<Power> = qty![(i * v) in W];
    assert_eq!(qty![*p in mW], 72.6);

    //  After 5 minutes, should have dissipated 21.78J in total.
    let t: Quantity<Time> = qty![300.0 s];
    let e: Quantity<Energy> = qty![(p * t) in J];
    assert_eq!(qty![*e in J], 21.78);
}


#[test]
fn test_electrical_charge() {
    //  7500mAh 12V battery (with no discharge curve) across a 50Ω resistor.
    let q: Quantity<Charge> = qty![7.5 Ah as Q];
    let v: Quantity<Voltage> = qty![12.0 V];
    let r: Quantity<Resistance> = qty![50.0 Ω];

    //  Should measure 240mA of current through the resistor.
    let i: Quantity<Current> = qty![(v / r) as I];
    assert_eq!(i, qty![240.0 mA]);

    //  Resistor should be dissipating 2.88W as heat.
    let p: Quantity<Power> = qty![(i * v) as P];
    assert_eq!(p, qty![2.88 W]);

    //  Battery should last for 31h15m.
    let t: Quantity<Time> = qty![q / [i in C/s] -> t];
    assert_eq!(t, qty![31.0 h, 15.0 min]);

    //  After that time, should have dissipated 324kJ (90Wh) in total.
    let e: Quantity<Energy> = qty![(p * t) as E];
    assert_eq!(e, qty![324.0 kJ]);
    assert_eq!(e, qty![90.0 Wh]);
}


#[test]
fn test_cancel() {
    let v: Quantity<Speed> = qty![2.0 mm/ms];
    let t: Quantity<Time> = qty![3.0 min];
    let d: Quantity<Length> = (v * t).convert();
    assert_eq!(d, qty!(360.0 m));

    let dt: qtype!(l * t) = qty![90.0 m*s];
    let t: Quantity<Time> = qty![0.5 min];
    let d: Quantity<Length> = (dt / t).convert();
    assert_eq!(d, qty!(3.0 m));
}


#[test]
fn test_compounds() {
    let l = qty![5.0 m];
    let t = qty![2.0/s];

    let q: qtype!(l * (1/t)) = l * t;
    let v: qtype!(l / t) = q.convert();

    assert_eq!(v, qty![10.0 m/s]);
}


#[test]
fn test_powers() {
    //  Start with basic length.
    let x1: Quantity<Length> = qty![2.0 m];

    //  Multiply and then simplify to square.
    let x1mul:  qtype!(l * l)   = x1*x1.with_unit(Length::MilliMeter);
    let x2:     qtype!(l^2)     = x1mul.convert();

    //  Multiply and then simplify to cube.
    let x2mul:  qtype!(l^2 * l) = x2*x1.with_unit(Length::KiloMeter);
    let x3:     qtype!(l^3)     = x2mul.convert();

    //  Ensure the results match.
    assert_eq!(x2, x1.squared());
    assert_eq!(x3, x1.cubed());

    //  Ensure the results are actually correct.
    assert_eq!(x2, qty![4.0 m^2]);
    assert_eq!(x3, qty![8.0 m^3]);

    //  Climb back down.

    //  Divide and then simplify back down to square.
    let x3div:  qtype!(l^3 / l) = x3/x1.with_unit(Length::MilliMeter);
    let x2:     qtype!(l^2)     = x3div.convert();

    //  Divide and then simplify back down to square.
    let x2div:  qtype!(l^2 / l) = x2/x1.with_unit(Length::KiloMeter);
    let x1:     qtype!(l^1)     = x2div.convert();

    //  Ensure the results are still correct.
    assert_eq!(x2, qty![4.0 m^2]);
    assert_eq!(x1, qty![2.0 m^1]);
}
