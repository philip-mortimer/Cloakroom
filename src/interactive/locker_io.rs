use super::console;
use crate::cloakroom;

pub fn print_locker_info(locker: &cloakroom::Locker) {
    println!("\n-----------------------------------------------------------------------------");

    println!(
        "Current contents of locker number {} are:",
        locker.get_locker_number()
    );

    println!("{}", locker.get_items());

    println!(
        "Total number of items currently in locker: {}, max items: {}",
        locker.get_total_num_items(),
        locker.get_max_items()
    );

    println!("-----------------------------------------------------------------------------\n");
}

pub fn input_num_coats(locker: &mut cloakroom::Locker) {
    let num_coats = input_num_items("coats");
    if let Err(()) = locker.set_num_coats(num_coats) {
        print_insufficient_space_error();
    };
}

pub fn input_num_backpacks(locker: &mut cloakroom::Locker) {
    let num_backpacks = input_num_items("backpacks");
    if let Err(()) = locker.set_num_backpacks(num_backpacks) {
        print_insufficient_space_error();
    };
}

pub fn input_num_umbrellas(locker: &mut cloakroom::Locker) {
    let num_umbrellas = input_num_items("umbrellas");
    if let Err(()) = locker.set_num_umbrellas(num_umbrellas) {
        print_insufficient_space_error();
    };
}

pub fn input_num_other_items(locker: &mut cloakroom::Locker) {
    let num_other_items = input_num_items("other items");
    if let Err(()) = locker.set_num_other_items(num_other_items) {
        print_insufficient_space_error();
    };
}

fn print_insufficient_space_error() {
    console::print_err("not enough space in locker");
    console::halt();
}

fn input_num_items(item_descr: &str) -> u8 {
    let prompt = format!("Enter number of {0}: ", item_descr);
    console::input_loop(&prompt)
}
