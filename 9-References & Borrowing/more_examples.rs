// More examples for References & Borrowing in Rust
fn print_length(s: &String) {
    println!("Length is {}", s.len());
}

fn main() {
    let s = String::from("borrowed");
    print_length(&s);
    println!("s is still valid: {}", s);
}
