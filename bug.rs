fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;  // z is an immutable reference to x

    *y += 1; // Modifying x through y is fine
    println!("x = {}", x); // x is now 6

    // This line will cause a compile-time error because the immutable reference z is still active
    *y += 1; // error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
}