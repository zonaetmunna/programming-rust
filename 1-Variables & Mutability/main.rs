fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    // make variable mutable with `mut`
    let mut y = 10;
    println!("y before = {}", y);
    y += 15;
    println!("y after  = {}", y);

    // shadowing: re-declaring a variable with the same name
    let x = x + 1;
    println!("shadowed x = {}", x);

    //
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // constants must specify type and be uppercase by conventions
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);

    // explicit type annotation
    let z: f64 = 3.0;
    println!("The value of z is: {}", z);
}
