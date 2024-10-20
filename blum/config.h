#ifndef __CONFIG_H__
#define __CONFIG_H__

#if !defined(_WIN32) || !(defined(_WIN64))
  #define B_EXPORT __attribute__((visibility("default")))
#else
  #define B_EXPORT __declspec(dllexport)
#endif

#endif
