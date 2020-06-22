use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(pub i32, pub i32);

pub struct EntityGrid<T> {
    data: HashMap<Coordinate, HashSet<T>>,
}

impl<T> EntityGrid<T> where T: std::hash::Hash + Eq {
    pub fn new() -> EntityGrid<T> {
        EntityGrid{
            data: HashMap::with_capacity(5000),
        }
    }

    pub fn add(&mut self, coordinate: Coordinate, val: T) {
        let entry = self.data.entry(coordinate).or_default();

        entry.insert(val);
    }

    pub fn get(&mut self, coordinate: Coordinate) -> Option<&HashSet<T>> {
        self.data.get(&coordinate)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safely_returns_nothing_when_not_found() {
        let mut eg: EntityGrid<i32> = EntityGrid::new();
        let key = Coordinate(-4, 3);

        let result = eg.get(key);

        match result {
            Some(_) => panic!("Expected to be empty"),
            None => (),
        };
    }

    #[test]
    fn stores_value_and_retrieves_it() {
        let mut eg: EntityGrid<i32> = EntityGrid::new();
        let expected_value = 7;

        eg.add(Coordinate(-3, 4), expected_value);

        let result = eg.get(Coordinate(-3, 4));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&expected_value), true);
            },
            None => panic!("Not found"),
        };
    }

    #[test]
    fn clears_stored_values() {
        let mut eg: EntityGrid<i32> = EntityGrid::new();
        let expected_value = 7;

        eg.add(Coordinate(-3, 4), expected_value);
        eg.clear();

        let result = eg.get(Coordinate(-3, 4));

        // Don't actually care about underlying implementation, just shouldn't
        // actually have the value stored here anymore
        match result {
            Some(set) => {
                assert_eq!(set.len(), 0);
            }
            None => (),
        };
    }

    #[test]
    fn stores_values_in_different_locations() {
        let mut eg: EntityGrid<i32> = EntityGrid::new();
        let first_value = 7;
        let other_value = 30;

        eg.add(Coordinate(-3, 4), first_value);
        eg.add(Coordinate(10, 5), other_value);

        let result = eg.get(Coordinate(-3, 4));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&first_value), true);
            },
            None => panic!("Not found"),
        };

        let result = eg.get(Coordinate(10, 5));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&other_value), true);
            },
            None => panic!("Not found"),
        };
    }

    #[test]
    fn stores_multiple_unique_values_in_same_location() {
        let mut eg: EntityGrid<i32> = EntityGrid::new();
        let first_value = 7;
        let other_value = 30;

        eg.add(Coordinate(-3, 4), first_value);
        eg.add(Coordinate(-3, 4), other_value);

        let result = eg.get(Coordinate(-3, 4));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 2);
                assert_eq!(set.contains(&first_value), true);
                assert_eq!(set.contains(&other_value), true);
            },
            None => panic!("Not found"),
        };
    }
}
