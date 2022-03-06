mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        pub mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }

            fn cook_order() {}

            // По умолчанию является приватными, если не указано pub
            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

            // По умолчанию варианты перечислений являются публичными
            pub enum Appetizer {
                Soup,
                Salad,
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Абсолютный путь берет своё начало с корня крейта: названия крейта или ключевого слова crate
    crate::front_of_house::hosting::add_to_waitlist();

    // Относительный путь начинается с текущего модуля и использует ключевые слова self, super или идентификатор в текущем модуле
    front_of_house::hosting::add_to_waitlist();
    self::front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = front_of_house::serving::back_of_house::Appetizer::Soup;
    let _order2 = front_of_house::serving::back_of_house::Appetizer::Salad;
}