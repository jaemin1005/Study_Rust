mod front_of_house;

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_over() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 절대 경로
//     crate::front_of_house::hosting::add_to_waitlist();

//     front_of_house::hosting::add_to_waitlist();
// }

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}

// mod customer {
//     pub fn eat_at_restaurant(){
//         hosting::add_to_waitlist();
//     }
// }