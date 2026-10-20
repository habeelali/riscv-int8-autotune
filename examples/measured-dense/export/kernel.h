/* RV32IMC INT8 kernel; see README.md for lengths, quantization and non-aliasing. */
#ifndef RISCV_INT8_DENSE_DEMO_H
#define RISCV_INT8_DENSE_DEMO_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
void riscv_int8_dense_demo(const int8_t *input, const int8_t *weights,
    const int32_t *bias, const int32_t *multiplier, const int32_t *shift, int8_t *output);
#ifdef __cplusplus
}
#endif
#endif
