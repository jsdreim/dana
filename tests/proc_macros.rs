#[test]
fn qty_invalid() {
    let test = trybuild::TestCases::new();

    test.compile_fail("tests/err_qty/*.rs");
}


#[test]
fn qty_valid() {
    use dim_macros::qty;
    use dimensional::{Quantity, units::{*, symbols::*}};

    //  Test basic definitions against fully-explicit types.
    let _qty: Quantity<L> = qty![1.0 m];
    let _qty: Quantity<L> = qty![1.0 m,];

    let _qty: Quantity<UnitSquared<L>> = qty![1.0 m^2];
    let _qty: Quantity<UnitMul<L, PerUnit<L>>> = qty![1.0 m * 1/m];
    let _qty: Quantity<UnitMul<L, PerUnit<L>>> = qty![1.0 m * 1/(m)];

    //  Test sum definitions.
    assert_eq!(qty![250.0 cm], qty![2.0 m,  50.0 cm]);
    assert_eq!(qty![150.0 cm], qty![2.0 m, -50.0 cm]);
    // assert_eq!(qty![250.0 cm], qty![2.0 m, +50.0 cm]); // Forbidden by Rust.

    //  Test basic operations and signs.
    assert_eq!(qty![250.0 cm], qty![2.0 m +  50.0 cm]);
    assert_eq!(qty![150.0 cm], qty![2.0 m + -50.0 cm]);
    assert_eq!(qty![150.0 cm], qty![2.0 m -  50.0 cm]);
    assert_eq!(qty![250.0 cm], qty![2.0 m - -50.0 cm]);

    //  Test chained definition, multiplication, and simplification.
    assert_eq!(qty![6.0 m^2], qty![2.0 m * 3.0 m -> L^2]);

    //  Test chained definition, addition, conversion, and deref with all
    //      combinations of recursion brackets.
    //  NOTE: Deref on multiple literals should probably not be encouraged, but
    //      to explicitly *forbid* it would likely cause more problems than the
    //      simple *ability* to write confusing invocations. Just use recursion
    //      brackets to make it clear.
    assert_eq!(qty![*   2.0 m  +  10.0 cm   in mm ], 2.1e3);
    assert_eq!(qty![*  [2.0 m] + [10.0 cm]  in mm ], 2.1e3);
    assert_eq!(qty![* [ 2.0 m  +  10.0 cm ] in mm ], 2.1e3);
    assert_eq!(qty![* [[2.0 m] + [10.0 cm]] in mm ], 2.1e3);
    assert_eq!(qty![*[  2.0 m  +  10.0 cm   in mm]], 2.1e3);
    assert_eq!(qty![*[ [2.0 m] + [10.0 cm]  in mm]], 2.1e3);
    assert_eq!(qty![*[[ 2.0 m  +  10.0 cm ] in mm]], 2.1e3);
    assert_eq!(qty![*[[[2.0 m] + [10.0 cm]] in mm]], 2.1e3);
}


#[test]
fn qty_ops() {
    use dim_macros::qty;
    use dimensional::{constants::*, Quantity, units::{*, symbols::*}};

    let v: Quantity<Voltage> = qty![3.3 V];
    let r: Quantity<Resistance> = qty![150.0 Ω];
    let i: Quantity<Current> = qty![v/r as _];

    assert_eq!(22.0, qty![*     i              in mA]);
    assert_eq!(22.0, qty![*     v  /        r  in mA]);
    assert_eq!(22.0, qty![*     v  / [150.0 Ω] in mA]);
    assert_eq!(22.0, qty![*[3.3 V] /        r  in mA]);
    assert_eq!(22.0, qty![*[3.3 V] / [150.0 Ω] in mA]);

    assert_eq!(
        qty![*[[9.80665 N] / GFORCE] as M * {CONST_C2} in kJ].floor(),
        89_875_517_873_681.0,
    );

    //  Test heavily mixed chains of operations.
    let accel: Quantity<Accel> = qty![30.0 cm/s/s];
    let added: f64 = 25.0;

    let _qty: Quantity<Force> = qty![
        1.0 kg, 50.0 g,
        * {CONST_C2} as (::dimensional::units::Energy)
        + {added} TJ
        / {CONST_C2} in (::dimensional::units::Mass::MetricTon)
        * {accel} as Force
        + 3.0 N
        + 1.0 Force::Newton
    ];
}


