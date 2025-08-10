// More examples for Ownership in Rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
                 // println!("{}", s1); // This would not compile
    println!("s2 = {}", s2);

    let x = 5;
    let y = x; // i32 is Copy, so x is still valid
    println!("x = {}, y = {}", x, y);
}
