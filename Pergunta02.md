# Pergunta 02

Dado a sequência de Fibonacci, onde se inicia por 0 e 1 e o próximo valor sempre será a soma dos 2 valores anteriores (exemplo: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34...), escreva um programa na linguagem que desejar onde, informado um número, ele calcule a sequência de Fibonacci e retorne uma mensagem avisando se o número informado pertence ou não a sequência.

IMPORTANTE: Esse número pode ser informado através de qualquer entrada de sua preferência ou pode ser previamente definido no código;

```rust
fn main() {
    let informed_number = 13;

    if is_fibonacci(informed_number) {
        println!("{informed_number} is present in Fibonnaci");
    } else {
        println!("{informed_number} isn't present in Fibonnaci");
    }
}

fn is_fibonacci(w: i32) -> bool {
    let x1 = 5 * w.pow(2) + 4;
    let x2 = 5 * w.pow(2) - 4;

    let x1_sqrt = (x1 as f64).sqrt() as i64;
    let x2_sqrt = (x2 as f64).sqrt() as i64;

    x1_sqrt * x1_sqrt == x1 as i64 || x2_sqrt * x2_sqrt == x2 as i64
}
```
