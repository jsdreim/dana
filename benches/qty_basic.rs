use criterion::{Criterion, criterion_group, criterion_main};
use dana::{constants::*, qty, Quantity, symbols::*, units::*};


fn mass_energy_float(mass_kg: f64) -> f64 {
    const C: f64 = 299_792_458.0;
    const C2: f64 = C * C;
    mass_kg * C2
}


fn mass_energy_qty_macro(mass_kg: f64) -> f64 {
    qty![*[mass_kg kg] * CONST_C2 in J]
}


fn mass_energy_quantity_methods(mass_kg: f64) -> f64 {
    let mass: Quantity<Mass> = Mass::KiloGram.quantity(mass_kg);
    let prod: Quantity<_> = mass * CONST_C2;
    let energy: Quantity<Energy> = prod.convert_to(Energy::Joule);
    energy.value
}


fn mass_energy(c: &mut Criterion) {
    let mut group = c.benchmark_group("Basic");

    group.bench_function(
        "native",
        |b| b.iter(|| mass_energy_float(criterion::black_box(20.0))),
    );

    group.bench_function(
        "qty macro",
        |b| b.iter(|| mass_energy_qty_macro(criterion::black_box(20.0))),
    );

    group.bench_function(
        "qty methods",
        |b| b.iter(|| mass_energy_quantity_methods(criterion::black_box(20.0))),
    );
}


criterion_group!(benches, mass_energy);
criterion_main!(benches);
