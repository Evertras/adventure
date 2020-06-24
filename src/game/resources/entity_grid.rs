use specs::Entity;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(pub i32, pub i32);

pub struct EntityGrid {
    data: HashMap<Coordinate, HashSet<Entity>>,
}

impl EntityGrid {
    pub fn new() -> EntityGrid {
        EntityGrid {
            data: HashMap::with_capacity(5000),
        }
    }

    pub fn add(&mut self, coordinate: Coordinate, entity: Entity) {
        let entry = self.data.entry(coordinate).or_default();

        entry.insert(entity);
    }

    pub fn get(&self, coordinate: Coordinate) -> Option<&HashSet<Entity>> {
        self.data.get(&coordinate)
    }

    pub fn contains_any<T>(&self, coordinate: Coordinate) -> bool {
        match self.get(coordinate) {
            None => false,
            Some(entities) => {
                for entity in entities {}

                true
            }
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for EntityGrid {
    fn default() -> Self {
        EntityGrid::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, World, WorldExt};

    #[test]
    fn safely_returns_nothing_when_not_found() {
        let eg: EntityGrid = EntityGrid::new();
        let key = Coordinate(-4, 3);

        let result = eg.get(key);

        match result {
            Some(_) => panic!("Expected to be empty"),
            None => (),
        };
    }

    #[test]
    fn stores_value_and_retrieves_it() {
        let mut eg: EntityGrid = EntityGrid::new();

        let mut world = World::new();
        let expected_value = world.create_entity().build();

        eg.add(Coordinate(-3, 4), expected_value);

        let result = eg.get(Coordinate(-3, 4));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&expected_value), true);
            }
            None => panic!("Not found"),
        };
    }

    #[test]
    fn clears_stored_values() {
        let mut eg: EntityGrid = EntityGrid::new();

        let mut world = World::new();
        let expected_value = world.create_entity().build();

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
        let mut eg: EntityGrid = EntityGrid::new();

        let mut world = World::new();
        let first_value = world.create_entity().build();
        let other_value = world.create_entity().build();

        eg.add(Coordinate(-3, 4), first_value);
        eg.add(Coordinate(10, 5), other_value);

        let result = eg.get(Coordinate(-3, 4));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&first_value), true);
            }
            None => panic!("Not found"),
        };

        let result = eg.get(Coordinate(10, 5));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&other_value), true);
            }
            None => panic!("Not found"),
        };
    }

    #[test]
    fn stores_multiple_unique_values_in_same_location() {
        let mut eg: EntityGrid = EntityGrid::new();

        let mut world = World::new();
        let first_value = world.create_entity().build();
        let other_value = world.create_entity().build();

        eg.add(Coordinate(-3, 4), first_value);
        eg.add(Coordinate(-3, 4), other_value);

        let result = eg.get(Coordinate(-3, 4));

        match result {
            Some(set) => {
                assert_eq!(set.len(), 2);
                assert_eq!(set.contains(&first_value), true);
                assert_eq!(set.contains(&other_value), true);
            }
            None => panic!("Not found"),
        };
    }
}
