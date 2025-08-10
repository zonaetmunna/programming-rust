// More examples for Shadowing in Rust
fn main() {
    let x = 8;
    let x = x + 2; // shadowed
    println!("x after shadowing: {}", x);
    {
        let x = x * 3; // shadowed in inner scope
        println!("x in inner scope: {}", x);
    }
    println!("x after inner scope: {}", x);
}
