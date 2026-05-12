# A1 검증 결과: `-C target-cpu=native` 효과 측정

## 결론 요약

`-C target-cpu=native`를 전역으로 켜는 **블랭킷 적용은 절대 안 됨**.
일부 함수는 1.5~2.5x 빨라지지만, 일부는 **2.3x까지 느려짐**.

| 영향 패턴 | 함수 | native 효과 |
|----------|-----|------------|
| 곱셈 류 (MULX 활용) | `mul_1`, `addmul_1`, `submul_1`, `mul_1` in-place | **+5~60% 빨라짐** |
| 시프트 류 (벡터화) | `lshift`, `rshift` | **+50~150% 빨라짐** |
| 단순 가산/감산 (carry 체인) | `add_n`, `add_n` in-place, `sub_n` | **−10~56% 느려짐** |

해석:
- 곱셈 루프는 MULX(no implicit RAX/RDX) + LLVM의 더 적극적 unrolling 덕에 win
- 시프트 루프는 LLVM이 AVX2/AVX-512로 auto-vectorize → 대박 win
- carry 체인은 vectorize 불가 + AVX-512 frequency throttling 의심 → loss

## 측정 환경

- Intel Xeon @ 2.80GHz (AVX-512 + BMI2 + ADX 지원)
- rustc 1.94.1, criterion 0.3
- `taskset -c 0` 고정, `--sample-size 15 --warm-up-time 1 --measurement-time 2`
- 기준선: `RUSTFLAGS=""` (default x86-64)
- 후보: `RUSTFLAGS="-C target-cpu=native"`
- bench: `malachite-criterion-bench/benches/mpn_primitives.rs`

## 그룹별 결과 (per-size speedup of native vs stock)

```
add_n            min=0.44x med=0.58x max=1.03x
                 4:1.03 16:0.64 32:0.67 64:0.63 128:0.60 256:0.58 512:0.58
                 1024:0.57 2048:0.44 8192:0.44 32768:0.57

add_n in-place   min=0.60x med=0.62x max=0.90x
                 4:0.90 16:0.75 32:0.71 64:0.66 128:0.64 256:0.62 512:0.62
                 1024:0.61 2048:0.60 8192:0.60 32768:0.60

addmul_1         min=0.89x med=1.08x max=1.38x
                 4:0.97 16:1.14 32:1.05 64:1.09 128:1.17 256:1.14 512:1.08
                 1024:1.38 2048:1.06 8192:1.05 32768:0.89

lshift           min=1.05x med=2.18x max=2.54x
                 4:1.05 16:1.19 32:1.51 64:1.79 128:2.05 256:2.34 512:2.45
                 1024:2.54 2048:2.29 8192:2.33 32768:2.18

mul_1            min=0.99x med=1.19x max=1.38x
                 4:0.99 16:1.07 32:1.11 64:1.19 128:1.24 256:1.37 512:1.01
                 1024:1.38 2048:1.22 8192:1.20 32768:1.02

mul_1 in-place   min=1.00x med=1.46x max=1.59x
                 4:1.00 16:1.11 32:1.14 64:1.24 128:1.38 256:1.52 512:1.53
                 1024:1.59 2048:1.59 8192:1.46 32768:1.48

rshift           min=1.03x med=1.78x max=2.14x
                 4:1.03 16:1.21 32:1.39 64:1.60 128:1.75 256:1.78 512:2.11
                 1024:2.14 2048:1.79 8192:1.83 32768:1.78

sub_n            min=0.88x med=0.90x max=0.97x
                 4:0.97 16:0.92 32:0.91 64:0.91 128:0.91 256:0.88 512:0.90
                 1024:0.90 2048:0.90 8192:0.90 32768:0.90

submul_1         min=0.97x med=1.19x max=1.41x
                 4:0.97 16:1.11 32:1.19 64:1.23 128:1.36 256:1.36 512:1.13
                 1024:1.41 2048:1.40 8192:1.15 32768:1.11
```

## Codegen 검증

표준 RUSTFLAGS별로 같은 함수가 어떻게 컴파일되는지:

| RUSTFLAGS | MUL/MULX | Carry 체인 | Unrolling |
|-----------|----------|-----------|-----------|
| (default x86-64) | MUL (RAX/RDX 묶임) | setb + add + adc | 없음 |
| `+bmi2,+adx` | MULX | setb + add + adc | 없음 |
| `target-cpu=native` | MULX | setb + add + adc | 2x ~ 4x |

핵심: **LLVM 19/20은 ADCX/ADOX를 자동으로 emit하지 않음**. dual carry chain
패턴을 safe Rust로 명시해도 단일 ADC 체인으로 컴파일됨. ADCX/ADOX를 쓰려면
`_addcarryx_u64` intrinsic이나 inline `asm!` 필요 (Track B의 영역).

또한 LLVM은:
- `overflowing_add` → ADC를 잘 emit함 (add_n에 적합)
- `wrapping_add + comparison` → ADC를 못 emit함 (malachite 현재 패턴)
- u128 곱셈 → MULX 또는 MUL (target-feature에 따라)

## `add_n` 회귀의 원인 (가설)

malachite의 `add_with_carry_limb`은 다음 패턴을 사용:
```rust
let result_no_carry = x.wrapping_add(y);
let result = result_no_carry.wrapping_add(carry);
let carry = T::from((result_no_carry < x) || (result < result_no_carry));
```

LLVM은 이 비교 기반 carry 검출을 깨끗한 ADC 체인으로 인식하지 못함.
`target-cpu=native`에서는 이 비효율적 패턴 위에 AVX-512 벡터 path를 시도(혹은
혼합 코드 생성)하여 더 나빠짐. AVX-512 frequency throttling도 의심됨.

권장: `add_with_carry_limb`을 `overflowing_add` 기반으로 바꾸기:
```rust
pub(crate) fn add_with_carry_limb<T: PrimitiveUnsigned>(x: T, y: T, carry: T) -> (T, T) {
    let (s1, c1) = x.overflowing_add(y);
    let (s2, c2) = s1.overflowing_add(carry);
    (s2, T::from(c1 | c2))
}
```

asm_check 실험에서 `overflowing_add` 버전은 stock으로도 깨끗한 `adc` + 2x
unroll을 emit하는 것을 확인. malachite의 비교 기반 버전은 더 느릴 가능성.

## A1 권장 적용 방식

- **전역 `target-cpu=native`/`-C target-feature` 금지**: 회귀 위험
- **함수 단위 `#[target_feature(enable = "bmi2")]`**: 곱셈/시프트만 선별 적용
  - `limbs_mul_limb_to_out`, `limbs_slice_mul_limb_in_place`
  - `limbs_slice_add_mul_limb_same_length_in_place_left`
  - `limbs_sub_mul_limb_same_length_in_place_left`
  - `limbs_shl_to_out`, `limbs_shr_to_out`
- **추가 quick win**: `add_with_carry_limb`을 `overflowing_add` 기반으로 교체
- **추가 quick win**: `submul_1`의 branchy 조건을 `addmul_1`처럼 branchless로 통일

## 다음 단계

1. `add_with_carry_limb` 패턴 교체 → add_n/sub_n 회귀 자체를 제거
2. 곱셈/시프트 함수에 per-fn `#[target_feature(enable="bmi2")]` 적용
3. `target-cpu=native -C target-feature=-avx512f` 변형도 테스트 (AVX-512 throttling 격리)
4. (Track A2) addmul_1 dual carry chain 명시 패턴 재시도 — A1만으로 부족한 잔여 격차 메우기
