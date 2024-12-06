fn main() {
    let mut x = 5;
    { //Creating a new scope.
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modifying x through y is fine
        println!("x = {}", x); // x is now 6
    } //The mutable reference goes out of scope here.

    let z = &x;  // z is an immutable reference to x
    println!("x = {}", x); //Prints 6
}