use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::io;
use crate::place::Place;
use crate::product::Product;
use crate::shop::Shop;
use crate::shopping_list::ShoppingList;
use crate::telegram;

#[derive(Debug)]
pub struct RouteCreator {
}


impl RouteCreator {
    pub fn create(order_of_walking: Vec<String>, shopping_list: ShoppingList, shop: Shop) -> Vec<String> {
        let (shopping_list2, missing_products) = shopping_list.match_to_shop(&shop);
        let mut route: Vec<String> = vec![];

        for place_name in order_of_walking {
            for product in shopping_list2.iter() {
                let place_opt = shop.places.get(&place_name);
                if let Some(place) = place_opt {
                    if place.matching_product_exact(product) {
                        let message = format!("{}\n{} {} {}", &place_name, &product.name, &product.comment, &product.number);
                        route.push(message);
                    }
                }
            }
        }
        for product in &missing_products {
            let message = format!("Not in the list\n{} {} {}", &product.name, &product.comment, &product.number);
            route.push(message);
        }
        route
    }

    pub fn send_route_to_telegram(route: Vec<String>) {
        telegram::send_multiple_messages(route);
    }
}

pub fn map_products_to_places(shopping_list: ShoppingList, shop: &Shop) -> (HashMap<Product, String>, Vec<String>){
    let mut result: HashMap<Product, String> = HashMap::new();
    let mut not_in_list: Vec<String> = vec![];
    // for product in shopping_list.iter() {
    //     let mut found = false;
    //     for (place_name, place) in &shop.places {
    //         let matching_prods = place.matching_product(product);
    //         if matching_prods.len() == 0 {continue}
    //         else if matching_prods.len() == 1 {
    //             result.insert(product.clone(), place_name.to_owned());
    //             found = true;
    //             continue;
    //         } else {
    //             let picked_product = pick_product(matching_prods, product);
    //             result.insert( picked_product.clone(), place_name.to_owned());
    //             found = true;
    //             continue;
    //         }
    //     }
    //     if !found {
    //         not_in_list.push(product.name.to_owned());
    //     }
    // }

    (result, not_in_list)
}