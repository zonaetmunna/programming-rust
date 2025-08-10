// Ownership Rules example for Rust
fn main() {
    {
        let s = String::from("hello");
        println!("{}", s);
    } // s is dropped here
}
