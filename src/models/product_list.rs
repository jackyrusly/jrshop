use super::product::Product;
use crate::format;
use std::io::{self, Write};

pub struct ProductList {
    data: Vec<Product>,
}

impl ProductList {
    pub fn new() -> ProductList {
        ProductList { data: Vec::new() }
    }

    pub fn create_product(&mut self) {
        let mut name = String::new();
        let mut price = String::new();
        let mut description = String::new();
        let product_price: f64;
        let product_description: Option<String>;

        loop {
            name.clear();
            print!("\nInput product name: ");
            io::stdout().flush().expect("Could not flush stdout");

            match io::stdin().read_line(&mut name) {
                Ok(_) => {
                    if name.trim().chars().count() < 5 {
                        println!("Name too short (Minimal 5)");
                    } else if name.trim().chars().count() > 20 {
                        println!("Name too long (Maximum 20)");
                    } else {
                        break;
                    }
                }
                Err(error) => println!("Error: {}", error),
            }
        }

        loop {
            price.clear();
            print!("\nInput product price: ");
            io::stdout().flush().expect("Could not flush stdout");

            match io::stdin().read_line(&mut price) {
                Ok(_) => match price.trim().parse::<f64>() {
                    Ok(price_parsed) => {
                        product_price = price_parsed;

                        break;
                    }
                    Err(error) => println!("Error: {}", error),
                },
                Err(error) => println!("Error: {}", error),
            }
        }

        loop {
            description.clear();
            print!("\nInput product description (Optional): ");
            io::stdout().flush().expect("Could not flush stdout");

            match io::stdin().read_line(&mut description) {
                Ok(_) => {
                    if description.trim().is_empty() {
                        product_description = None;
                    } else {
                        product_description = Some(description.trim().to_string());
                    }

                    break;
                }
                Err(error) => println!("Error: {}", error),
            }
        }

        format::trim_newline(&mut name);

        self.data
            .push(Product::new(name, product_price, product_description));
    }

    pub fn print(&self) {
        format::clear();

        println!("{:#?}", self.data);
        format::pause();
    }
}
