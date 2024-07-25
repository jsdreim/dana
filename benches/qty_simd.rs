#![feature(portable_simd)]

use std::{array::from_fn, ops::{Add, Mul}, simd::*};
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use rand::Rng;
use dimensional::{Scalar, simd::*, units::{*, symbols::*}};


type Num = f32;
const WIDTH: usize = 8;


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


fn simd_quantity</*Num,*/ const N: usize>(
    position: QtySimd<Length, Num, { N }>,
    velocity: QtySimd<Speed, Num, { N }>,
    time: QtySimd<Time, Num, { N }>,
) -> QtySimd<Length, Num, { N }> where
    LaneCount<{ N }>: SupportedLaneCount,
    Num: Scalar + SimdElement,
{
    position + velocity * time
}


fn bench_simd(c: &mut Criterion) {
    let mut group = c.benchmark_group("SIMD");

    let rng = &mut rand::thread_rng();
    let pos: [Num; WIDTH] = from_fn(|_| rng.gen_range( 1.0..=20.0));
    let vel: [Num; WIDTH] = from_fn(|_| rng.gen_range(-2.0..= 2.0));
    let time: Num         =             rng.gen_range( 1.0..= 5.0);

    let pos_f: Simd<Num, WIDTH> = Simd::from(pos);
    let vel_f: Simd<Num, WIDTH> = Simd::from(vel);
    let time_f: Simd<Num, WIDTH> = Simd::from([time; WIDTH]);

    let pos_q: QtySimd<Length, Num, WIDTH> = m.quantity_simd(pos);
    let vel_q: QtySimd<Speed, Num, WIDTH> = (m/s).quantity_simd(vel);
    let time_q: QtySimd<Time, Num, WIDTH> = s.quantity(time).to_simd();

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


criterion_group!(benches, bench_simd);
criterion_main!(benches);
