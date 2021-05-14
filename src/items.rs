use std::fmt;

/// Specifies items to be stored in the cloakroom.
///
/// # Examples
///
/// ```
/// use cloakroom_model::items::CloakroomItems;
///
///
/// let mut items = CloakroomItems::new();
/// assert_eq!(items.get_total_num_items(), 0);
///
/// let expected_str =
/// "num coats: 0, num backpacks: 0, num umbrellas: 0, num other items: 0"
///     .to_string();
/// assert_eq!(items.to_string(), expected_str);
///
/// items.num_coats = 2;
/// assert_eq!(items.get_total_num_items(), 2);
///
/// let expected_str =
/// "num coats: 2, num backpacks: 0, num umbrellas: 0, num other items: 0"
///     .to_string();
/// assert_eq!(items.to_string(), expected_str);
///
///
/// let items = CloakroomItems {
///     num_coats: 3,
///     num_backpacks: 1,
///     num_umbrellas: 1,
///     num_other_items: 10,
/// };
/// assert_eq!(items.get_total_num_items(), 15);
///
/// let expected_str =
/// "num coats: 3, num backpacks: 1, num umbrellas: 1, num other items: 10"
///     .to_string();
/// assert_eq!(items.to_string(), expected_str);
/// ```
#[derive(Debug)]
pub struct CloakroomItems {
    pub num_coats: u8,
    pub num_backpacks: u8,
    pub num_umbrellas: u8,
    pub num_other_items: u8,
}

impl CloakroomItems {
    pub fn new() -> CloakroomItems {
        CloakroomItems {
            num_coats: 0,
            num_backpacks: 0,
            num_umbrellas: 0,
            num_other_items: 0,
        }
    }

    pub fn get_total_num_items(&self) -> u16 {
        self.num_coats as u16
            + self.num_backpacks as u16
            + self.num_umbrellas as u16
            + self.num_other_items as u16
    }
}

impl fmt::Display for CloakroomItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "num coats: {}, ", self.num_coats)?;
        write!(f, "num backpacks: {}, ", self.num_backpacks)?;
        write!(f, "num umbrellas: {}, ", self.num_umbrellas)?;
        write!(f, "num other items: {}", self.num_other_items)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::items::CloakroomItems;

    #[test]
    fn test1() {
        let mut items = CloakroomItems::new();
        assert_eq!(items.get_total_num_items(), 0);

        let expected_str =
            "num coats: 0, num backpacks: 0, num umbrellas: 0, num other items: 0".to_string();
        assert_eq!(items.to_string(), expected_str);

        items.num_coats = 2;
        items.num_other_items = 1;
        assert_eq!(items.get_total_num_items(), 3);

        let expected_str =
            "num coats: 2, num backpacks: 0, num umbrellas: 0, num other items: 1".to_string();
        assert_eq!(items.to_string(), expected_str);
    }

    #[test]
    fn test2() {
        let items = CloakroomItems {
            num_coats: 0,
            num_backpacks: 1,
            num_umbrellas: 0,
            num_other_items: 0,
        };
        assert_eq!(items.get_total_num_items(), 1);

        let expected_str =
            "num coats: 0, num backpacks: 1, num umbrellas: 0, num other items: 0".to_string();
        assert_eq!(items.to_string(), expected_str);
    }

    #[test]
    fn test3() {
        let items = CloakroomItems {
            num_coats: 2,
            num_backpacks: 0,
            num_umbrellas: 1,
            num_other_items: 2,
        };
        assert_eq!(items.get_total_num_items(), 5);

        let expected_str =
            "num coats: 2, num backpacks: 0, num umbrellas: 1, num other items: 2".to_string();
        assert_eq!(items.to_string(), expected_str);
    }

    #[test]
    fn test4() {
        let items = CloakroomItems {
            num_coats: 1,
            num_backpacks: 2,
            num_umbrellas: 1,
            num_other_items: 10,
        };
        assert_eq!(items.get_total_num_items(), 14);

        let expected_str =
            "num coats: 1, num backpacks: 2, num umbrellas: 1, num other items: 10".to_string();
        assert_eq!(items.to_string(), expected_str);
    }
}
