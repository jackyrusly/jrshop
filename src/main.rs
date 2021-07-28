mod helpers;
mod models;

use helpers::format;
use models::product_list::ProductList;
use std::io::{self, Write};

fn print_menu() {
    format::clear();
    println!("Menu");
    println!("====");
    println!("1. Create New Product");
    println!("2. Product List");
    println!("3. Exit");
    print!("Choose: ");
    io::stdout().flush().expect("Could not flush stdout");
}

fn main() {
    let mut product_list = ProductList::new();

    loop {
        let mut input = String::new();

        print_menu();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.as_str().trim() {
                "1" => product_list.create_product(),
                "2" => product_list.print(),
                "3" => break,
                _ => {
                    println!("Invalid menu!");
                }
            },
            Err(error) => {
                println!("Something went wrong: {}", error);
            }
        }
    }
}
