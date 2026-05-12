// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use criterion::*;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::natural::Natural;
use malachite_nz::natural::random::get_random_natural_with_bits;
use num::BigUint;
use std::str::FromStr;

fn natural_to_biguint(n: &Natural) -> BigUint {
    BigUint::from_str(n.to_string().as_ref()).unwrap()
}

fn natural_to_rug_integer(n: &Natural) -> rug::Integer {
    rug::Integer::from_str(n.to_string().as_ref()).unwrap()
}

const BIT_SIZES: &[u64] = &[256, 1024, 4096, 16384, 65536, 262144, 1048576];

// Shift amounts: a non-limb-aligned small shift exercises the bit-shift path;
// large limb-aligned shifts exercise the memcpy/window path.
fn bench_shl(c: &mut Criterion) {
    let mut group = c.benchmark_group("Natural << u64");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &bits in BIT_SIZES {
        let x = get_random_natural_with_bits(
            &mut random_primitive_ints(EXAMPLE_SEED.fork("a")),
            bits,
        );
        let xn = natural_to_biguint(&x);
        let xr = natural_to_rug_integer(&x);
        for &amt in &[5u64, 67, 4096] {
            let label = format!("{}b@{}", bits, amt);
            group.bench_function(BenchmarkId::new("malachite", &label), |b| {
                b.iter_with_setup(|| x.clone(), |x| x << amt)
            });
            group.bench_function(BenchmarkId::new("num", &label), |b| {
                b.iter_with_setup(|| xn.clone(), |x| x << amt as usize)
            });
            group.bench_function(BenchmarkId::new("rug", &label), |b| {
                b.iter_with_setup(|| xr.clone(), |x| x << amt as u32)
            });
        }
    }
    group.finish();
}

fn bench_shr(c: &mut Criterion) {
    let mut group = c.benchmark_group("Natural >> u64");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &bits in BIT_SIZES {
        let x = get_random_natural_with_bits(
            &mut random_primitive_ints(EXAMPLE_SEED.fork("a")),
            bits,
        );
        let xn = natural_to_biguint(&x);
        let xr = natural_to_rug_integer(&x);
        for &amt in &[5u64, 67, 4096] {
            let label = format!("{}b@{}", bits, amt);
            group.bench_function(BenchmarkId::new("malachite", &label), |b| {
                b.iter_with_setup(|| x.clone(), |x| x >> amt)
            });
            group.bench_function(BenchmarkId::new("num", &label), |b| {
                b.iter_with_setup(|| xn.clone(), |x| x >> amt as usize)
            });
            group.bench_function(BenchmarkId::new("rug", &label), |b| {
                b.iter_with_setup(|| xr.clone(), |x| x >> amt as u32)
            });
        }
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = bench_shl, bench_shr
}
criterion_main!(benches);
