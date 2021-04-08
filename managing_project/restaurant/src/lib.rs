pub mod front_of_house;

pub use crate::front_of_house::{hosting as H, serving as S};

pub use crate::front_of_house::*;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // with use
    H::add_to_waitlist();
}