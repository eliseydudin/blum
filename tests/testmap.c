///
///  Test whether the `map_t` type actually works
///

#include <assert.h>
#include <blum/blum.h>

int main() {
  map_t(int) nums;
  map_init(&nums);

  map_set(&nums, "num1", 10);
  map_set(&nums, "num2", 20);
  map_set(&nums, "num2", 30);

  assert(*map_get(&nums, "num1") == 10);
  assert(*map_get(&nums, "num2") == 30);

  map_deinit(&nums);

  return 0;
}
