#![feature(portable_simd)]

use std::{array::from_fn, ops::{Add, Mul}, simd::*};
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use num_traits::AsPrimitive;
use rand::Rng;
use dimensional::{Scalar, simd::*, symbols::*, units::*};


fn simd_float<V, const N: usize>(
    position: Simd<V, { N }>,
    velocity: Simd<V, { N }>,
    time: Simd<V, { N }>,
) -> Simd<V, { N }> where
    LaneCount<{ N }>: SupportedLaneCount,
    V: SimdElement,
    Simd<V, { N }>: Add<Output=Simd<V, { N }>> + Mul<Output=Simd<V, { N }>>,
{
    position + velocity * time
}


fn simd_quantity<V, const N: usize>(
    position: QtySimd<Length, V, { N }>,
    velocity: QtySimd<Speed, V, { N }>,
    time: QtySimd<Time, V, { N }>,
) -> QtySimd<Length, V, { N }> where
    LaneCount<{ N }>: SupportedLaneCount,
    V: Scalar + SimdElement,
    QtySimd<Speed, V, { N }>: Mul<QtySimd<Time, V, { N }>>,
    QtySimd<Length, V, { N }>: Add<
        <QtySimd<Speed, V, { N }> as Mul<QtySimd<Time, V, { N }>>>::Output,
        Output = QtySimd<Length, V, { N }>,
    >
{
    position + velocity * time
}


fn bench_simd_group<V, const N: usize>(c: &mut Criterion, name: &str) where
    LaneCount<{ N }>: SupportedLaneCount,
    V: Scalar + SimdElement /*+ Add<Output=V> + Mul<Output=V>*/,
    f64: AsPrimitive<V>,
    Simd<V, { N }>: Add<Output=Simd<V, { N }>> + Mul<Output=Simd<V, { N }>>,
    QtySimd<Speed, V, { N }>: Mul<QtySimd<Time, V, { N }>>,
    QtySimd<Length, V, { N }>: Add<
        <QtySimd<Speed, V, { N }> as Mul<QtySimd<Time, V, { N }>>>::Output,
        Output = QtySimd<Length, V, { N }>,
    >
{
    let mut group = c.benchmark_group(name);

    let rng = &mut rand::thread_rng();
    let pos: [V; N] = from_fn(|_| rng.gen_range( 1.0..=20.0).as_());
    let vel: [V; N] = from_fn(|_| rng.gen_range(-2.0..= 2.0).as_());
    let time: V     =             rng.gen_range( 1.0..= 5.0).as_();

    let pos_f: Simd<V, { N }> = Simd::from(pos);
    let vel_f: Simd<V, { N }> = Simd::from(vel);
    let time_f: Simd<V, { N }> = Simd::from([time; N]);

    let pos_q: QtySimd<Length, V, { N }> = m.quantity_simd(pos);
    let vel_q: QtySimd<Speed, V, { N }> = (m/s).quantity_simd(vel);
    let time_q: QtySimd<Time, V, { N }> = s.quantity(time).to_simd();

    group.bench_function(
        "float",
        |b| b.iter(|| simd_float(
            black_box(pos_f),
            black_box(vel_f),
            black_box(time_f),
        )),
    );

    group.bench_function(
        "quantity",
        |b| b.iter(|| simd_quantity(
            black_box(pos_q),
            black_box(vel_q),
            black_box(time_q),
        )),
    );
}


fn bench_simd(c: &mut Criterion) {
    macro_rules! bench {($c:ident, $ty:ty, $($n:literal),+ $(,)?) => {$(
        bench_simd_group::<$ty, $n>($c, concat!("SIMD/", stringify!($ty), "x", $n));
    )+}}

    // bench!(c, f32, 1, 2, 4, 8, 16, 32);
    // bench!(c, f64, 1, 2, 4, 8, 16, 32);

    bench!(c, f32, 2, 4);
    bench!(c, f64, 1, 2);
}


criterion_group!(benches, bench_simd);
criterion_main!(benches);
