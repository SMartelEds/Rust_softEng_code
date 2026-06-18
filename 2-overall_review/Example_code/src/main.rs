use core::num;
use std::num::FpCategory::Nan;

enum weirdResult {
    numericInt(i32),
    numerciFloat(f64),
}

#[derive(Eq, PartialEq)]
struct Point<E> {
    x: E,
    y: E,
}

impl<E> Point<E> {
    pub fn distance(&self, other: &Point<E>) -> &E {
        // I know it's not a distance function but return what you need
        //other.x -self.x + self.y- other.y
        &self.y
    }
}

fn main() {
    println!("Hello, world!");
    let test = get_first_value_from_vec(&vec![0, 2, 3]);

    let t1 = test.unwrap_or(0);

    match test {
        Some(value) => println!("{}", value), //do something with value
        None => println!("I have no value"),  //do something without value
    }

    let p: Point<i32> = Point { x: 1, y: 3 };
    let p: Point<f64> = Point { x: 1f64, y: 3f64 };
}

fn get_first_value_from_vec(t: &Vec<u8>) -> Option<u8> {
    if t.len() >= 1 { Some(t[0]) } else { None }
}

fn divide_by2(x: i32) -> weirdResult {
    if (x % 2 == 0) {
        return weirdResult::numericInt((x / 2) as i32);
    }
    weirdResult::numerciFloat((x / 2) as f64)
}
