// Dangling References example for Rust (will not compile)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// let reference_to_nothing = dangle();

fn main() {
    println!("Rust prevents dangling references at compile time.");
}
