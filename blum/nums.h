#ifndef __NUMS_H__
#define __NUMS_H__

#include <stdint.h>

typedef uint64_t u64;
typedef uint32_t u32;
typedef uint16_t u16;
typedef uint8_t u8;

#ifndef bool
typedef u8 bool;
  #define true 1
  #define false 0
#endif

#endif
