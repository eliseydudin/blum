#ifndef __NUMS_H__
#define __NUMS_H__

#include <stdint.h>

typedef uint64_t u64;
typedef uint32_t u32;
typedef uint16_t u16;
typedef uint8_t u8;

typedef int64_t i64;
typedef int32_t i32;
typedef int16_t i16;
typedef int8_t i8;

typedef float f32;
typedef double f64;

#ifndef bool
typedef u8 bool;
  #define true 1
  #define false 0
#endif

#endif
