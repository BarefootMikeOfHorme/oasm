// FastMath - Optimized mathematical operations for OASM runtime
// Provides SIMD-accelerated computations when available

#include <iostream>
#include <cmath>
#include <cstdint>
#include <vector>
#include <immintrin.h>  // SSE/AVX intrinsics

extern "C" {

// Fast integer addition (example - can be SIMD-optimized)
int32_t fast_add_i32(int32_t a, int32_t b) {
    return a + b;
}

// Fast floating-point operations
float fast_sqrt(float x) {
    return std::sqrt(x);
}

double fast_sin(double x) {
    return std::sin(x);
}

// Vector operations (SIMD when possible)
void fast_vec_add_f32(float* dest, const float* a, const float* b, size_t len) {
    for (size_t i = 0; i < len; ++i) {
        dest[i] = a[i] + b[i];
    }
    // TODO: Use SSE/AVX intrinsics for performance
}

// Checksum for data integrity (executive function: verify operations)
uint32_t fast_checksum(const uint8_t* data, size_t len) {
    uint32_t sum = 0;
    for (size_t i = 0; i < len; ++i) {
        sum += data[i];
    }
    return sum;
}

} // extern "C"

// Test harness
int main() {
    std::cout << "FastMath Library v0.1" << std::endl;
    std::cout << "Test: 5 + 3 = " << fast_add_i32(5, 3) << std::endl;
    std::cout << "Test: sqrt(16) = " << fast_sqrt(16.0f) << std::endl;
    return 0;
}
