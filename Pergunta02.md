# Pergunta 02

Dado a sequência de Fibonacci, onde se inicia por 0 e 1 e o próximo valor sempre será a soma dos 2 valores anteriores (exemplo: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...), escreva um programa na linguagem que desejar onde, informado um número, ele calcule a sequência de Fibonacci e retorne uma mensagem avisando se o número informado pertence ou não a sequência.

IMPORTANTE: Esse número pode ser informado através de qualquer entrada de sua preferência ou pode ser previamente definido no código;

```java
public class pergunta02 {

    public static void main(String[] args) {
        int informedNumber = 13;

        if (isFibonacci(informedNumber)) {
            System.out.println(informedNumber + " is present in Fibonacci");
        } else {
            System.out.println(informedNumber + " isn't present in Fibonacci");
        }
    }

    public static boolean isFibonacci(int w) {
        long x1 = 5L * w * w + 4;
        long x2 = 5L * w * w - 4;

        long x1Sqrt = (long) Math.sqrt(x1);
        long x2Sqrt = (long) Math.sqrt(x2);

        return (x1Sqrt * x1Sqrt == x1) || (x2Sqrt * x2Sqrt == x2);
    }
}
```
