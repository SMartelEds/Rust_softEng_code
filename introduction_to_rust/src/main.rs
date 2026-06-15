use std::{cell::RefCell, default};

fn main() {
    println!("Hello, world!");

    //store in the stack
    let _x: i32 = 5;
    let mut _y: i32 = 5;
    //explicitely typed (goes to the stack)
    let _t: (i32, i32) = (1, 2);
    test();

    //Heap with string
    let text = String::from("Hello world");
    println!("{}", text);

    // not possible (immutable by default) _x=6;
    _y = 6;
    ownership_test();
    borrowing_example();
    string_example();
    array_example();
    vector_example();
    example_enum();
    exmple_raw_pointer();
    smart_pointer();
}

fn test() {
    let _y = 5;
    println!("{}", _y);
}

fn ownership_test() {
    let s1: String = String::from("hello"); // ownership to s1
    println!("{}", s1);
    let s2: String = s1; //Transfert the ownership to s2
    // lead to an error because s1 has no value println!("{}", s1);
    let s1: String = String::from("hello2");

    //You can copy value so you will set another part of memory with same value
    let s3 = s2.clone();
    println!("{}", s1);
    println!("{}", s3);

    //I want s4 getting access to value of s1
}

fn borrowing_example() {
    println!("Example Borrowing :");
    let mut s1: String = String::from("Hello");
    println!("Len of {} : {}", s1, calculate_length(&s1));
    change_string(&mut s1);
    println!("Len of {} : {}", s1, calculate_length(&s1));
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

/*fn example_if_return() {
    let x = 5;

    let y: i32 = if x < 5 { -1 } else { 1 };
}*/

fn string_example() {
    println!("Example String");
    let s = "hello";

    for c in s.chars() {
        println!("char : {}", c);
    }

    println!("Substring : {}", &s[1..5]);
}

fn array_example() {
    println!("Array Example");

    let my_arr: [i32; 5] = [1, 2, 3, 4, 5];

    print_array_element(&my_arr, 0); //Display 1
    //print_array_element(&my_arr, 5); //Out of Boun
}

fn print_array_element(arr: &[i32], idx: usize) {
    println!("Value at position {} : {}", idx, arr[idx]);
}

fn vector_example() {
    println!("Vector Example");
    let mut v = vec![0, 1, 3, 4];
    println!("Vector Init");

    for n in &v {
        println!("Element : {}", n);
    }

    v.push(6);
    println!("Vector Push");

    for n in &v {
        println!("Element : {}", n);
    }
    v.pop();
    println!("Vector Pop");

    for n in &v {
        println!("Element : {}", n);
    }
}

enum Direction {
    UP,
    DOWN,
}

fn example_enum() {
    let t = Direction::UP;

    match t {
        Direction::UP => println!("Go UP"),
        _default => println!("Go Down"),
    }
}

fn exmple_raw_pointer() {
    println!("Raw Pointers Example");
    let mut z = 30;
    println!("Value of Z : {}", z);
    let raw_z = &mut z as *mut i32;

    unsafe {
        // raw_z = 0x1457; If I modify directly raw z I modify the address
        *raw_z = 40;
    }

    println!("Value of Z : {}", z);
}

fn smart_pointer() -> [i32; 100000000000] {
    let b: Box<[i32; 100000000000]> = Box::new([0; 100000000000]);
    let c: [i32; 100000000000] = [0; 100000000000];

    //Smart pointer inner mutability
    let r: RefCell<i32> = RefCell::new(10);
    *r.borrow_mut() += 5;

    print!("My smart Pointer : {}", r.borrow());
    c
}
