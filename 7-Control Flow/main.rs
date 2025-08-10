// Control Flow example for Rust
fn main() {
    let number = 7;
    if number < 5 {
        println!("less than five");
    } else {
        println!("five or more");
    }

    for i in 1..4 {
        println!("i = {}", i);
    }

    let mut count = 0;
    while count < 3 {
        println!("count = {}", count);
        count += 1;
    }
}
