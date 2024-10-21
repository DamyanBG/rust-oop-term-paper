use rust_oop_article::{Drink, MenuBuilder, Order, PizzaBuilder, Product};

fn main() {
    let tomato_prod = Product::new("tomato", 18);
    let mozzarella_prod = Product::new("mozzarella", 280);
    let olive_oil = Product::new("olive_oil", 884);
    let basil = Product::new("basil", 20);
    let pepperoni_prod = Product::new("pepperoni", 500);
    let mushrooms = Product::new("mushrooms", 20);
    let meat = Product::new("meat", 350);
    let onion = Product::new("onion", 30);

    let capricciosa = PizzaBuilder::new("Margherita", 20)
        .add_ingredient(&tomato_prod, 200)
        .add_ingredient(&mushrooms, 100)
        .add_ingredient(&olive_oil, 80)
        .add_ingredient(&onion, 60)
        .build_pizza();

    let quattro_stagioni = PizzaBuilder::new("Quattro_stagioni", 22)
        .add_ingredient(&tomato_prod, 200)
        .add_ingredient(&mozzarella_prod, 100)
        .add_ingredient(&olive_oil, 80)
        .add_ingredient(&mushrooms, 100)
        .add_ingredient(&pepperoni_prod, 80)
        .add_ingredient(&basil, 20)
        .build_pizza();
    
    let diavola = PizzaBuilder::new("Diavola", 25)
        .add_ingredient(&tomato_prod, 200)
        .add_ingredient(&mozzarella_prod, 100)
        .add_ingredient(&olive_oil, 80)
        .add_ingredient(&pepperoni_prod, 150)
        .add_ingredient(&meat, 100)
        .build_pizza();
    
    let vegetariana = PizzaBuilder::new("Vegetariana", 18)
        .add_ingredient(&tomato_prod, 200)
        .add_ingredient(&mozzarella_prod, 100)
        .add_ingredient(&olive_oil, 80)
        .add_ingredient(&mushrooms, 100)
        .add_ingredient(&basil, 20)
        .build_pizza();

    let menu = MenuBuilder::new()
        .add_drink(Drink::new("Coca-Cola", 2, 400))
        .add_drink(Drink::new("Fanta", 2, 400))
        .add_drink(Drink::new("Pepsi", 1, 380))
        .add_drink(Drink::new("Beer", 3, 320))
        .add_drink(Drink::new("Rakia", 5, 150))
        .add_drink(Drink::new("Whiskey", 7, 120))
        .add_pizza(capricciosa)
        .add_pizza(quattro_stagioni)
        .add_pizza(diavola)
        .add_pizza(vegetariana)
        .build_menu();

    menu.list_menu();

    let mut order = Order::new(menu);
    
    order.order_pizza(1, 2);
    order.order_drink(2, 2);
    order.order_drink(5, 2);

    order.order_pizza(4, 1);
    order.order_drink(1, 1);

    order.finish_order();

    order.serve_drinks();
    order.cook_pizzas();

    order.serve_pizzas();

    order.checkout_order();
    order.pay_order();
    
}
