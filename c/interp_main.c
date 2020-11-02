#include <stdio.h>
#include <unistd.h>

// gcc -Wall -shared main.c -o libmain.so -Wl,-soname,libmain.so -Wl,-e,entry

const char interpreter[] __attribute__((section(".interp"))) =
    "/lib64/ld-linux-x86-64.so.2";

void greet(const char *name) { printf("Hello, %s!\n", name); }

void entry() {

  greet("rain");
  _exit(0);
}