#[test]
fn qty_ops_adv() {
    use dim_macros::qty;
    use dimensional::{Quantity, units::{*, symbols::*}};

    let n = 5.0;
    let w_4: Quantity<Power> = qty![4.0 W];
    let kg_3: Quantity<Mass> = qty![3.0 kg];
    let m_s_12: Quantity<Speed> = qty![12.0 m/s];

    //  Test reuse of units.
    assert_eq!(qty![3.0 mW], qty![3e-3 w_4.unit]);
    assert_eq!(qty![3.0 kW], qty![3e+3 w_4.unit]);

    //  Test reuse of scalars.
    assert_eq!(qty![5e+3 W], qty![n kW]);
    assert_eq!(qty![5e+3 W], qty![{n} kW]);
    assert_eq!(qty![3e+3 W], qty![{kg_3.value} kW]);
    assert_eq!(qty![12e-3 W], qty![{m_s_12.value} mW]);

    //  Test entirely reused definitions, in combination with operations.
    assert_eq!(qty![36.0 kg*(m/s)], qty![
        {kg_3.value} kg_3.unit
        * {m_s_12.value} m_s_12.unit
        as Momentum
    ]);
    assert_eq!(qty![36.0 kg*(m/s)], qty![
        {m_s_12.value} kg_3.unit
        * {kg_3.value} m_s_12.unit
        as Momentum
    ]);

    //  Test...this.
    //  TODO: Allowing field access at all may make it too easy to accidentally
    //      break unit safety like this. Should it be reverted?
    assert_eq!(qty![16.0 W], qty![w_4 * w_4.value]);
}


#[test]
fn qtype_valid() {
    use dim_macros::qtype;
    use dimensional::units::{*, symbols::*, traits::*};

    let u = Length::Meter / Time::Second.squared();

    //  Unspecified inner type.
    let _: qtype!(L^1/T^2) = u.quantity(1.0);
    let _: qtype!(L^1/T^2) = u.quantity(1f64);
    let _: qtype!(L^1/T^2) = u.quantity(1 as _);

    //  Inferred inner type.
    let _: qtype!(_; L^1/T^2) = u.quantity(1.0);
    let _: qtype!(_; L^1/T^2) = u.quantity(1f64);
    let _: qtype!(_; L^1/T^2) = u.quantity(1f32);
    let _: qtype!(_; L^1/T^2) = u.quantity(1i32);
    let _: qtype!(_; L^1/T^2) = u.quantity(1u128);
    let _: qtype!(_; L^1/T^2) = u.quantity(1usize);

    //  Explicit inner type (f32).
    let _: qtype!(f32; L^1/T^2) = u.quantity(1.0);
    let _: qtype!(f32; L^1/T^2) = u.quantity(1f32);
    let _: qtype!(f32; L^1/T^2) = u.quantity(1 as _);

    //  Explicit inner type (f64).
    let _: qtype!(f64; L^1/T^2) = u.quantity(1.0);
    let _: qtype!(f64; L^1/T^2) = u.quantity(1f64);
    let _: qtype!(f64; L^1/T^2) = u.quantity(1 as _);

    //  Explicit inner type (usize).
    let _: qtype!(usize; L^1/T^2) = u.quantity(1);
    let _: qtype!(usize; L^1/T^2) = u.quantity(1usize);
    let _: qtype!(usize; L^1/T^2) = u.quantity(1 as _);
}
