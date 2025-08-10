// More examples for Control Flow in Rust
fn main() {
    let n = 2;
    match n {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        _ => println!("other"),
    }

    let mut i = 0;
    loop {
        if i >= 2 {
            break;
        }
        println!("loop i = {}", i);
        i += 1;
    }
}
