#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

// gcc -Wall load.c -o load -ldl

void assert(void *p) {
  if (!p) {
    fprintf(stderr, "woops");
    exit(1);
  }
}

// this function is 101% pragmatic, don't @ me
void print_mapping_count() {
  const size_t buf_size = 1024;
  char buf[buf_size];
  printf("mapping count: ");
  fflush(stdout);
  snprintf(buf, buf_size, "bash -c 'cat /proc/%d/maps | grep libgreet | wc -l'",
           getpid());
  system(buf);
}

int main(void) {
  print_mapping_count();

  printf("> dlopen(RTLD_NOW)\n");
  void *lib = dlopen("./../libgreet.so", RTLD_NOW);
  assert(lib);
  print_mapping_count();

  printf("> dlclose()\n");
  dlclose(lib);
  print_mapping_count();

  return 0;
}
