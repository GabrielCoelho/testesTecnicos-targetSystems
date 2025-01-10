# Primeiro exercício

Observe o trecho de código abaixo:

int INDICE = 13, SOMA = 0, K = 0;
Enquanto K < INDICE faça { K = K + 1; SOMA = SOMA + K; }
Imprimir(SOMA);

## Ao final do processamento, qual será o valor da variável SOMA?

Resposta: ==91==

```c
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
```
