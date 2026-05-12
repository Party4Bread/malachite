# malachite-criterion-bench

Criterion-driven benchmarks for [`malachite-nz`], with side-by-side comparisons
against [`num`] and [`rug`] (GMP). Used to drive performance work on the mpn
layer.

[`malachite-nz`]: ../malachite-nz
[`num`]: https://crates.io/crates/num
[`rug`]: https://crates.io/crates/rug

## Benches

### `Natural`-level (user-facing operations)

| Bench            | What it measures                          |
|------------------|-------------------------------------------|
| `natural_add`    | `x + y` across sizes                      |
| `natural_sub`    | `x - y` across sizes                      |
| `natural_mul`    | `x * y` across sizes                      |
| `natural_sqr`    | `x.square()` (and `x*x` for `num`)        |
| `natural_div`    | `x / y` with `bits(x) = 2 * bits(y)`      |
| `natural_shift`  | `x << k` and `x >> k` at small/medium/large `k` |

All Natural-level benches cover bit sizes `64, 256, 1024, 4096, 16384, 65536,
262144, 1048576` (1Mb), which spans the basecase, Karatsuba, Toom-Cook, and FFT
thresholds.

### mpn-level (inner loops)

`mpn_primitives` calls the internal `limbs_*` primitives directly — the hot
loops where GMP's hand-written assembly (MULX/ADCX/ADOX, dual carry chain)
typically wins. This is where optimization work shows up first.

| Group                | Equivalent GMP function    |
|----------------------|----------------------------|
| `mpn::addmul_1`      | `mpn_addmul_1`             |
| `mpn::submul_1`      | `mpn_submul_1`             |
| `mpn::mul_1`         | `mpn_mul_1`                |
| `mpn::mul_1 in-place`| `mpn_mul_1` (rp == up)     |
| `mpn::add_n`         | `mpn_add_n`                |
| `mpn::add_n in-place`| `mpn_add_n` (rp == up)     |
| `mpn::sub_n`         | `mpn_sub_n`                |
| `mpn::lshift`        | `mpn_lshift`               |
| `mpn::rshift`        | `mpn_rshift`               |

Limb counts span `4 .. 32768` to cover both single-cacheline runs and
multi-megabyte arrays.

The mpn benches require the `test_build` feature on `malachite-nz` (already
wired up in this crate's `Cargo.toml`). That feature converts the internal
`pub_test!` / `pub_crate_test!` macros into real `pub` so the bench can
import `limbs_*` symbols.

## Running

```bash
# All benches, pinned to core 0, with -C target-cpu=native:
./scripts/bench.sh

# A single bench:
./scripts/bench.sh natural_mul

# Filter inside a bench (criterion regex):
./scripts/bench.sh mpn_primitives addmul_1

# Just the malachite implementation in a Natural-level bench:
./scripts/bench.sh natural_mul malachite
```

### Baseline / regression workflow

```bash
# 1. On the starting revision (e.g. main):
./scripts/save-baseline.sh main

# 2. Make changes. Then compare:
COMPARE_TO=main ./scripts/bench.sh
```

criterion writes baselines under `target/criterion/<group>/<id>/<baseline>/`.

### Extracting a malachite-vs-GMP ratio table

After running a bench, `scripts/compare-against-gmp.sh` walks the criterion
output and prints a table of `malachite_ns / rug_ns` per (group, size).
Requires `jq`.

## Notes on reproducibility

- `bench.sh` sets `RUSTFLAGS="-C target-cpu=native"`. To opt out, run
  `RUSTFLAGS_EXTRA= ./scripts/bench.sh`.
- `bench.sh` pins to CPU 0 when `taskset` is available.
- The workspace `release` profile already uses `lto = "fat"` and
  `codegen-units = 1`.
- Disable Turbo Boost / set the performance governor before serious runs
  if you have access to either knob.

## Adding a new bench

1. Create `benches/<name>.rs` (use one of the existing files as a template —
   each defines a tiny `natural_to_biguint` / `natural_to_rug_integer`
   helper inline because bench targets can't share a module).
2. Add `[[bench]] name = "<name>" harness = false` to `Cargo.toml`.
3. For an mpn-level bench, import from `malachite_nz::natural::arithmetic::*`;
   the `test_build` feature already exposes the `limbs_*` primitives.
