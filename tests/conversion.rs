use num_traits::Inv;
use dana::{prelude::*, symbols::{electrical::*, physics::*}};


#[test]
fn test_anonymous() {
    use dana::quantity::QuantityAnon;

    let l: QuantityAnon<_> = qty![72.0 km as ?];
    let t: QuantityAnon<_> = qty![4.0 h as ?];

    assert_eq!(qty![ 5.0  m/s  ], l / t);
    assert_eq!(qty![ 5.0  m/s  ], l * t.inv());
    assert_eq!(qty![36.0 km    ], l / t * qty![120.0 min]);
    assert_eq!(qty![ 0.5  m/s^2], l / t / qty![ 10.0 s  ]);

    let width: QuantityAnon<_>  = qty![40.0 cm as ?];
    let height: QuantityAnon<_> = qty![ 1.5  m as ?];

    let area_base = width.squared();
    let volume_post = height * area_base;

    assert!(qty![0.16 m^2].almost_eq(area_base, 1e-10));
    assert!(qty![0.24 m^3].almost_eq(volume_post, 1e-10));

    // assert_ne!(qty![2.0 m], qty![2.0 m^0 as ?]);
    assert_eq!(qty![2.0 m], qty![2.0 m^1 as ?]);
    assert_eq!(qty![2.0 m], qty![4.0 m^2 as ?].sqrt());
    assert_eq!(qty![2.0 m], qty![8.0 m^3 as ?].cbrt());
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
    //  3V3 across a 7g 150Ω resistor.
    let v: Quantity<Voltage> = qty![3.3 V];
    let r: Quantity<Resistance> = qty![150.0 Ω];
    let c: Quantity<HeatCapacity> = qty![[1.0 J/K/g] * [7.0 g] in J/K];

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

    //  Resistor should now be about 3.1K hotter.
    let delta: Quantity<Temp> = qty![(e / c) in K];
    assert_eq!(qty![*delta in mK].round(), 3111.0);
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
    let t: Quantity<Time> = qty![(q / i) as T];
    assert_eq!(t, qty![31.0 h, 15.0 min]);

    //  After that time, should have dissipated 324kJ (90Wh) in total.
    let e: Quantity<Energy> = qty![(p * t) as E];
    assert_eq!(e, qty![324.0 kJ]);
    assert_eq!(e, qty![90.0 Wh]);
}


#[test]
fn test_cancel() {
    //  Simplify: (L/T)*T -> L
    let v = qty![2.0 mm/ms];
    let t = qty![3.0 min];
    let d = v * t;
    assert_eq!(d, qty![360.0 m]);

    //  Simplify: L/L -> 1
    let n = qty![d / 5.0 m].cancel();
    assert_eq!(n, 72.0);
}


#[test]
fn test_powers() {
    //  Start with basic length.
    let x1: Quantity<Length> = qty![2.0 m];

    //  Multiply and then simplify to square.
    let x1mul:  qtype!(L * L)   = x1*x1.with_unit(Length::MilliMeter);
    let x2:     qtype!(L^2)     = x1mul.convert();

    //  Multiply and then simplify to cube.
    let x2mul:  qtype!(L^2 * L) = x2*x1.with_unit(Length::KiloMeter);
    let x3:     qtype!(L^3)     = x2mul.convert();

    //  Ensure the results match.
    assert_eq!(x2, x1.squared());
    assert_eq!(x3, x1.cubed());

    //  Ensure the results are actually correct.
    assert_eq!(x2, qty![4.0 m^2]);
    assert_eq!(x3, qty![8.0 m^3]);

    //  Climb back down.

    //  Divide and then simplify back down to square.
    let x3div:  qtype!(L^3 / L) = x3/x1.with_unit(Length::MilliMeter);
    let x2:     qtype!(L^2)     = x3div.convert();

    //  Divide and then simplify back down to square.
    let x2div:  qtype!(L^2 / L) = x2/x1.with_unit(Length::KiloMeter);
    let x1:     qtype!(L^1)     = x2div.convert();

    //  Ensure the results are still correct.
    assert_eq!(x2, qty![4.0 m^2]);
    assert_eq!(x1, qty![2.0 m^1]);
}


#[test]
fn test_volume() {
    use dana::symbols::{volume_si::*, volume_us::*};

    assert_eq!(qty![*[1.0 L] in m^3], 1e-3);

    let density_water: Quantity<Density> = qty![0.997_048 kg/L];
    assert_eq!(qty![*[1.0 gal] * density_water in mg].round(), 3_774_237.0);
}
