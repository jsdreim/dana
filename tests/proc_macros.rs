#[test]
fn qty_invalid() {
    let test = trybuild::TestCases::new();

    test.compile_fail("tests/err_qty/*.rs");
}


#[test]
fn qty_valid() {
    use dimensional::{qty, Quantity, units::{*, symbols::*}};

    //  Test basic definitions against fully-explicit types.
    let _qty: Quantity<L> = qty![1.0 m];
    let _qty: Quantity<L> = qty![1.0 m,];

    let _qty: Quantity<UnitSquared<L>> = qty![1.0 m^2];
    let _qty: Quantity<UnitMul<L, PerUnit<L>>> = qty![1.0 m * 1/m];
    let _qty: Quantity<UnitMul<L, PerUnit<L>>> = qty![1.0 m * 1/(m)];

    //  Test additive definitions.
    assert_eq!(qty![250.0 cm], qty![2.0 m + 50.0 cm]);
    assert_eq!(qty![250.0 cm], qty![2.0 m, 50.0 cm]);
    assert_eq!(qty![250.0 cm], qty![2.0 m 50.0 cm]);

    //  Test subtractive definitions.
    assert_eq!(qty![150.0 cm], qty![2.0 m + -50.0 cm]);
    assert_eq!(qty![150.0 cm], qty![2.0 m, -50.0 cm]);
    assert_eq!(qty![150.0 cm], qty![2.0 m - 50.0 cm]);

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

    //  Test heavily mixed chains of operations.
    let accel: Quantity<Accel> = qty![30.0 cm/s/s];
    let added: f64 = 25.0;

    let _qty: Quantity<Force> = qty![
        1.0 kg, 50.0 g,
        as ::dimensional::units::Energy + {added} TJ
        in ::dimensional::units::Mass::MetricTon
        * {accel}
        as Force + 3.0 N + 1.0 Force::Newton
    ];
}


#[test]
fn qty_ops() {
    use dimensional::{constants, qty, Quantity, units::{*, symbols::*}};

    let v: Quantity<Voltage> = qty![3.3 V];
    let r: Quantity<Resistance> = qty![150.0 Ω];
    let i: Quantity<Current> = qty![v/r as _];

    assert_eq!(22.0, qty![*     i              in mA]);
    assert_eq!(22.0, qty![*     v  /        r  in mA]);
    assert_eq!(22.0, qty![*     v  / [150.0 Ω] in mA]);
    assert_eq!(22.0, qty![*[3.3 V] /        r  in mA]);
    assert_eq!(22.0, qty![*[3.3 V] / [150.0 Ω] in mA]);

    assert_eq!(
        qty![*[[9.80665 N] / (constants::GFORCE)] as M in kJ].floor(),
        89_875_517_873_681.0,
    );
}
