fn swap<T>(a: T, b: T) -> (T, T) {
    // just return a tuple with swapped values
    (b, a)
}
pub fn exercise_3() {
    let (a, b) = swap(5, 10);
    let (sa, sb) = swap(String::from("hello"), String::from("world"));
    println!("a: {}, b: {}", a, b); // Expected: a: 10, b: 5
}
