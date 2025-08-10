// More examples for Mutable References in Rust
fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("hi");
    append_exclamation(&mut s);
    println!("{}", s);
}
