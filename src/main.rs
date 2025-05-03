use std::collections::HashMap;
use std::env;
use zakupy_rust::shopping_list::ShoppingList;
use zakupy_rust::route_creator::RouteCreator;
use zakupy_rust::shop::Shop;

fn main()  {
    let args: Vec<String> = env::args().collect();

    let mut shop_name: String;
    if (args.len() == 1) {
        shop_name = format!("lidl");
    } else {
        shop_name = args[1].to_owned();
    }

    let shop_mapping: HashMap<String, (String, String)> = HashMap::from(
    [
            (format!("mercadona"), (format!("./shops/mercadona_rambla.txt"), format!("./shops/mercadona_rambla_walking.txt"))),
            (format!("lidl"), (format!("./shops/lidl_wielicka.json"), format!("./shops/lidl_wielicka_walking.txt"))),
            (format!("carrefour"), (format!("./shops/carrefour_glories2.json"), format!("./shops/carrefour_glories_walking.txt")))
        ]
    );

    ShoppingList::get();
    let shopping_list = ShoppingList::load();
    let mut shop = Shop::new();
    shop.load_from_file(&shop_mapping.get(&shop_name).unwrap().0);
    let order_of_walking = Shop::order_of_walking(&shop_mapping.get(&shop_name).unwrap().1);

    let route = RouteCreator::create(order_of_walking, shopping_list, shop);
    RouteCreator::send_route_to_telegram(route);

    // jest bug taki, że jak dałem najpierw comment, a potem liczbę, to mi ustawi liczbę na 1. shopping_list/mod.rs
}
