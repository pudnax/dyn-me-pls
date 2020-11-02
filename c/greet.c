#include <stdio.h>

// gcc -Wall -shared greet.c -o libgreet.so

void greet(const char *name) { printf("Hello, %s!\n", name); }
