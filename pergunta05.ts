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
