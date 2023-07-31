mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::{self, hosting};
    use crate::back_of_house;

    pub fn eat_at_restaurant() {
        // Absolute
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative
        front_of_house::hosting::add_to_waitlist();

        // shorthand
        hosting::add_to_waitlist();

        // Breakfast in the summer with rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");

        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}
