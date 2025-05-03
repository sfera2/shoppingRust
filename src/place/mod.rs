use std::collections::HashSet;
use crate::product::Product;

#[derive(Debug)]
pub struct Place {
    pub products: HashSet<String>
}

impl Place {
    pub fn new() -> Place {
        Place {
            products: HashSet::new(),
        }
    }

    pub fn add(&mut self, product: String) {
        self.products.insert(product.to_lowercase());
    }

    pub fn matching_product(&self, product: &Product) -> Vec<String> {
        let temp = self.products.clone();
        temp.into_iter().filter(|x| x.contains(&product.name)).collect::<Vec<String>>()
    }

    pub fn matching_product_exact(&self, product: &Product) -> bool {
        self.products.contains(&product.name)
    }
}
