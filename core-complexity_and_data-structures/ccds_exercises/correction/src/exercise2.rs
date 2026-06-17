// Exercise 2: Structs
struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    // Constructor style function
    fn new(name: String, price: f64, stock: u32) -> Self {
        Self { name, price, stock }
    }

    // Method to apply a discount use mutable reference to itself
    fn apply_discount(&mut self, discount: f64) {
        self.price *= 1.0 - discount;
    }
}

pub fn exercise_2() {
    let mut product = Product::new(String::from("Laptop"), 999.99, 10);
    product.apply_discount(0.1); // Apply 10% discount
    println!("Discounted price: {:.2}", product.price); // Expected: 899.99
}
