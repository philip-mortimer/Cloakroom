use super::console;
use crate::cloakroom;
use std::fmt;
use std::str::FromStr;

const MIN_VALID_NUM_LOCKERS: usize = 1;
const MAX_VALID_NUM_LOCKERS: usize = 1000;

const MIN_VALID_MAX_ITEMS: u16 = 5;
const MAX_VALID_MAX_ITEMS: u16 = 15;

pub fn input_param<T>(min_valid: T, max_valid: T, descr: &str) -> T
where
    T: fmt::Display + FromStr + PartialOrd + Copy,
{
    let prompt = format!(
        "Enter {} (number between {} and {}): ",
        descr, min_valid, max_valid
    );
    console::input_within_range_loop(&prompt, min_valid, max_valid)
}

fn input_num_lockers() -> usize {
    input_param(
        MIN_VALID_NUM_LOCKERS,
        MAX_VALID_NUM_LOCKERS,
        "number of lockers",
    )
}

fn input_locker_capacity() -> u16 {
    input_param(
        MIN_VALID_MAX_ITEMS,
        MAX_VALID_MAX_ITEMS,
        "number of items each locker can hold",
    )
}

pub fn create_cloakroom() -> cloakroom::Cloakroom {
    let num_lockers = input_num_lockers();
    let max_items_per_locker = input_locker_capacity();

    cloakroom::Cloakroom::new(num_lockers, max_items_per_locker)
}

pub fn print_contents_of_closed_lockers(cloakroom: &cloakroom::Cloakroom) {
    for locker_number in 1..=cloakroom.get_num_lockers() {
        let locker_state = cloakroom.get_locker_state(locker_number);
        if let cloakroom::LockerState::Closed(items) = locker_state {
            println!("locker number {}: [{}]", locker_number, items);
        }
    }
}
