#ifndef __OBJECT_H__
#define __OBJECT_H__

#include "map.h"
#include "nums.h"

typedef struct b_obj *(*b_obj_meth)(struct b_obj *self, struct b_obj *args);

typedef struct b_obj {
  map_t(b_obj_meth) * methods;
  map_t(struct b_obj *) children;
  const char *type_identifier;
  void *opaque_data;
} b_obj;

/// Create a new `b_obj *` of type `object`
b_obj *b_obj_new(void);
/// Deallocate the given object. Don't forget to free the `opaque_data` field yourself
void b_obj_del(b_obj *);
/// Checks if the method exists
bool b_hasmeth(b_obj *, const char *);
/// Call a method without giving args
b_obj *b_call(b_obj *, const char *);
/// Add a method to an object
void b_obj_add_method(b_obj *, const char *, b_obj_meth);

#endif
