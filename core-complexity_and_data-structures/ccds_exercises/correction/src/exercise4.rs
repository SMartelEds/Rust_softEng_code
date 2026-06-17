fn find_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
    //use iterator to find target with the position method then process index to return Option<usize>
    vec.iter().enumerate().find_map(
        |(index, &value)| {
            if value == target { Some(index) } else { None }
        },
    )
    // Alternatively, we could use the position method directly
    //vec.iter().position(|&x| x == target)
}
pub fn exercise_4() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", find_index(&numbers, 3)); // Expected: Some(2)
    println!("{:?}", find_index(&numbers, 6)); // Expected: None
}
