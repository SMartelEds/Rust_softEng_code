// Exercise 5: Result
fn safe_sqrt(num: f64) -> Result<f64, String> {
    // Check if the number is negative
    if num < 0.0 {
        //If it rise an error with a message through Err variant
        Err(String::from(
            "Cannot compute square root of a negative number",
        ))
    } else {
        //else compute the square root and return it through Ok variant
        Ok(num.sqrt())
    }
}

pub fn exercise_5() {
    println!("{:?}", safe_sqrt(16.0)); // Expected: Ok(4.0)
    println!("{:?}", safe_sqrt(-1.0)); // Expected: Err("Cannot compute square root of a negative number")
}
