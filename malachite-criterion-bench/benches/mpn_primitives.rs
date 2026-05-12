// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// mpn-layer inner-loop benchmarks. These are the hot loops where GMP's hand-written
// assembly (MULX/ADCX/ADOX, dual carry chain) wins against pure-Rust implementations.
// Use these to measure the impact of optimizations to the limbs_* primitives.

use criterion::*;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_same_length_in_place_left,
};
use malachite_nz::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::shl::limbs_shl_to_out;
use malachite_nz::natural::arithmetic::shr::limbs_shr_to_out;
use malachite_nz::natural::arithmetic::sub::limbs_sub_same_length_to_out;
use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_limb_same_length_in_place_left;
use malachite_nz::platform::{DoubleLimb, Limb};

// Limb counts spanning basecase / Karatsuba / Toom / FFT regimes.
// At 64-bit limbs: 4 -> 256b, 32 -> 2Kb, 256 -> 16Kb, 2048 -> 128Kb, 16384 -> 1Mb.
const LIMB_COUNTS: &[usize] = &[4, 16, 32, 64, 128, 256, 512, 1024, 2048, 8192, 32768];

fn random_limbs(n: usize, tag: &'static str) -> Vec<Limb> {
    let mut it = random_primitive_ints::<Limb>(EXAMPLE_SEED.fork(tag));
    (0..n).map(|_| it.next().unwrap()).collect()
}

fn bench_addmul_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::addmul_1 (limbs_slice_add_mul_limb_same_length_in_place_left)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        let xs0 = random_limbs(n, "xs");
        let ys = random_limbs(n, "ys");
        let z: Limb = 0x9E37_79B9_7F4A_7C15u64 as Limb;
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || xs0.clone(),
                |mut xs| {
                    let c = limbs_slice_add_mul_limb_same_length_in_place_left(
                        black_box(&mut xs),
                        black_box(&ys),
                        black_box(z),
                    );
                    black_box((xs, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_submul_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::submul_1 (limbs_sub_mul_limb_same_length_in_place_left)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        // Make xs >= ys * z + carry so we don't underflow into wrap territory.
        let xs0: Vec<Limb> = random_limbs(n, "xs").into_iter().map(|x| x | Limb::MAX >> 1).collect();
        let ys = random_limbs(n, "ys");
        let z: Limb = 0x1234_5678_9ABC_DEFu64 as Limb;
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || xs0.clone(),
                |mut xs| {
                    let c = limbs_sub_mul_limb_same_length_in_place_left(
                        black_box(&mut xs),
                        black_box(&ys),
                        black_box(z),
                    );
                    black_box((xs, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_mul_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::mul_1 (limbs_mul_limb_to_out)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        let xs = random_limbs(n, "xs");
        let z: Limb = 0x9E37_79B9_7F4A_7C15u64 as Limb;
        let out0 = vec![0 as Limb; n];
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || out0.clone(),
                |mut out| {
                    let c = limbs_mul_limb_to_out::<DoubleLimb, Limb>(
                        black_box(&mut out),
                        black_box(&xs),
                        black_box(z),
                    );
                    black_box((out, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_mul_1_in_place(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::mul_1 in-place (limbs_slice_mul_limb_in_place)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        let xs0 = random_limbs(n, "xs");
        let z: Limb = 0x9E37_79B9_7F4A_7C15u64 as Limb;
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || xs0.clone(),
                |mut xs| {
                    let c = limbs_slice_mul_limb_in_place(black_box(&mut xs), black_box(z));
                    black_box((xs, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_add_n(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::add_n (limbs_add_same_length_to_out)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        let xs = random_limbs(n, "xs");
        let ys = random_limbs(n, "ys");
        let out0 = vec![0 as Limb; n];
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || out0.clone(),
                |mut out| {
                    let c = limbs_add_same_length_to_out(
                        black_box(&mut out),
                        black_box(&xs),
                        black_box(&ys),
                    );
                    black_box((out, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_add_n_in_place(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::add_n in-place (limbs_slice_add_same_length_in_place_left)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        let xs0 = random_limbs(n, "xs");
        let ys = random_limbs(n, "ys");
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || xs0.clone(),
                |mut xs| {
                    let c = limbs_slice_add_same_length_in_place_left(
                        black_box(&mut xs),
                        black_box(&ys),
                    );
                    black_box((xs, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_sub_n(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::sub_n (limbs_sub_same_length_to_out)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        // Ensure xs >= ys to avoid borrow at termination.
        let xs: Vec<Limb> = random_limbs(n, "xs").into_iter().map(|x| x | (Limb::MAX >> 1)).collect();
        let ys: Vec<Limb> = random_limbs(n, "ys").into_iter().map(|x| x & (Limb::MAX >> 1)).collect();
        let out0 = vec![0 as Limb; n];
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || out0.clone(),
                |mut out| {
                    let c = limbs_sub_same_length_to_out(
                        black_box(&mut out),
                        black_box(&xs),
                        black_box(&ys),
                    );
                    black_box((out, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_lshift(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::lshift (limbs_shl_to_out, bit-shift)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        let xs = random_limbs(n, "xs");
        let out0 = vec![0 as Limb; n];
        let bits: u64 = 17;
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || out0.clone(),
                |mut out| {
                    let c = limbs_shl_to_out(black_box(&mut out), black_box(&xs), black_box(bits));
                    black_box((out, c))
                },
            )
        });
    }
    group.finish();
}

fn bench_rshift(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpn::rshift (limbs_shr_to_out, bit-shift)");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &n in LIMB_COUNTS {
        let xs = random_limbs(n, "xs");
        let out0 = vec![0 as Limb; n];
        let bits: u64 = 17;
        group.throughput(Throughput::Elements(n as u64));
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_with_setup(
                || out0.clone(),
                |mut out| {
                    let c = limbs_shr_to_out(black_box(&mut out), black_box(&xs), black_box(bits));
                    black_box((out, c))
                },
            )
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(30);
    targets =
        bench_addmul_1,
        bench_submul_1,
        bench_mul_1,
        bench_mul_1_in_place,
        bench_add_n,
        bench_add_n_in_place,
        bench_sub_n,
        bench_lshift,
        bench_rshift
}
criterion_main!(benches);
