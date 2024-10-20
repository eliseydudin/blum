#ifndef __CONFIG_H__
#define __CONFIG_H__

#ifndef _WIN32
  #define B_EXPORT __attribute__((visibility("default")))
#else
  #define __declspec(dllexport)
#endif

#endif
