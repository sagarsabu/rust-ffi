#include "c-lib.h"

#include <stdio.h>

int c_lib_add_together(int a, int b) {
  printf("c_lib_add_together %d + %d\n", a, b);
  return a + b;
}
