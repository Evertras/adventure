use super::super::components;
use specs::{Builder, World, WorldExt};

pub fn create_in(world: &mut World) {
    let mut walls: Vec<(i32, i32)> = Vec::new();
    let mut floors: Vec<(i32, i32)> = Vec::new();

    for x in -6..6 {
        walls.push((x, -6));
        walls.push((x, 5));
    }

    for y in -5..5 {
        walls.push((-6, y));
        walls.push((5, y));
    }

    for x in -5..5 {
        for y in -5..5 {
            floors.push((x, y));
        }
    }

    for wall in walls {
        world
            .create_entity()
            .with(components::Position {
                x: wall.0 as f64,
                y: wall.1 as f64,
            })
            .with(components::Draw {
                fg_r: 255,
                fg_g: 255,
                fg_b: 255,
                bg_r: 128,
                bg_g: 128,
                bg_b: 128,
                layer: components::DL_MAP,
                rune: 'X',
            })
            .build();
    }

    for floor in floors {
        world
            .create_entity()
            .with(components::Position {
                x: floor.0 as f64,
                y: floor.1 as f64,
            })
            .with(components::Draw {
                fg_r: 255,
                fg_g: 255,
                fg_b: 255,
                bg_r: 28,
                bg_g: 28,
                bg_b: 28,
                layer: components::DL_MAP,
                rune: ' ',
            })
            .build();
    }
}
