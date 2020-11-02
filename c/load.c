#include <dlfcn.h>
#include <stdio.h>

// gcc -Wall load.c -o load -ldl

typedef void (*greet_t)(const char *name);

int main() {
  void *lib = dlopen("./../libgreet.so", RTLD_LAZY);
  if (!lib) {
    fprintf(stderr, "failed to load library\n");
    return 1;
  }

  greet_t greet = (greet_t)dlsym(lib, "greet");
  if (!greet) {
    fprintf(stderr, "could not look up symbl 'greet'\n");
    return 1;
  }

  greet("venus");
  dlclose(lib);
  return 0;
}
