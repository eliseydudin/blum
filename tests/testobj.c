#include <assert.h>
#include <blum/blum.h>

b_obj *func(b_obj *_self, b_obj *_unused) {
  return (b_obj *)10;
}

int main() {
  b_obj *obj = b_obj_new();
  b_obj_add_method(obj, "func", func);

  assert(b_hasmeth(obj, "func"));
  assert(b_call(obj, "func") == (b_obj *)10);

  b_obj_del(obj);
  return 0;
}
