// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use criterion::*;
use malachite_base::num::arithmetic::traits::Square;
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

const BIT_SIZES: &[u64] = &[64, 256, 1024, 4096, 16384, 65536, 262144, 1048576];

fn bench_sqr(c: &mut Criterion) {
    let mut group = c.benchmark_group("Natural.square()");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &bits in BIT_SIZES {
        let x = get_random_natural_with_bits(
            &mut random_primitive_ints(EXAMPLE_SEED.fork("a")),
            bits,
        );
        let xn = natural_to_biguint(&x);
        let xr = natural_to_rug_integer(&x);
        group.throughput(Throughput::Elements(bits));
        group.bench_function(BenchmarkId::new("malachite", bits), |b| {
            b.iter_with_setup(|| x.clone(), |x| x.square())
        });
        // num::BigUint has no dedicated square; benchmark x*x for comparison.
        group.bench_function(BenchmarkId::new("num (x*x)", bits), |b| {
            b.iter_with_setup(|| (xn.clone(), xn.clone()), |(a, b)| a * b)
        });
        group.bench_function(BenchmarkId::new("rug (x.square())", bits), |b| {
            b.iter_with_setup(|| xr.clone(), |x| x.square())
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(15);
    targets = bench_sqr
}
criterion_main!(benches);
