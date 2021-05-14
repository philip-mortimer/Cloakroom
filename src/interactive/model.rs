use std::collections::hash_map;

use super::cloakroom_io::{create_cloakroom, print_contents_of_closed_lockers};
use super::console;
use super::locker_io;
use crate::cloakroom;

type KeyCollection = hash_map::HashMap<cloakroom::LockerNumber, cloakroom::Key>;

fn input_menu_option(max_option: u8) -> u8 {
    let prompt = format!("Please enter option between 1 and {}: ", max_option);

    console::input_within_range_loop(&prompt, 1, max_option)
}

/// Interactive model of a cloakroom.
pub struct Model {
    cloakroom: cloakroom::Cloakroom,
    keys: KeyCollection,
}

impl Model {
    pub fn new() -> Model {
        let cloakroom = create_cloakroom();

        Model {
            cloakroom,
            keys: KeyCollection::new(),
        }
    }

    fn open_locker(&mut self) -> Result<cloakroom::Locker, ()> {
        if self.keys.len() == 0 {
            console::print_err("there are no closed lockers from which to collect items");
            console::halt();
            return Err(());
        };

        let locker_number = console::input_loop("Enter locker number printed on key: ");
        match self.keys.remove(&locker_number) {
            Some(key) => match self.cloakroom.open_locker(key) {
                Ok(locker) => Ok(locker),

                Err(err_str) => {
                    println!("{} {}", console::ERR_PREFIX, err_str);
                    Err(())
                }
            },

            None => {
                println!(
                    "{} key for locker number {} not found.",
                    console::ERR_PREFIX,
                    locker_number
                );
                console::halt();

                Err(())
            }
        }
    }

    fn close_locker(&mut self, locker: cloakroom::Locker) {
        let locker_number = locker.get_locker_number();
        let locker_contents_str = locker.get_items().to_string();
        let num_items = locker.get_total_num_items();

        let key = self.cloakroom.close_locker(locker);
        self.keys.insert(locker_number, key);

        println!("---------------------------------------------------------------------------------------");
        if num_items > 0 {
            print!(
                "Locker number {} has been closed and key has been obtained. ",
                locker_number
            );
            println!("Contents are as follows:");
            println!("{}", locker_contents_str);
        } else {
            print!("Locker number {}, which is empty, ", locker_number);
            println!("has been closed and key has been obtained.");
        }
        println!("---------------------------------------------------------------------------------------");
        console::halt();
    }

    fn change_locker_contents(&mut self, mut locker: cloakroom::Locker) {
        loop {
            locker_io::print_locker_info(&locker);

            println!("1) Change number of coats");
            println!("2) Change number of backpacks");
            println!("3) Change number of umbrellas");
            println!("4) Change number of other items");
            println!("5) Close locker\n");

            let option = input_menu_option(5);
            println!();
            match option {
                1 => {
                    locker_io::input_num_coats(&mut locker);
                }

                2 => {
                    locker_io::input_num_backpacks(&mut locker);
                }

                3 => {
                    locker_io::input_num_umbrellas(&mut locker);
                }

                4 => {
                    locker_io::input_num_other_items(&mut locker);
                }

                5 => {
                    self.close_locker(locker);
                    break;
                }

                _ => {
                    // Should not get here.
                }
            }
        }
    }

    fn deposit_items(&mut self) {
        let locker = match self.cloakroom.find_free_locker() {
            cloakroom::FreeLockerResult::FreeLockerFound(locker) => locker,

            cloakroom::FreeLockerResult::NoFreeLockers => {
                println!("There are no free lockers.");
                console::halt();
                return;
            }
        };
        println!(
            " *** Found free locker number {} ***",
            locker.get_locker_number()
        );
        self.change_locker_contents(locker);
    }

    fn collect_items(&mut self) {
        let locker = self.open_locker();
        if let Ok(locker) = locker {
            let locker_number = locker.get_locker_number();
            let items = self.cloakroom.vacate_locker(locker);
            println!(
                "-----------------------------------------------------------------------------"
            );
            println!(
                "Collected following items from locker number {}:",
                locker_number
            );
            println!("{}", items);
            println!(
                "-----------------------------------------------------------------------------"
            );

            console::halt();
        };
    }

    fn change_contents(&mut self) {
        let locker = self.open_locker();
        if let Ok(locker) = locker {
            self.change_locker_contents(locker);
        }
    }

    fn print_cloakroom_contents(&self) {
        if self.keys.len() == 0 {
            println!("There are no closed lockers.");
        } else {
            print_contents_of_closed_lockers(&self.cloakroom);
        };
        console::halt();
    }

    pub fn run(&mut self) {
        loop {
            println!("\n1) Deposit items in a locker");
            println!("2) Collect items from a locker");
            println!("3) Change locker contents");
            println!("4) Print contents of closed lockers");
            println!("5) Quit\n");

            let option = input_menu_option(5);
            println!();
            match option {
                1 => {
                    self.deposit_items();
                }

                2 => {
                    self.collect_items();
                }

                3 => {
                    self.change_contents();
                }

                4 => {
                    self.print_cloakroom_contents();
                }

                5 => {
                    break;
                }

                _ => {
                    // Should not get here.
                }
            }
        }
    }
}
