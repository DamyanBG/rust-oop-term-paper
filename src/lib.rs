pub trait Item {
    fn get_name(&self) -> &String;

    fn get_caloric_value(&self) -> f64;
}

#[derive(Clone)]
pub struct Pizza {
    name: String,
    ingredients: Vec<Ingredient>,
    state: String,
    price: u16,
}

impl Pizza {
    fn new(name: &String, ingredients: Vec<Ingredient>, price: u16) -> Pizza {
        Pizza {
            name: String::from(name),
            ingredients,
            state: String::from("Non ordered"),
            price,
        }
    }

    fn get_price(&self) -> u16 {
        self.price
    }

    fn order_pizza(&mut self) {
        self.state = String::from("Ordered");
    }

    fn cook_pizza(&mut self) {
        self.state = String::from("Cooking");
    }

    fn serve_pizza(&mut self) {
        self.state = String::from("Seved");
    }
}

impl Item for Pizza {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_caloric_value(&self) -> f64 {
        let mut total_value = 0.0;

        for ingredient in &self.ingredients {
            let caloric_value = (ingredient.caloric_value as f64) * (ingredient.weight as f64) / 100.0;
            total_value += caloric_value;
        };

        total_value
    }
}

pub struct Product {
    pub name: String,
    pub caloric_value: u16,
}

impl Product {
    pub fn new(name: &str, caloric_value: u16) -> Self {
        Self {
            name: String::from(name),
            caloric_value,
        }
    }
}

#[derive(Clone)]
pub struct Ingredient {
    pub name: String,
    pub weight: u16,
    pub caloric_value: u16,
}

impl Ingredient {
    pub fn new_from_product(product: &Product, weight: u16) -> Self {
        Self {
            name: String::from(&product.name),
            caloric_value: product.caloric_value,
            weight,
        }
    }
}

#[derive(Clone)]
pub struct Drink {
    pub name: String,
    status: String,
    price: u16,
    caloric_value: u16,
}

impl Drink {
    pub fn new(name: &str, price: u16, caloric_value: u16) -> Drink {
        Drink { name: String::from(name), status: String::from("Non ordered"), price, caloric_value }
    }

    pub fn order_drink(&mut self) {
        self.status = String::from("Ordered");
    }

    pub fn serve_drink(&mut self) {
        self.status = String::from("Served");
    }

    pub fn get_price(&self) -> u16 {
        self.price
    }
}

impl Item for Drink {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_caloric_value(&self) -> f64 {
        self.caloric_value as f64
    }
}

pub struct Menu {
    pub pizzas: Vec<Pizza>,
    pub drinks: Vec<Drink>,
}

impl Menu {
    fn list_items_names<T: Item>(&self, items: &Vec<T>, message: &str) {
        println!("{}", message);
        for (index, item) in items.iter().enumerate() {
            println!("{}. {} with energy {}", index + 1, item.get_name(), item.get_caloric_value());
        }
    }

    pub fn list_menu(&self) {
        self.list_items_names(&self.drinks, "Drinks:");
        self.list_items_names(&self.pizzas, "Pizzas:");
    }
}

pub struct MenuBuilder {
    pizzas: Vec<Pizza>,
    drinks: Vec<Drink>,
}

impl MenuBuilder {
    pub fn new() -> Self {
        Self {
            pizzas: Vec::new(),
            drinks: Vec::new(),
        }
    }

    pub fn add_pizza(&mut self, pizza: Pizza) -> &mut Self {
        self.pizzas.push(pizza);
        self
    }

    pub fn add_drink(&mut self, drink: Drink) -> &mut Self {
        self.drinks.push(drink);
        self
    }

    pub fn build_menu(&mut self) -> Menu {
        Menu {
            pizzas: self.pizzas.clone(),
            drinks: self.drinks.clone(),
        }
    }
}

pub struct PizzaBuilder {
    name: String,
    ingredients: Vec<Ingredient>,
    price: u16,
}

impl PizzaBuilder {
    pub fn new(name: &str, price: u16) -> Self {
        Self {
            name: String::from(name),
            ingredients: Vec::new(),
            price,
        }
    }

