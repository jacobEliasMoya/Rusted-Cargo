// use std::{fmt::Result,io::Result as IoResult};

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     pub use crate::front_of_house::hosting as foh_hosting;

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// fn deliver_order() {}

// mod back_of_house {

//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }

//     pub struct Breakfast {
//         pub toast: String,
//         pub seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// mod customer {
//     use crate::front_of_house::hosting::add_to_waitlist;

//     pub fn eat_at_restaurant() {
//         // let order1 = back_of_house::Appetizer::Soup;
//         // let order2 = back_of_house::Appetizer::Salad;
//         add_to_waitlist();
//     }
// }

// // fn function1() -> Result {

// // }

// // fn function()-> IoResult<()>{

// // }
