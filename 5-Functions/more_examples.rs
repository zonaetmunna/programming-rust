// More examples for Functions in Rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    greet("Rustacean");
    let result = multiply(6, 7);
    println!("6 * 7 = {}", result);
}
