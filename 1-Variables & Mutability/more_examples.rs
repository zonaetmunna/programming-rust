// More examples for Variables & Mutability in Rust
fn main() {
    // Immutable variable
    let a = 42;
    println!("a = {}", a);
    // Mutable variable
    let mut b = 7;
    println!("b before = {}", b);
    b += 5;
    println!("b after = {}", b);
    // Shadowing
    let c = 10;
    let c = c * 2;
    println!("c (shadowed) = {}", c);
}
