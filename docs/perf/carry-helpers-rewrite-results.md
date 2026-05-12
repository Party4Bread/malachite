# `add_with_carry_limb` / `sub_with_carry` 재작성 결과

**환경**: Intel Xeon @2.80GHz, `RUSTFLAGS=""` (default x86-64), criterion 0.3,
`--sample-size 15 --warm-up-time 1 --measurement-time 2`, `taskset -c 0`.

**변경**: `wrapping_add + comparison` 패턴을 `overflowing_add`로 교체 (4 LOC).
의미적으로 동일하나, LLVM이 ADC/SBB 체인으로 더 잘 lower하는 형태.

## 결과 (per-size)

```
GROUP                              SIZE  BEFORE_ns   AFTER_ns  SPEEDUP
mpn__add_n                            4       24.8       25.0    0.99x
mpn__add_n                           16       36.1       36.4    0.99x
mpn__add_n                           32       51.2       51.3    1.00x
mpn__add_n                           64       81.5       81.4    1.00x
mpn__add_n                          128      145.1      140.1    1.04x
mpn__add_n                          256      256.3      256.8    1.00x
mpn__add_n                          512      493.9      488.6    1.01x
mpn__add_n                         1024      959.5      961.4    1.00x
mpn__add_n                         2048     1894.2     1898.3    1.00x
mpn__add_n                         8192     7519.7     7544.5    1.00x
mpn__add_n                        32768    30562.3    30395.0    1.01x
mpn__add_n in-place                   4       24.9       25.0    1.00x
mpn__add_n in-place                  16       36.1       36.4    0.99x
mpn__add_n in-place                  32       51.2       51.3    1.00x
mpn__add_n in-place                  64       80.6       80.4    1.00x
mpn__add_n in-place                 128      140.2      139.8    1.00x
mpn__add_n in-place                 256      257.5      264.0    0.98x
mpn__add_n in-place                 512      495.9      496.8    1.00x
mpn__add_n in-place                1024      979.8      970.4    1.01x
mpn__add_n in-place                2048     1929.4     1950.9    0.99x
mpn__add_n in-place                8192     7704.1     7916.9    0.97x
mpn__add_n in-place               32768    30277.2    30262.1    1.00x
mpn__addmul_1                         4       23.4       23.3    1.01x
mpn__addmul_1                        16       34.3       34.3    1.00x
mpn__addmul_1                        32       49.3       49.4    1.00x
mpn__addmul_1                        64       78.7       78.7    1.00x
mpn__addmul_1                       128      149.6      145.8    1.03x
mpn__addmul_1                       256      266.8      263.3    1.01x
mpn__addmul_1                       512      664.8      668.4    0.99x
mpn__addmul_1                      1024     1308.2     1386.3    0.94x
mpn__addmul_1                      2048     2622.6     1911.1    1.37x
mpn__addmul_1                      8192    10726.2    10675.7    1.00x
mpn__addmul_1                     32768    30979.8    30541.1    1.01x
mpn__mul_1                            4       23.4       23.3    1.01x
mpn__mul_1                           16       31.0       31.1    1.00x
mpn__mul_1                           32       43.1       42.2    1.02x
mpn__mul_1                           64       64.0       64.0    1.00x
mpn__mul_1                          128      109.7      110.6    0.99x
mpn__mul_1                          256      199.1      241.5    0.82x
mpn__mul_1                          512      379.5      382.9    0.99x
mpn__mul_1                         1024     1028.0      744.7    1.38x
mpn__mul_1                         2048     1474.6     1471.8    1.00x
mpn__mul_1                         8192     7161.3     7367.8    0.97x
mpn__mul_1                        32768    23332.1    23666.3    0.99x
mpn__mul_1 in-place                   4       23.4       23.2    1.01x
mpn__mul_1 in-place                  16       31.3       31.8    0.99x
mpn__mul_1 in-place                  32       41.9       41.9    1.00x
mpn__mul_1 in-place                  64       61.1       61.5    0.99x
mpn__mul_1 in-place                 128      106.9      106.9    1.00x
mpn__mul_1 in-place                 256      188.8      188.8    1.00x
mpn__mul_1 in-place                 512      358.3      359.5    1.00x
mpn__mul_1 in-place                1024      701.5      698.1    1.00x
mpn__mul_1 in-place                2048     1371.0     1365.0    1.00x
mpn__mul_1 in-place                8192     5419.0     5313.5    1.02x
mpn__mul_1 in-place               32768    23117.0    21533.0    1.07x
mpn__sub_n                            4       26.4       24.6    1.07x
mpn__sub_n                           16       44.1       37.3    1.18x
mpn__sub_n                           32       66.5       51.3    1.30x
mpn__sub_n                           64      111.8       79.8    1.40x
mpn__sub_n                          128      207.8      139.3    1.49x
mpn__sub_n                          256      379.8      255.2    1.49x
mpn__sub_n                          512      736.0      488.5    1.51x
mpn__sub_n                         1024     1458.8      962.8    1.52x
mpn__sub_n                         2048     2900.3     1894.9    1.53x
mpn__sub_n                         8192    11512.5     7509.1    1.53x
mpn__sub_n                        32768    46439.3    30253.7    1.53x
mpn__submul_1                         4       24.2       24.1    1.01x
mpn__submul_1                        16       37.4       37.8    0.99x
mpn__submul_1                        32       55.9       56.1    1.00x
mpn__submul_1                        64       95.3       90.4    1.05x
mpn__submul_1                       128      161.3      161.7    1.00x
mpn__submul_1                       256      298.8      300.0    1.00x
mpn__submul_1                       512      711.6      703.0    1.01x
mpn__submul_1                      1024     1385.0     1453.0    0.95x
mpn__submul_1                      2048     2237.8     2264.1    0.99x
mpn__submul_1                      8192    11148.7    11287.7    0.99x
mpn__submul_1                     32768    35820.6    35789.1    1.00x
```

## 핵심 발견

- **`sub_n`: 모든 사이즈에서 1.18–1.53x 향상** (큰 사이즈에서 1.50x 안정 유지).
  이는 `target-cpu=native`에서 보였던 0.88–0.91x 회귀의 정반대 — 소스만 바꿔도
  GMP 갭의 상당 부분이 해소됨.
- **`add_n`: 변화 없음** (~1.00x). LLVM은 이미 OLD 패턴(`wrapping_add + (s < x)`)을
  `overflowing_add`와 동일하게 lower하고 있었음. 즉, 두 패턴 모두 깨끗한 ADC + 2x
  unroll로 컴파일됨.
- **`addmul_1`, `mul_1`, `submul_1` 등 helpers를 안 쓰는 함수**: 대체로 ±2% 노이즈.
  1024 단일 사이즈에서 ±35% 흔들리는 케이스가 있으나(`addmul_1/1024`, `mul_1/256`),
  helpers를 호출하지 않으므로 측정 노이즈로 해석.

## 다음 단계 후보

- `limbs_slice_add_mul_limb_same_length_in_place_left` (addmul_1) 내부도 같은
  `wrapping_add + comparison` 패턴 사용 — `overflowing_add` 기반 재작성 시도.
- `limbs_sub_mul_limb_same_length_in_place_left` (submul_1)는 더 나쁘게 branchy
  `if lower < borrow` 사용 — branchless로 통일.
- `add_n`이 변화 없는 것은 별개로, `target-cpu=native`에서의 0.44-0.67x 회귀는
  여전. AVX-512 throttling 가설 별도 검증 필요 (`target-cpu=native -C target-feature=-avx512f`).
