#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

__thread int a = 0;

void *work() {
  for (int a = 0; a < 3; a++) {
    printf("[%lu] a =%d\n", pthread_self() % 10, a);
    sleep(1);
  }
  return NULL;
}

int main() {
  pthread_t t1, t2, t3;

  pthread_create(&t1, NULL, work, NULL);
  pthread_create(&t2, NULL, work, NULL);
  pthread_create(&t3, NULL, work, NULL);
  sleep(4);
  return 0;
}
