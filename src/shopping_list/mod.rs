use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::Command;
use crate::product::Product;
use crate::shop::Shop;

const SHOPPING_LIST: &str = "lists/shopping_list.txt";

fn user_input() -> usize {
    // return 2;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num: usize = input.trim().parse().expect("Please type a number!");
    num
}

pub fn pick_product(matching_products: Vec<String>, product: &Product) -> Product {
    if matching_products.len() <= 1 {
        Product{name:matching_products[0].to_owned(), number: product.number, comment: product.comment.to_owned()}
    } else {
        println!("Which - {:?} - pick number", product.name);
        for (i, prod) in matching_products.iter().enumerate() {
            println!("{:?}. {:?}", i+1, prod);
        }
        let ind = user_input();
        Product{name:matching_products[ind-1].to_owned(), number: product.number, comment: product.comment.to_owned()}
    }
}

pub fn list_matching_products(shop: &Shop, product: &Product) -> Vec<String> {
    shop.places.iter().
        map(|x| x.1).
        filter(|&x| x.matching_product(product).len() > 0).
        map(|x| x.matching_product(product)).
        flatten().
        collect::<Vec<String>>()
}

#[derive(Debug, Clone)]
pub struct ShoppingList {
    pub products: HashSet<Product>
}

impl ShoppingList {
    pub fn get() {
        let mut child = Command::new("xed")
            .arg(SHOPPING_LIST)
            .spawn()
            .expect("failed to execute process");

        // Wait for the 'xed' process to finish
        let _ = child.wait().expect("failed to wait on child");
    }

    pub fn load() -> ShoppingList {
        let mut file = File::open(SHOPPING_LIST).unwrap();

        // Read the file contents into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut products = HashSet::new();
        for line in contents.trim().split("\n") {
            let product_info = line.split(";").map(|x| x.trim().to_string()).collect::<Vec<String>>();
            let mut product = Product::new();
            let name = product_info[0].clone();
            let mut number = match product_info.get(1) {
                Some(value) => {
                    match i32::from_str_radix(value.as_str(), 10) {
                        Ok(number) => number,
                        Err(_) => -1,
                    }
                },
                None => 1
            };
            // let number = u32::from_str_radix(product_info.get(1).as_str(), 10).expect("Not a number");
            let mut comment = match product_info.get(2) {
                Some(value) => value.clone(),
                None => "".to_string()
            };
            if number == -1 {
                comment = match product_info.get(1) {
                    Some(value) => value.clone(),
                    None => "".to_string()
                };
                number = 1;
            }
            product.set_name(name);
            product.set_comment(comment);
            product.set_number(number);
            products.insert(product);
        }
        return ShoppingList {products };
    }

    pub fn edit() {
        open::that(SHOPPING_LIST).unwrap();
    }
    pub fn match_to_shop(&self, shop: &Shop) -> (ShoppingList, HashSet<Product>) {
        let mut new_products: HashSet<Product> = HashSet::new();
        let mut missing_products: HashSet<Product> = HashSet::new();
        for prod in &self.products {
            let matching_list = list_matching_products(shop, prod);
            if matching_list.len() == 0 {
                missing_products.insert(prod.to_owned());
            } else {
                let prod = pick_product(matching_list, prod);
                new_products.insert(prod);
            }
        }

        (ShoppingList{products: new_products}, missing_products)
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<Product> {
        self.products.iter()
    }
}