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

const BIT_SIZES: &[u64] = &[64, 256, 1024, 4096, 16384, 65536, 262144, 1048576];

fn bench_sub(c: &mut Criterion) {
    let mut group = c.benchmark_group("Natural - Natural");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for &bits in BIT_SIZES {
        let big = get_random_natural_with_bits(
            &mut random_primitive_ints(EXAMPLE_SEED.fork("a")),
            bits + 1,
        );
        let small = get_random_natural_with_bits(
            &mut random_primitive_ints(EXAMPLE_SEED.fork("b")),
            bits,
        );
        let bn = natural_to_biguint(&big);
        let sn = natural_to_biguint(&small);
        let br = natural_to_rug_integer(&big);
        let sr = natural_to_rug_integer(&small);
        group.throughput(Throughput::Elements(bits));
        group.bench_function(BenchmarkId::new("malachite", bits), |b| {
            b.iter_with_setup(|| (big.clone(), small.clone()), |(x, y)| x - y)
        });
        group.bench_function(BenchmarkId::new("num", bits), |b| {
            b.iter_with_setup(|| (bn.clone(), sn.clone()), |(x, y)| x - y)
        });
        group.bench_function(BenchmarkId::new("rug", bits), |b| {
            b.iter_with_setup(|| (br.clone(), sr.clone()), |(x, y)| x - y)
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(20);
    targets = bench_sub
}
criterion_main!(benches);
