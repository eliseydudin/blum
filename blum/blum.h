#ifndef __BLUM_H__
#define __BLUM_H__

#include <stdlib.h>

typedef struct object (*method_t)(struct object *, struct object *);

typedef struct object_mapping {
} object_mapping;

typedef struct object {
  object_mapping *mapping;
  struct object **children;
} object;

#endif
