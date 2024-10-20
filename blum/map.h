#ifndef __BLUM_MAP_H__
#define __BLUM_MAP_H__

///
/// Source from https://github.com/rxi/map
/// Embedding this to reduce the amount of required libraries.
///

#include <string.h>

#include "config.h"

struct map_node_t;
typedef struct map_node_t map_node_t;

typedef struct {
  map_node_t **buckets;
  unsigned nbuckets, nnodes;
} map_base_t;

typedef struct {
  unsigned bucketidx;
  map_node_t *node;
} map_iter_t;

#define map_t(T) \
  struct { \
    map_base_t base; \
    T *ref; \
    T tmp; \
  }

#define map_init(m) memset(m, 0, sizeof(*(m)))
#define map_deinit(m) map_deinit_(&(m)->base)

#define map_get(m, key) ((m)->ref = map_get_(&(m)->base, key))
#define map_set(m, key, value) \
  ((m)->tmp = (value), map_set_(&(m)->base, key, &(m)->tmp, sizeof((m)->tmp)))
#define map_remove(m, key) map_remove_(&(m)->base, key)

void B_EXPORT map_deinit_(map_base_t *m);
void B_EXPORT *map_get_(map_base_t *m, const char *key);
int B_EXPORT map_set_(map_base_t *m, const char *key, void *value, int vsize);
void B_EXPORT map_remove_(map_base_t *m, const char *key);

#endif
