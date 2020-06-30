use super::super::components::Position;
use specs::Entity;
use std::collections::{HashMap, HashSet};

bitflags! {
    #[derive(Default)]
    pub struct TileProperties: u16 {
        const BLOCKED = 1 << 0;
    }
}

#[derive(Clone, Default)]
struct TileData {
    properties: TileProperties,
    entities: HashSet<Entity>,
}

pub struct GameMap {
    data: HashMap<Position, TileData>,
}

impl GameMap {
    pub fn new() -> GameMap {
        GameMap {
            data: HashMap::with_capacity(5000),
        }
    }

    pub fn add(&mut self, coordinate: &Position, entity: Entity) {
        let entry = &mut self.data.entry(coordinate.clone()).or_default().entities;

        entry.insert(entity);
    }

    pub fn get_entities(&self, coordinate: &Position) -> Option<&HashSet<Entity>> {
        match self.data.get(coordinate) {
            None => None,
            Some(entry) => Some(&entry.entities),
        }
    }

    pub fn mark_tile(&mut self, coordinate: &Position, flags: TileProperties) {
        let mut entry = &mut self.data.entry(coordinate.clone()).or_default();

        entry.properties = entry.properties | flags;
    }

    pub fn clear_tile_properties(&mut self, coordinate: &Position) {
        match self.data.get_mut(&coordinate) {
            None => (),
            Some(entry) => {
                entry.properties = TileProperties::empty();
            }
        }
    }

    pub fn tile_is(&self, coordinate: &Position, flags: TileProperties) -> bool {
        match self.data.get(&coordinate) {
            None => false,
            Some(entry) => entry.properties.contains(flags),
        }
    }

    pub fn clear_all(&mut self) {
        self.data.clear();
    }
}

impl Default for GameMap {
    fn default() -> Self {
        GameMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, World, WorldExt};

    #[test]
    fn coordinate_up_returns_1_up() {
        let initial = Position { x: -4, y: 3 };

        // Remember that up is negative in Y coordinates for our world
        let expected = Position { x: -4, y: 2 };

        assert_eq!(initial.up(), expected);
    }

    #[test]
    fn coordinate_down_returns_1_down() {
        let initial = Position { x: -4, y: 3 };

        // Remember that down is positive in Y coordinates for our world
        let expected = Position { x: -4, y: 4 };

        assert_eq!(initial.down(), expected);
    }

    #[test]
    fn coordinate_right_returns_1_right() {
        let initial = Position { x: 5, y: 3 };
        let expected = Position { x: 6, y: 3 };

        assert_eq!(initial.right(), expected);
    }

    #[test]
    fn coordinate_left_returns_1_left() {
        let initial = Position { x: 5, y: 3 };
        let expected = Position { x: 4, y: 3 };

        assert_eq!(initial.left(), expected);
    }

    #[test]
    fn safely_returns_no_entities_when_not_found() {
        let map: GameMap = GameMap::new();
        let square = Position { x: -4, y: 3 };

        let result = map.get_entities(&square);

        match result {
            Some(_) => panic!("Expected to be empty"),
            None => (),
        };
    }

    #[test]
    fn adds_entities_and_returns_them() {
        let mut map: GameMap = GameMap::new();

        let mut world = World::new();
        let expected_value = world.create_entity().build();
        let square = Position { x: -3, y: 4 };

        map.add(&square, expected_value);

        let result = map.get_entities(&square);

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&expected_value), true);
            }
            None => panic!("Not found"),
        };

        let result = map.get_entities(&square.up());

        match result {
            Some(_) => panic!("Should not have gotten anything back from wrong square"),
            None => (),
        };
    }

    #[test]
    fn clears_stored_values() {
        let mut map: GameMap = GameMap::new();

        let mut world = World::new();
        let expected_value = world.create_entity().build();
        let square = Position { x: -3, y: 4 };

        map.add(&square, expected_value);
        map.clear_all();

        let result = map.get_entities(&square);

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
    fn stores_entities_in_different_locations() {
        let mut map: GameMap = GameMap::new();

        let mut world = World::new();
        let first_value = world.create_entity().build();
        let second_value = world.create_entity().build();
        let first_square = Position { x: -3, y: 4 };
        let second_square = first_square.up();

        map.add(&first_square, first_value);
        map.add(&second_square, second_value);

        let result = map.get_entities(&first_square);

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&first_value), true);
            }
            None => panic!("Not found"),
        };

        let result = map.get_entities(&second_square);

        match result {
            Some(set) => {
                assert_eq!(set.len(), 1);
                assert_eq!(set.contains(&second_value), true);
            }
            None => panic!("Not found"),
        };
    }

    #[test]
    fn stores_multiple_unique_entities_in_same_location() {
        let mut map: GameMap = GameMap::new();

        let mut world = World::new();
        let first_value = world.create_entity().build();
        let second_value = world.create_entity().build();
        let square = Position { x: -3, y: 4 };

        map.add(&square, first_value);
        map.add(&square, second_value);

        let result = map.get_entities(&square);

        match result {
            Some(set) => {
                assert_eq!(set.len(), 2);
                assert_eq!(set.contains(&first_value), true);
                assert_eq!(set.contains(&second_value), true);
            }
            None => panic!("Not found"),
        };
    }

    #[test]
    fn marks_square_as_blocked() {
        let mut map: GameMap = GameMap::new();
        let square = Position { x: -3, y: 4 };

        assert!(!map.tile_is(&square, TileProperties::BLOCKED));
        map.mark_tile(&square, TileProperties::BLOCKED);
        assert!(map.tile_is(&square, TileProperties::BLOCKED));
    }

    #[test]
    fn clears_tile_properties() {
        let mut map: GameMap = GameMap::new();
        let square = Position { x: -3, y: 4 };

        map.mark_tile(&square, TileProperties::BLOCKED);
        assert!(map.tile_is(&square, TileProperties::BLOCKED));
        map.clear_tile_properties(&square);
        assert!(!map.tile_is(&square, TileProperties::BLOCKED));
    }
}
