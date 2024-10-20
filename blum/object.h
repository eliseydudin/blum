#ifndef __OBJECT_H__
#define __OBJECT_H__

#include <blum/map.h>

typedef struct b_obj *(*b_obj_meth)(struct b_obj *self, struct b_obj *args);

typedef struct {
  map_t(b_obj_meth) methods;
} b_obj_methmap;

typedef struct {
  b_obj_methmap *methods;
  struct b_obj **children;
  char *type_identifier;
  void *opaque_data;
} b_obj;

extern const b_obj *B_OBJ_NULL;

/// Create a new empty `b_obj *`
b_obj *b_obj_new(void);

#endif
