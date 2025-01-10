# Pergunta 05

Escreva um programa que inverta os caracteres de um string.

IMPORTANTE:
a) Essa string pode ser informada através de qualquer entrada de sua preferência ou pode ser previamente definida no código;
b) Evite usar funções prontas, como, por exemplo, reverse;

```typescript
function reverseString(input: string): string {
  let reversed = "";

  for (let i = input.length - 1; i >= 0; i--) {
    reversed += input[i];
  }

  return reversed;
}
const inputString = "Exemplo de string a ser invertida";
const result = reverseString(inputString);
console.log("String original:", inputString);
console.log("String invertida:", result);
```
