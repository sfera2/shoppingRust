#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::fmt::format;
    use crate::{telegram, utils};
    use crate::place::Place;
    use crate::product::Product;
    use crate::route_creator::{map_products_to_places, RouteCreator};
    use crate::shop;
    use crate::shop::Shop;
    use crate::shopping_list::{list_matching_products, ShoppingList};
    use crate::utils::number;

    #[test]
    fn number_test() {
        assert_eq!(5, utils::number(2));
    }

    #[test]
    fn shop_init() {
        let s = shop::Shop::new();
        assert_eq!(0, s.list_places().len());
    }

    #[test]
    fn read_json() {
        let data = utils::read_json("./shops/carrefour_glories.json");
        if let Some(map) = data.as_object() {
            for (key, value) in map {
                if let Some(array) = value.as_array() {
                    for el in array {
                        println!("{:?}", el);
                    }
                }
            }
        }
        assert!(false);
    }

    #[test]
    fn load_shop() {
        let mut shop = Shop::new();
        shop.load_from_file("./shops/carrefour_glories2.json");
        println!("{:?}", shop);
        assert!(false);
    }

    #[test]
    fn load_shopping_list() {
        let shopping_list = ShoppingList::load();
        println!("{:?}", &shopping_list);
        let names: HashSet<String> = shopping_list.products.iter().map(|prod| prod.name.to_owned()).collect();
        assert_eq!(true, names.contains("kostki do wc"));
        assert_eq!(false, names.contains("kostki do wcccc"));
    }

    #[test]
    fn send_telegram_message() {
        // let x = telegram::send_single_message("Tekst".to_string());
        let messages = vec![format!("First message"), format!("Second message")];
        let x = telegram::send_multiple_messages(messages);
        // println!("{:?}", x);
        assert!(false);
    }

    #[test]
    fn order_of_walking() {
        let order_of_walking = Shop::order_of_walking("shops/carrefour_glories_walking.txt");
        println!("{:?}", order_of_walking);
        assert!(false);
    }

    #[test]
    fn route_creator() {
        let order_of_walking = Shop::order_of_walking("shops/carrefour_glories_walking.txt");
        let mut shop = Shop::new();
        shop.load_from_file("./shops/carrefour_glories2.json");
        let shopping_list = ShoppingList::load();
        let (l, nil) = map_products_to_places(shopping_list, &shop);
        let chusteczki: HashMap<&Product, &String>= l.iter().filter(|x| x.0.name == format!("chusteczki")).collect();
        let ziola: HashMap<&Product, &String>= l.iter().filter(|x| x.0.name == format!("zioła")).collect();
        let kostki_do_wc: HashMap<&Product, &String>= l.iter().filter(|x| x.0.name == format!("kostki do wc")).collect();
        let filety_losos: HashMap<&Product, &String>= l.iter().filter(|x| x.0.name == format!("filety z łososia")).collect();
        // RouteCreator::create(order_of_walking, shopping_list, shop);



        assert_eq!(**chusteczki.values().next().unwrap(), format!("L1"));
        assert_eq!(chusteczki.keys().next().unwrap().name, format!("chusteczki"));
        assert_eq!(chusteczki.keys().next().unwrap().number, 2);
        assert_eq!(chusteczki.keys().next().unwrap().comment, format!("zupełnie nie prowansalskie"));
    }

    #[test]
    fn matching_product() {
        let product1 = Product{name: format!("kefir"), number: 1, comment: format!("cos")};
        let product2 = Product{name: format!("kefiryt"), number: 1, comment: format!("cos")};
        let place = Place{products: HashSet::from([format!("kefiry"), format!("kefirek"), format!("costam")])};
        // RouteCreator::create(order_of_walking, shopping_list, shop);
        // assert_eq!(HashSet::from([format!("kefiry"), format!("kefirek")]), place.matching_product(&product1));
        // assert_eq!(HashSet::new(), place.matching_product(&product2));
    }

    #[test]
    fn test_list_matching_products() {
        let mut shop = Shop::new();
        shop.load_from_file("./shops/carrefour_glories2.json");

        let product1 = Product{name: format!("kefir"), number: 1, comment: format!("cos")};
        let product2 = Product{name: format!("kefiryt"), number: 1, comment: format!("cos")};
        let product3 = Product{name: format!("chusteczki"), number: 1, comment: format!("cos")};
        let product4 = Product{name: format!("cukinia"), number: 1, comment: format!("cos")};

        let correct_result1 = vec!["kefir".to_string()];
        let correct_result2: Vec<String> = vec![];
        let correct_result3 = vec!["chusteczki do prania".to_string(), "mokre chusteczki".to_string(), "chusteczki higieniczne".to_string(), "chusteczki higieniczne kwadratowe".to_string(), "chusteczki kwadratowe".to_string(), "chusteczki".to_string(), "chusteczki nawilżone".to_string()];
        let correct_result4 = vec!["cukinia".to_string()];
;
        let res1 = list_matching_products(&shop, &product1);
        let res2 = list_matching_products(&shop, &product2);
        let res3 = list_matching_products(&shop, &product3);
        let res4 = list_matching_products(&shop, &product4);

        assert_eq!(res1.into_iter().collect::<HashSet<String>>(), correct_result1.into_iter().collect::<HashSet<String>>());
        assert_eq!(res2.into_iter().collect::<HashSet<String>>(), correct_result2.into_iter().collect::<HashSet<String>>());
        assert_eq!(res3.into_iter().collect::<HashSet<String>>(), correct_result3.into_iter().collect::<HashSet<String>>());
        assert_eq!(res4.into_iter().collect::<HashSet<String>>(), correct_result4.into_iter().collect::<HashSet<String>>());

    }
    #[test]
    fn test_match_to_shop() {
        let mut shop = Shop::new();
        shop.load_from_file("./shops/carrefour_glories2.json");
        let shopping_list = ShoppingList::load();

        let (shopping_list2, missing_products) = shopping_list.match_to_shop(&shop);

        assert_eq!(shopping_list2.products.len(), 22);
        assert_eq!(missing_products.iter().next().unwrap().name, format!("nicnicnic"));
    }
}