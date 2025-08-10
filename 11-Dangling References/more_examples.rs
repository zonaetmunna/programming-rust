// More examples for Dangling References in Rust
// Example 1: Attempting to return a reference to a local variable (will not compile)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// let reference_to_nothing = dangle();

// Example 2: Correct way - return the owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    println!("Rust prevents dangling references at compile time.");
    let s = no_dangle();
    println!("Returned owned String: {}", s);
}
