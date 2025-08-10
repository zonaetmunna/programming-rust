// Scalar and Compound Types example for Rust
fn main() {
    // Scalar Types
    let a: i32 = -10;
    let b: u32 = 10;
    let c: f64 = 3.14;
    let d: char = 'z';
    let e: bool = true;
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);

    // Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!(
        "tup = ({}, {}, {}), destructured: x = {}, y = {}, z = {}",
        tup.0, tup.1, tup.2, x, y, z
    );
    let arr = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);
}
