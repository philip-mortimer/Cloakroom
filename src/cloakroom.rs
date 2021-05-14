use crate::items::CloakroomItems;
use std::collections::hash_map::HashMap;

pub type LockerNumber = usize;

pub struct Locker {
    number: LockerNumber,
    max_items: u16,
    items: CloakroomItems,
}

impl Locker {
    pub fn get_locker_number(&self) -> LockerNumber {
        self.number
    }

    pub fn get_total_num_items(&self) -> u16 {
        self.items.get_total_num_items()
    }

    pub fn get_max_items(&self) -> u16 {
        self.max_items
    }

    pub fn get_items(&self) -> &CloakroomItems {
        &self.items
    }

    pub fn set_num_coats(&mut self, num_items: u8) -> Result<(), ()> {
        self.items.num_coats = self.check_change_items(self.items.num_coats, num_items)?;
        Ok(())
    }

    pub fn set_num_backpacks(&mut self, num_items: u8) -> Result<(), ()> {
        self.items.num_backpacks = self.check_change_items(self.items.num_backpacks, num_items)?;
        Ok(())
    }

    pub fn set_num_umbrellas(&mut self, num_items: u8) -> Result<(), ()> {
        self.items.num_umbrellas = self.check_change_items(self.items.num_umbrellas, num_items)?;
        Ok(())
    }

    pub fn set_num_other_items(&mut self, num_items: u8) -> Result<(), ()> {
        self.items.num_other_items =
            self.check_change_items(self.items.num_other_items, num_items)?;
        Ok(())
    }

    fn check_change_items(&self, curr_num_items: u8, new_num_items: u8) -> Result<u8, ()> {
        if self.items.get_total_num_items() - curr_num_items as u16 + new_num_items as u16
            > self.max_items
        {
            Err(())
        } else {
            Ok(new_num_items)
        }
    }
}

pub struct Key {
    locker_number: LockerNumber,
}

impl Key {
    fn new(locker_number: LockerNumber) -> Key {
        Key { locker_number }
    }

    pub fn get_locker_number(&self) -> LockerNumber {
        self.locker_number
    }
}

pub enum FreeLockerResult {
    FreeLockerFound(Locker),
    NoFreeLockers,
}

pub enum LockerState<'a> {
    Free,
    Closed(&'a CloakroomItems),
    ContentsBeingChanged,
    NonExistent,
}

enum LockerInUseState {
    Closed(CloakroomItems),
    ContentsBeingChanged,
}

/// Representation of a cloakroom containing 1 or more lockers.
/// A customer can:
///    - find a free locker
///    - place items (coats, backpacks, umbrellas and other items) in the
///      locker
///    - close and lock the locker, and take away the key
///    - open the locker with the key
///    - remove the items from an open locker
///
/// # Examples
///
/// ```
/// use cloakroom_model::cloakroom::{Cloakroom, FreeLockerResult};
///
/// // Create cloakroom containing 10 lockers each holding up to 7 items.
/// let mut cloakroom = Cloakroom::new(10, 7);
///
/// let mut locker = match cloakroom.find_free_locker() {
///     FreeLockerResult::FreeLockerFound(locker) => locker,
///
///     FreeLockerResult::NoFreeLockers => {
///         assert!(false);
///         return;
///     }
/// };
///
/// assert!(locker.set_num_backpacks(2).is_ok());
/// assert!(locker.set_num_other_items(5).is_ok());
///
/// // Cannot add any more items because locker is full.
/// assert!(locker.set_num_umbrellas(1).is_err());
///
/// let key = cloakroom.close_locker(locker);
///
/// let locker = match cloakroom.open_locker(key) {
///     Ok(locker) => locker,
///
///     Err(err_str) => {
///         assert!(false, err_str);
///         return;
///     }
///
/// };
///
/// let items = cloakroom.vacate_locker(locker);
/// assert_eq!(items.get_total_num_items(), 7);
/// let expected_str =
///     "num coats: 0, num backpacks: 2, num umbrellas: 0, num other items: 5"
///     .to_string();
/// assert_eq!(items.to_string(), expected_str);
/// ```
pub struct Cloakroom {
    num_lockers: usize,
    max_items_per_locker: u16,

    lockers_in_use: HashMap<LockerNumber, LockerInUseState>,
}

impl Cloakroom {
    pub fn new(num_lockers: usize, max_items_per_locker: u16) -> Cloakroom {
        Cloakroom {
            num_lockers,
            max_items_per_locker,
            lockers_in_use: HashMap::new(),
        }
    }

    pub fn get_num_lockers(&self) -> usize {
        self.num_lockers
    }

    pub fn find_free_locker(&mut self) -> FreeLockerResult {
        if self.lockers_in_use.len() < self.get_num_lockers() {
            let mut locker_number = 1;
            while self.lockers_in_use.contains_key(&locker_number) {
                locker_number += 1;
            }

            self.lockers_in_use
                .insert(locker_number, LockerInUseState::ContentsBeingChanged);

            let locker = Locker {
                number: locker_number,
                max_items: self.max_items_per_locker,
                items: CloakroomItems::new(),
            };
            FreeLockerResult::FreeLockerFound(locker)
        } else {
            FreeLockerResult::NoFreeLockers
        }
    }

    pub fn close_locker(&mut self, locker: Locker) -> Key {
        let locker_number = locker.get_locker_number();
        self.lockers_in_use
            .insert(locker_number, LockerInUseState::Closed(locker.items));

        Key::new(locker_number)
    }

    pub fn open_locker(&mut self, key: Key) -> Result<Locker, String> {
        let locker_number = key.get_locker_number();
        let items = match self.lockers_in_use.remove(&locker_number) {
            Some(items) => items,

            None => {
                let err_str = format!(
                    "unexpectedly failed to find record for locker number {}",
                    key.get_locker_number()
                );
                return Err(err_str);
            }
        };

        match items {
            LockerInUseState::Closed(items) => {
                self.lockers_in_use
                    .insert(locker_number, LockerInUseState::ContentsBeingChanged);

                let locker = Locker {
                    number: locker_number,
                    max_items: self.max_items_per_locker,
                    items,
                };
                Ok(locker)
            }

            LockerInUseState::ContentsBeingChanged => {
                let err_str = format!(
                    "record for locker number {} unexpectedly contains no data",
                    locker_number
                );
                Err(err_str)
            }
        }
    }

    /// Remove all items from a locker and leave it in a free state so
    /// that it can be used by another customer.
    pub fn vacate_locker(&mut self, locker: Locker) -> CloakroomItems {
        // Remove record for locker from records of lockers in use so that
        // it's state is free.
        let _ = self.lockers_in_use.remove(&locker.get_locker_number());

        locker.items
    }

    pub fn get_locker_state(&self, locker_number: LockerNumber) -> LockerState {
        if (locker_number < 1) || (locker_number > self.num_lockers) {
            return LockerState::NonExistent;
        };

        let items = match self.lockers_in_use.get(&locker_number) {
            Some(items) => items,
            None => {
                return LockerState::Free;
            }
        };

        match items {
            LockerInUseState::Closed(ref items) => LockerState::Closed(items),

            LockerInUseState::ContentsBeingChanged => LockerState::ContentsBeingChanged,
        }
    }
}
