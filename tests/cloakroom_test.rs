extern crate cloakroom_model;

use cloakroom_model::cloakroom;
use cloakroom_model::items::CloakroomItems;

fn find_free_locker(cloakroom: &mut cloakroom::Cloakroom) -> Result<cloakroom::Locker, ()> {
    match cloakroom.find_free_locker() {
        cloakroom::FreeLockerResult::FreeLockerFound(locker) => Ok(locker),

        cloakroom::FreeLockerResult::NoFreeLockers => {
            assert!(false);
            Err(())
        }
    }
}

fn open_locker(
    cloakroom: &mut cloakroom::Cloakroom,
    key: cloakroom::Key,
) -> Result<cloakroom::Locker, ()> {
    match cloakroom.open_locker(key) {
        Ok(locker) => Ok(locker),

        Err(err_str) => {
            assert!(false, "{}", err_str);
            Err(())
        }
    }
}

fn test1_items(items: &CloakroomItems) {
    assert!(matches!(
        items,
        CloakroomItems {
            num_coats: 1,
            num_backpacks: 1,
            num_umbrellas: 0,
            num_other_items: 6
        }
    ));

    assert_eq!(items.get_total_num_items(), 8);

    let expected_str =
        "num coats: 1, num backpacks: 1, num umbrellas: 0, num other items: 6".to_string();
    assert_eq!(items.to_string(), expected_str);
}

fn test1_initial_locker_state(cloakroom: &cloakroom::Cloakroom) {
    let free_locker_numbers = vec![1, 9, 15];
    for locker_number in free_locker_numbers {
        assert!(matches!(
            cloakroom.get_locker_state(locker_number),
            cloakroom::LockerState::Free
        ));
    }

    let free_locker_numbers = vec![0, 16, 100];
    for locker_number in free_locker_numbers {
        assert!(matches!(
            cloakroom.get_locker_state(locker_number),
            cloakroom::LockerState::NonExistent
        ));
    }
}

#[test]
fn test1() -> Result<(), ()> {
    let num_lockers = 15;
    let max_items_per_locker = 8;
    let mut cloakroom = cloakroom::Cloakroom::new(num_lockers, max_items_per_locker);
    test1_initial_locker_state(&cloakroom);

    let mut locker = find_free_locker(&mut cloakroom)?;
    assert!(matches!(
        cloakroom.get_locker_state(locker.get_locker_number()),
        cloakroom::LockerState::ContentsBeingChanged
    ));

    assert!(locker.set_num_backpacks(1).is_ok());
    assert!(locker.set_num_coats(1).is_ok());
    assert!(locker.set_num_other_items(6).is_ok());

    // Cannot add any more items because locker is full.
    assert!(locker.set_num_umbrellas(1).is_err());

    test1_items(locker.get_items());

    let locker_number = locker.get_locker_number();
    let key = cloakroom.close_locker(locker);
    match cloakroom.get_locker_state(locker_number) {
        cloakroom::LockerState::Closed(items) => {
            test1_items(items);
        }

        _ => {
            assert!(false);
        }
    }

    let locker = open_locker(&mut cloakroom, key)?;
    let locker_number = locker.get_locker_number();

    assert!(matches!(
        cloakroom.get_locker_state(locker_number),
        cloakroom::LockerState::ContentsBeingChanged
    ));

    let items = cloakroom.vacate_locker(locker);
    test1_items(&items);

    assert!(matches!(
        cloakroom.get_locker_state(locker_number),
        cloakroom::LockerState::Free
    ));

    Ok(())
}

#[test]
fn test2() -> Result<(), ()> {
    //  Create cloakroom.
    let num_lockers = 100;
    let max_items_per_locker = 10;
    let mut cloakroom = cloakroom::Cloakroom::new(num_lockers, max_items_per_locker);

    // Deposit some items.
    let mut locker = find_free_locker(&mut cloakroom)?;
    assert!(locker.set_num_coats(3).is_ok());
    let key = cloakroom.close_locker(locker);

    // Change locker contents.
    let mut locker = open_locker(&mut cloakroom, key)?;

    assert!(locker.set_num_other_items(11).is_err());

    assert!(locker.set_num_coats(2).is_ok());
    assert!(locker.set_num_other_items(7).is_ok());

    let key = cloakroom.close_locker(locker);

    // Collect items.
    let locker = open_locker(&mut cloakroom, key)?;
    let items = cloakroom.vacate_locker(locker);

    assert_eq!(items.get_total_num_items(), 9);

    let expected_str =
        "num coats: 2, num backpacks: 0, num umbrellas: 0, num other items: 7".to_string();
    assert_eq!(items.to_string(), expected_str);

    Ok(())
}

#[test]
fn test3() -> Result<(), ()> {
    //  Create cloakroom.
    let num_lockers = 2;
    let max_items_per_locker = 12;
    let mut cloakroom = cloakroom::Cloakroom::new(num_lockers, max_items_per_locker);

    let mut customer1_locker = find_free_locker(&mut cloakroom)?;
    assert!(customer1_locker.set_num_umbrellas(1).is_ok());
    let customer1_key = cloakroom.close_locker(customer1_locker);

    let mut _customer2_locker = find_free_locker(&mut cloakroom)?;

    // Cloakroom should now be full.
    assert!(matches!(
        cloakroom.find_free_locker(),
        cloakroom::FreeLockerResult::NoFreeLockers
    ));

    let customer1_locker = open_locker(&mut cloakroom, customer1_key)?;
    let items = cloakroom.vacate_locker(customer1_locker);

    assert_eq!(items.get_total_num_items(), 1);

    let expected_str =
        "num coats: 0, num backpacks: 0, num umbrellas: 1, num other items: 0".to_string();
    assert_eq!(items.to_string(), expected_str);

    // There should now be a free locker.
    let mut _customer3_locker = find_free_locker(&mut cloakroom)?;

    Ok(())
}

#[test]
fn test4() -> Result<(), ()> {
    //  Create cloakroom.
    let num_lockers = 1;
    let max_items_per_locker = 15;
    let mut cloakroom = cloakroom::Cloakroom::new(num_lockers, max_items_per_locker);

    let mut customer1_locker = find_free_locker(&mut cloakroom)?;

    // The only locker is now in use.
    assert!(matches!(
        cloakroom.find_free_locker(),
        cloakroom::FreeLockerResult::NoFreeLockers
    ));

    assert!(customer1_locker.set_num_umbrellas(2).is_ok());
    assert!(customer1_locker.set_num_other_items(4).is_ok());
    assert!(customer1_locker.set_num_coats(3).is_ok());
    assert!(customer1_locker.set_num_backpacks(1).is_ok());

    // Customer has change of heart and decides to vacate locker.
    let locker_number = customer1_locker.get_locker_number();
    let items = cloakroom.vacate_locker(customer1_locker);
    assert!(matches!(
        cloakroom.get_locker_state(locker_number),
        cloakroom::LockerState::Free
    ));

    assert_eq!(items.get_total_num_items(), 10);

    let expected_str =
        "num coats: 3, num backpacks: 1, num umbrellas: 2, num other items: 4".to_string();
    assert_eq!(items.to_string(), expected_str);

    // The locker is now available for use by another customer.
    let mut _customer2_locker = find_free_locker(&mut cloakroom)?;

    Ok(())
}
