// Exercise 1: Variables and Mutability
pub fn exercise_1() {
    // Declare an immutable variable x with the value 10
    let x = 10;
    // Attempt to reassign x to 20 and observe the compiler error
    // x = 20; // Uncommenting this line will cause a compile-time error

    // Declare a mutable variable y with the value 15
    let mut y = 15;
    // Reassign y to 25 and print its value
    y = 25;
    println!("Mutable variable y: {}", y);
}

// Exercise 2: Ownership and Borrowing
fn print_string(s: &String) {
    println!("{}", s);
}

pub fn exercise_2() {
    // Create a String with the value "hello" and assign it to s1
    let s1 = String::from("hello");
    // Transfer ownership of s1 to s2 and print s2
    let s2 = s1;
    println!("Ownership moved to s2: {}", s2);

    // Attempt to print s1 after transferring ownership and observe the compiler error
    // println!("{}", s1); // Uncommenting this line will cause a compile-time error

    // Call print_string with s2
    print_string(&s2);
}
