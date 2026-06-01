fn main() {
    paths_demo::eat_at_restaurant();

    let mut meal = paths_demo::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = paths_demo::Appetizer::Soup;
    let _order2 = paths_demo::Appetizer::Salad;
}

