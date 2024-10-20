#include <blum/object.h>
#include <stdlib.h>

b_obj *b_obj_new(void) {
  b_obj *ret = malloc(sizeof(b_obj));

  ret->type_identifier = "object";
  map_init(&ret->children);
  ret->opaque_data = NULL;
  ret->methods = NULL;

  return ret;
}

void b_obj_del(b_obj *obj) {
  map_deinit(&obj->children);

  if (obj->methods != NULL) {
    map_deinit(obj->methods);
  }

  free(obj);
}

bool b_hasmeth(b_obj *obj, const char *meth) {
  if (obj->methods == NULL)
    return 0;

  return map_get(obj->methods, meth) != NULL;
}

b_obj *b_call(b_obj *obj, const char *meth) {
  if (!b_hasmeth(obj, meth)) {
    return NULL;
  }

  b_obj_meth *method = map_get(obj->methods, meth);
  return (*method)(obj, NULL);
}

void b_obj_add_method(b_obj *self, const char *methname, b_obj_meth meth) {
  if (self->methods == NULL) {
    self->methods = malloc(sizeof(map_t(b_obj_meth)));
    map_init(self->methods);
  }

  map_set(self->methods, methname, meth);
}
