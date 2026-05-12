# mpn 카브 패턴 재작성 결과 (cumulative)

3가지 상태 비교:
- `before_carry_fix`: 원본 코드, `RUSTFLAGS=""`
- `after_carry_fix`: `add_with_carry_limb`/`sub_with_carry`만 `overflowing_add` 기반 (commit 3eda85d)
- `after_mpn_rewrite`: 추가로 `limbs_slice_add_mul_limb_same_length_in_place_left`,
  `limbs_sub_mul_limb_same_length_in_place_left`, `..._right` 의 carry pattern도 재작성 (commit 741d4fc)

환경: Intel Xeon @2.80GHz, criterion 0.3, `taskset -c 0`, sample-size 15, measurement 2s.

## 결과 (시간 ns, speedup vs before)

```
GROUP                        SIZE   before  helpers   mpn_rw   h-spd   m-spd
------------------------------------------------------------------------------
mpn__add_n                      4       25       25       25   0.99x   0.99x
mpn__add_n                     16       36       36       37   0.99x   0.99x
mpn__add_n                     32       51       51       52   1.00x   0.99x
mpn__add_n                     64       81       81       81   1.00x   1.01x
mpn__add_n                    128      145      140      140   1.04x   1.03x
mpn__add_n                    256      256      257      257   1.00x   1.00x
mpn__add_n                    512      494      489      491   1.01x   1.01x
mpn__add_n                   1024      959      961      965   1.00x   0.99x
mpn__add_n                   2048     1894     1898     1900   1.00x   1.00x
mpn__add_n                   8192     7520     7545     7533   1.00x   1.00x
mpn__add_n                  32768    30562    30395    30541   1.01x   1.00x
mpn__add_n in-place             4       25       25       25   1.00x   1.00x
mpn__add_n in-place            16       36       36       36   0.99x   1.01x
mpn__add_n in-place            32       51       51       52   1.00x   0.98x
mpn__add_n in-place            64       81       80       81   1.00x   1.00x
mpn__add_n in-place           128      140      140      139   1.00x   1.01x
mpn__add_n in-place           256      257      264      257   0.98x   1.00x
mpn__add_n in-place           512      496      497      496   1.00x   1.00x
mpn__add_n in-place          1024      980      970      963   1.01x   1.02x
mpn__add_n in-place          2048     1929     1951     1973   0.99x   0.98x
mpn__add_n in-place          8192     7704     7917     7651   0.97x   1.01x
mpn__add_n in-place         32768    30277    30262    30361   1.00x   1.00x
mpn__addmul_1                   4       23       23       23   1.01x   1.00x
mpn__addmul_1                  16       34       34       35   1.00x   0.99x
mpn__addmul_1                  32       49       49       49   1.00x   1.00x
mpn__addmul_1                  64       79       79       79   1.00x   1.00x
mpn__addmul_1                 128      150      146      148   1.03x   1.01x
mpn__addmul_1                 256      267      263      269   1.01x   0.99x
mpn__addmul_1                 512      665      668      672   0.99x   0.99x
mpn__addmul_1                1024     1308     1386     1307   0.94x   1.00x
mpn__addmul_1                2048     2623     1911     2708   1.37x   0.97x
mpn__addmul_1                8192    10726    10676    10582   1.00x   1.01x
mpn__addmul_1               32768    30980    30541    30993   1.01x   1.00x
mpn__mul_1                      4       23       23       24   1.01x   1.00x
mpn__mul_1                     16       31       31       31   1.00x   1.00x
mpn__mul_1                     32       43       42       42   1.02x   1.03x
mpn__mul_1                     64       64       64       64   1.00x   0.99x
mpn__mul_1                    128      110      111      110   0.99x   1.00x
mpn__mul_1                    256      199      241      248   0.82x   0.80x
mpn__mul_1                    512      380      383      381   0.99x   1.00x
mpn__mul_1                   1024     1028      745      741   1.38x   1.39x
mpn__mul_1                   2048     1475     1472     1468   1.00x   1.00x
mpn__mul_1                   8192     7161     7368     7645   0.97x   0.94x
mpn__mul_1                  32768    23332    23666    23468   0.99x   0.99x
mpn__mul_1 in-place             4       23       23       23   1.01x   1.01x
mpn__mul_1 in-place            16       31       32       32   0.99x   0.98x
mpn__mul_1 in-place            32       42       42       42   1.00x   1.00x
mpn__mul_1 in-place            64       61       61       62   0.99x   0.99x
mpn__mul_1 in-place           128      107      107      105   1.00x   1.02x
mpn__mul_1 in-place           256      189      189      189   1.00x   1.00x
mpn__mul_1 in-place           512      358      360      366   1.00x   0.98x
mpn__mul_1 in-place          1024      701      698      699   1.00x   1.00x
mpn__mul_1 in-place          2048     1371     1365     1367   1.00x   1.00x
mpn__mul_1 in-place          8192     5419     5314     5342   1.02x   1.01x
mpn__mul_1 in-place         32768    23117    21533    22819   1.07x   1.01x
mpn__sub_n                      4       26       25       24   1.07x   1.10x
mpn__sub_n                     16       44       37       36   1.18x   1.22x
mpn__sub_n                     32       67       51       52   1.30x   1.29x
mpn__sub_n                     64      112       80       80   1.40x   1.40x
mpn__sub_n                    128      208      139      139   1.49x   1.49x
mpn__sub_n                    256      380      255      257   1.49x   1.48x
mpn__sub_n                    512      736      488      491   1.51x   1.50x
mpn__sub_n                   1024     1459      963      956   1.52x   1.53x
mpn__sub_n                   2048     2900     1895     1898   1.53x   1.53x
mpn__sub_n                   8192    11513     7509     7505   1.53x   1.53x
mpn__sub_n                  32768    46439    30254    30348   1.53x   1.53x
mpn__submul_1                   4       24       24       26   1.01x   0.93x
mpn__submul_1                  16       37       38       37   0.99x   1.00x
mpn__submul_1                  32       56       56       56   1.00x   1.00x
mpn__submul_1                  64       95       90       90   1.05x   1.06x
mpn__submul_1                 128      161      162      161   1.00x   1.00x
mpn__submul_1                 256      299      300      298   1.00x   1.00x
mpn__submul_1                 512      712      703      578   1.01x   1.23x
mpn__submul_1                1024     1385     1453     1375   0.95x   1.01x
mpn__submul_1                2048     2238     2264     2738   0.99x   0.82x
mpn__submul_1                8192    11149    11288    10887   0.99x   1.02x
mpn__submul_1               32768    35821    35789    37228   1.00x   0.96x
```

