struct Product{
    name: String,
    price: f64,
    stock: i32,
}

impl Product {
    fn new(name: String, price: f64) -> Self{
        Self {
            name,
            price,
            stock: 0,
        }
    }

    fn apply_discount(&mut self, discount: &f64)
}