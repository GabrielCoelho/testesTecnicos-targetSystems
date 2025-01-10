#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  int index = 13, k = 0, sum = 0;

  while (k < index) {
    k += 1;
    sum = sum + k;
  }

  printf("%d", sum);
  return EXIT_SUCCESS;
}
