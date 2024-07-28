use criterion::{black_box, Criterion, criterion_group, criterion_main};
use dana::{qty, Quantity, symbols::electrical::*, units::*};


fn capacitor_float(amp_hours: f64, volts: f64, ohms: f64) -> (f64, f64) {
    let q = amp_hours * 3_600.0;
    let v = volts;
    let r = ohms;

    let i = v / r;
    let p = i * v;
    let t = q / i;
    let e = p * t;

    (t, e)
}


fn capacitor_float_opt(amp_hours: f64, volts: f64, ohms: f64) -> (f64, f64) {
    let q = amp_hours * 3_600.0;
    let v = volts;
    let r = ohms;

    //  T = Q / I
    //  T = Q / (V/R)
    //  T = Q * R / V
    let t = q * r / v;

    //  E = P * T
    //  E = (I*V) * (Q/I)
    //  E = ((V/R) * V) * (Q / (V/R))
    //  E = Q * V * (V/R) / (V/R)
    //  E = Q * V
    let e = q * v;

    // assert_eq!(q,  27_000.0);
    // assert_eq!(t, 112_500.0);
    // assert_eq!(e, 324_000.0);

    (t, e)
}


fn capacitor_quantity(amp_hours: f64, volts: f64, ohms: f64) -> (f64, f64) {
    let q: Quantity<Charge> = qty![amp_hours Ah as Q];
    let v: Quantity<Voltage> = qty![volts V];
    let r: Quantity<Resistance> = qty![ohms Ω];

    let i: Quantity<Current> = qty![(v / r) as _];
    let p: Quantity<Power> = qty![(i * v) as _];
    let t: Quantity<Time> = qty![(q / i) as _];
    let e: Quantity<Energy> = qty![(p * t) as _];

    (t.value, e.value)
}


fn capacitor_quantity_opt(amp_hours: f64, volts: f64, ohms: f64) -> (f64, f64) {
    let q: Quantity<Charge> = qty![amp_hours Ah as Q];
    let v: Quantity<Voltage> = qty![volts V];
    let r: Quantity<Resistance> = qty![ohms Ω];

    let t: Quantity<Time> = qty![q * r / v as _];
    let e: Quantity<Energy> = qty![q * v as _];

    (t.value, e.value)
}


fn capacitor(c: &mut Criterion) {
    let mut group = c.benchmark_group("Capacitor");

    group.bench_function(
        "native",
        |b| b.iter(|| capacitor_float(
            black_box(7.5),
            black_box(12.0),
            black_box(50.0),
        )),
    );

    group.bench_function(
        "qty",
        |b| b.iter(|| capacitor_quantity(
            black_box(7.5),
            black_box(12.0),
            black_box(50.0),
        )),
    );

    group.bench_function(
        "native (opt)",
        |b| b.iter(|| capacitor_float_opt(
            black_box(7.5),
            black_box(12.0),
            black_box(50.0),
        )),
    );

    group.bench_function(
        "qty (opt)",
        |b| b.iter(|| capacitor_quantity_opt(
            black_box(7.5),
            black_box(12.0),
            black_box(50.0),
        )),
    );
}


criterion_group!(benches, capacitor);
criterion_main!(benches);
