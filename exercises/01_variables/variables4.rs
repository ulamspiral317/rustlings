// TODO: Fix the compiler error.
fn main() {
    // Make `x` mutable so it can be changed.
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
