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
