#[derive(Debug)]
pub struct Product {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
}

impl Product {
    pub fn new(name: String, price: f64, description: Option<String>) -> Product {
        Product {
            name,
            price,
            description,
        }
    }
}
