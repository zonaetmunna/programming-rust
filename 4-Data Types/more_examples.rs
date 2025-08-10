// More examples for Data Types in Rust
fn main() {
    // Scalar types
    let i: i64 = -1000;
    let u: usize = 1000;
    let f: f32 = 2.718;
    let c: char = 'R';
    let b: bool = false;
    println!("i = {}, u = {}, f = {}, c = {}, b = {}", i, u, f, c, b);

    // Compound types
    let tuple = (1, 'a', 3.14);
    println!("tuple = ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    let array: [i32; 3] = [10, 20, 30];
    for val in array.iter() {
        println!("array element: {}", val);
    }
}