    pub fn add_ingredient(&mut self, product: &Product, weight: u16) -> &mut Self {
        self.ingredients.push(Ingredient::new_from_product(product, weight));
        self
    }

    pub fn build_pizza(&self) -> Pizza {
        Pizza::new(&self.name, self.ingredients.clone(), self.price)
    }
}

struct PizzaLineItem {
    pizza: Pizza,
    quantity: u8,
}

impl PizzaLineItem {
    fn new(pizza: Pizza, quantity: u8) -> Self {
        Self {
            pizza,
            quantity
        }
    }

    fn get_quantity(&self) -> u8 {
        self.quantity
    }
}

pub struct DrinkLineItem {
    drink: Drink,
    quantity: u8,
}

impl DrinkLineItem {
    fn new(drink: Drink, quantity: u8) -> Self {
        Self {
            drink,
            quantity,
        }
    }

    fn get_quantity(&self) -> u8 {
        self.quantity
    }
}

pub struct Order {
    menu: Menu,
    pizza_line_items: Vec<PizzaLineItem>,
    drink_line_items: Vec<DrinkLineItem>,
    status: String
}

impl Order {
    pub fn new(menu: Menu) -> Self {
        Self {
            menu,
            pizza_line_items: Vec::<PizzaLineItem>::new(),
            drink_line_items: Vec::<DrinkLineItem>::new(),
            status: String::from("creating"),
        }
    }

    pub fn order_pizza(&mut self, pizza_number: usize, quantity: u8) {
        let pizza_index = pizza_number - 1;
        let pizza = self.menu.pizzas.get(pizza_index).unwrap();
        let new_pizza = pizza.clone();
        let pizza_line_item = PizzaLineItem::new(new_pizza, quantity);
        self.pizza_line_items.push(pizza_line_item);
    }

    pub fn order_drink(&mut self, drink_number: usize, quantity: u8) {
        let drink_index = drink_number - 1;
        let drink = self.menu.drinks.get(drink_index).unwrap();
        let new_drink = drink.clone();
        let drink_line_item = DrinkLineItem::new(new_drink, quantity);
        self.drink_line_items.push(drink_line_item);
    }

    pub fn finish_order(&mut self) {
        let mut ordered_drinks = Vec::<String>::new();
        let mut ordered_pizzas = Vec::<String>::new();


        for pizza_line_item in &mut self.pizza_line_items {
            pizza_line_item.pizza.order_pizza();
            ordered_pizzas.push(String::from(&pizza_line_item.pizza.name))
        }

        for drink_line_item in &mut self.drink_line_items {
            drink_line_item.drink.order_drink();
            ordered_drinks.push(String::from(&drink_line_item.drink.name))
        }

        self.status = String::from("Ordered");

        println!("You ordered {:?} and {:?}", ordered_drinks, ordered_pizzas);
    }

    pub fn serve_drinks(&mut self) {
        for drink_line_item in &mut self.drink_line_items {
            drink_line_item.drink.serve_drink();
        }

        println!("The drinks are served!")
    }

    pub fn cook_pizzas(&mut self) {
        for pizza_line_item in &mut self.pizza_line_items {
            pizza_line_item.pizza.cook_pizza();
        }

        println!("The kitchen is preparing the pizzas!")
    }

    pub fn serve_pizzas(&mut self) {
        for pizza_line_item in &mut self.pizza_line_items {
            pizza_line_item.pizza.serve_pizza();
        }

        println!("The pizzas are served!")
    }

    pub fn checkout_order(&mut self) {
        let mut total = 0;

        for pizza_line_item in &mut self.pizza_line_items {
            let price = pizza_line_item.pizza.get_price();
            let quantity = pizza_line_item.get_quantity();
            let sub_total = price + u16::from(quantity);
            total += sub_total;
        }

        for drink_line_item in &mut self.drink_line_items {
            let price = drink_line_item.drink.get_price();
            let quantity = drink_line_item.get_quantity();
            let sub_total = price + u16::from(quantity);
            total += sub_total;
        }

        println!("Your bill is {} leva.", total);
    }

    pub fn pay_order(&mut self) {
        self.status = String::from("Paid");

        println!("Your bill is paid, thank and welcome again!");
    }

}