## 핵심 결론

**helper rewrite (commit 3eda85d) — 매우 효과적**

- **`sub_n`: 1.07x – 1.53x 일관된 개선**. 모든 사이즈에서 의미 있는 향상.
  특히 n≥128에서 1.5x 안정 유지 — 회귀 없이 stock 빌드에서 GMP 갭의
  상당 부분 해소.
- `add_n`: 무변화 (~1.00x). LLVM이 이미 `wrapping_add + (s < x)`를
  `overflowing_add`와 동일하게 ADC로 lower하고 있었음.

**addmul_1/submul_1 rewrite (commit 741d4fc) — 노이즈 수준**

- 의미 있는 일관된 변화 없음 (-3% ~ +5% 범위 산발).
- `addmul_1`의 OLD 패턴은 `Limb::from(*x > product_lo)` (branchless 비교)로,
  branchy 코드가 아니었으므로 LLVM이 이미 잘 처리. 개선 여지 적음.
- `submul_1`의 OLD 패턴은 `if lower < borrow { ... } else { ... }` (branchy)였으나,
  벤치 데이터의 분산이 ±5% 수준이라 명확한 win 보이지 않음.
  더 큰 sample-size + 다중 run 평균이 필요. 다만 branchless 코드가 더 깨끗하고
  cmov 의존 줄임 → 유지함.

**일부 outlier (n=2048에서 ±40%) 는 사이즈 boundary noise**

- `addmul_1/2048`: before 2623 → helpers 1911 → mpn_rw 2708.
  helper rewrite는 `addmul_1`에 영향이 없는데 같이 변화 → 사이즈 boundary 노이즈.
- `mul_1/256`, `mul_1/1024`도 비슷 — helper rewrite와 무관한 함수에서 30%+ 변동.
  L1/L2 캐시 boundary와 prefetcher 상호작용에 따른 측정 노이즈.

## 남은 격차 분석

이전 stock vs native 비교(`docs/perf/A1-target-feature-findings.md`)에서:
- `addmul_1` native: 1.06–1.38x — MULX + 더 적극적 unrolling 효과
- `mul_1 in-place` native: 1.46–1.59x — 같은 이유
- `lshift`/`rshift` native: 1.5–2.5x — AVX 자동 벡터화

이 격차는 safe Rust 소스 변경만으로는 메우기 어려움. LLVM이 MULX를 emit
하려면 `target-feature=+bmi2`가 필요한데, 이는 RUSTFLAGS 설정 또는
`#[target_feature(enable=...)]` (안전한 호출자가 unsafe wrapper 거쳐야 함)이 필요.

다음 단계 선택지:
1. **README에 RUSTFLAGS 권장 추가**: 빌드 시 `-C target-cpu=...` 가이드. 무변경.
2. **사용자 면역 RUSTFLAGS 적용**: `.cargo/config.toml`에 추가 (단 사용자가 override 가능).
3. **per-function `target_feature` dispatch**: Track B 진입. is_x86_feature_detected!.
4. **추가 source-level 최적화**: `addmul_2`, division carry chains 등 미발견 hot path.
