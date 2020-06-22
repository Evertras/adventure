use super::super::components;
use specs::{Builder, World, WorldExt};

pub fn create_in(world: &mut World) {
    const LEFT: i32 = -20;
    const RIGHT: i32 = 20;
    const TOP: i32 = -10;
    const BOTTOM: i32 = 10;

    let mut walls: Vec<(i32, i32)> = Vec::new();
    let mut floors: Vec<(i32, i32)> = Vec::new();

    for x in (LEFT-1)..(RIGHT+1) {
        walls.push((x, TOP-1));
        walls.push((x, BOTTOM));
    }

    for y in TOP..BOTTOM {
        walls.push((LEFT-1, y));
        walls.push((RIGHT, y));
    }

    for x in LEFT..RIGHT {
        for y in TOP..BOTTOM {
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
