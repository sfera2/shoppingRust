use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use crate::place::Place;
use std::string::String;
use crate::utils;

#[derive(Debug)]
pub struct Shop {
    pub places: HashMap<String, Place>,
}

impl Shop {
    pub fn new() -> Shop {
        Shop {places: HashMap::new()}
    }

    pub fn add(&mut self, place_name: String, place: Place) {
        self.places.insert(place_name, place);
    }

    pub fn list_places(&self) -> Vec<&String> {
        self.places.keys().collect()
    }

    pub fn load_from_file(&mut self, json_file: &String) {
        let data = utils::read_json(json_file);
        if let Some(map) = data.as_object() {
            for (place_name, value) in map.into_iter() {
                let place_name_copy = place_name.clone();
                let mut place = Place {products: HashSet::new()};
                if let Some(products) = value.as_array() {
                    for product in products {
                        if let product_name = product.to_string() {
                            place.add(product_name.replace("\"", ""));
                        }
                    }
                }
                self.places.insert(place_name_copy, place);
            }
        }
    }

    pub fn order_of_walking(file: &str) -> Vec<String> {
        let mut file = File::open(file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let order = contents
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        order
    }
}
