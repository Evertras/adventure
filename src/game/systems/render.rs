use super::super::{components, resources};
use specs::{Read, ReadStorage, System};

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

pub trait Renderer {
    fn draw_at(&mut self, x: usize, y: usize, fg: &Color, bg: &Color, rune: char);

    fn size(&self) -> (usize, usize);

    fn flush(&mut self);
}

pub struct Render<T: Renderer> {
    renderer: T,

    back_buffer: Vec<components::Draw>,
    back_buffer_width: usize,
    back_buffer_height: usize,
}

impl<T: Renderer> Render<T> {
    pub fn new(renderer: T) -> Render<T> {
        Render {
            renderer,
            back_buffer: vec![],
            back_buffer_width: 0,
            back_buffer_height: 0,
        }
    }
}

impl<'a, T: Renderer> System<'a> for Render<T> {
    type SystemData = (
        Read<'a, resources::CameraCenter>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Draw>,
    );

    fn run(&mut self, (camera_center, pos, draw): Self::SystemData) {
        use specs::Join;

        let (width, height) = self.renderer.size();

        let half_width = width as i32 / 2;
        let half_height = height as i32 / 2;

        let min_x = camera_center.x - half_width;
        let max_x = camera_center.x + half_width + (width as i32 % 2);
        let min_y = camera_center.y - half_height;
        let max_y = camera_center.y + half_height + (height as i32 % 2);

        let offset_x = if min_x < 0 { min_x - 1 } else { min_x };
        let offset_y = if min_y < 0 { min_y - 1 } else { min_y };

        let mut to_draw: Vec<(usize, usize, &components::Draw)> = vec![];

        for (pos, draw) in (&pos, &draw).join() {
            let tile_x = pos.x as i32;
            let tile_y = pos.y as i32;

            if tile_x >= min_x && tile_x < max_x && tile_y >= min_y && tile_y < max_y {
                to_draw.push((
                    (tile_x - offset_x) as usize,
                    (tile_y - offset_y) as usize,
                    draw,
                ));
            }
        }

        to_draw.sort_by_key(|k| &k.2.layer);

        let blank = components::Draw {
            fg_r: 200,
            fg_g: 0,
            fg_b: 0,

            bg_r: 0,
            bg_g: 0,
            bg_b: 0,

            layer: components::DL_MAP,

            rune: '?',
        };

        let mut buffer: Vec<&components::Draw> = vec![&blank; width * height];

        for (tile_x, tile_y, draw) in to_draw {
            let i = (tile_y * width + tile_x) as usize;

            buffer[i] = draw;
        }

        if width != self.back_buffer_width || height != self.back_buffer_height {
            self.back_buffer = vec![blank.clone(); width * height];
            self.back_buffer_width = width;
            self.back_buffer_height = height;
        }

        for x in 0..width {
            for y in 0..height {
                let i = (y * width + x) as usize;
                let draw = buffer[i];

                if *draw != self.back_buffer[i] {
                    self.back_buffer[i] = draw.clone();
                    self.renderer.draw_at(
                        x + 1,
                        y + 1,
                        &Color {
                            r: draw.fg_r,
                            g: draw.fg_g,
                            b: draw.fg_b,
                        },
                        &Color {
                            r: draw.bg_r,
                            g: draw.bg_g,
                            b: draw.bg_b,
                        },
                        draw.rune,
                    );
                }
            }
        }

        self.renderer.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    struct MockRenderer {
        width: usize,
        height: usize,

        drew_at_x: Option<usize>,
        drew_at_y: Option<usize>,
        drew_fg: Option<Color>,
        drew_bg: Option<Color>,
        drew_rune: Option<char>,
        drew_count: usize,
        flush_count: usize,
    }

    impl MockRenderer {
        fn new(width: usize, height: usize) -> MockRenderer {
            MockRenderer {
                width,
                height,

                drew_at_x: None,
                drew_at_y: None,

                drew_fg: None,
                drew_bg: None,
                drew_rune: None,

                drew_count: 0,
                flush_count: 0,
            }
        }
    }

    impl Renderer for MockRenderer {
        fn draw_at(&mut self, x: usize, y: usize, fg: &Color, bg: &Color, rune: char) {
            self.drew_at_x = Some(x);
            self.drew_at_y = Some(y);
            self.drew_fg = Some(fg.clone());
            self.drew_bg = Some(bg.clone());
            self.drew_rune = Some(rune);
            self.drew_count += 1;
        }

        fn size(&self) -> (usize, usize) {
            (self.width, self.height)
        }

        fn flush(&mut self) {
            self.flush_count += 1;
        }
    }

    #[test]
    fn culls_entities_outside_camera_but_draws_entities_inside() {
        let mut world = World::new();

        let fg_r = 37;
        let fg_g = 84;
        let fg_b = 244;
        let bg_r = 43;
        let bg_g = 134;
        let bg_b = 101;
        let visible_x: f64 = 1.2;
        let visible_y: f64 = -51.3;
        let rune = '+';
        let camera_x = 2;
        let camera_y = -50;
        let width = 5;
        let height = 5;
        let expected_x = 2;
        let expected_y = 3;
        let mock_renderer = MockRenderer::new(width, height);
        let camera_center = resources::CameraCenter {
            x: camera_x,
            y: camera_y,
        };

        world.register::<components::Position>();
        world.register::<components::Draw>();

        world.insert(camera_center);

        let mut spawn = |x: f64, y: f64| {
            world
                .create_entity()
                .with(components::Position { x, y })
                .with(components::Draw {
                    fg_r,
                    fg_g,
                    fg_b,

                    bg_r,
                    bg_g,
                    bg_b,

                    layer: components::DL_ENTITY,

                    rune,
                })
                .build()
        };

        // This should be visible
        spawn(visible_x, visible_y);

        // Off to the right
        spawn(5.0, visible_y);

        // Off to the left
        spawn(-1.001, visible_y);

        // Above
        spawn(visible_x, -53.00001);

        // Below
        spawn(visible_x, -46.9);

        let mut render = Render::new(mock_renderer);

        render.run_now(&world);
        world.maintain();

        // Should only have one thing drawn, the others should be culled
        assert_eq!(render.renderer.drew_count, 1);

        assert_eq!(render.renderer.flush_count, 1);

        assert_eq!(render.renderer.drew_at_x.unwrap(), expected_x);
        assert_eq!(render.renderer.drew_at_y.unwrap(), expected_y);

        assert_eq!(render.renderer.drew_rune.unwrap(), rune);

        assert_eq!(
            render.renderer.drew_fg.unwrap(),
            Color {
                r: fg_r,
                g: fg_g,
                b: fg_b
            }
        );

        assert_eq!(
            render.renderer.drew_bg.unwrap(),
            Color {
                r: bg_r,
                g: bg_g,
                b: bg_b
            }
        );
    }

    #[test]
    fn draws_entities_over_map() {
        let mut world = World::new();

        let lower_rune = '_';
        let upper_rune = '+';
        let camera_x = 2;
        let camera_y = -50;
        let width = 5;
        let height = 5;
        let mock_renderer = MockRenderer::new(width, height);
        let camera_center = resources::CameraCenter {
            x: camera_x,
            y: camera_y,
        };

        world.register::<components::Position>();
        world.register::<components::Draw>();

        world.insert(camera_center);

        let mut spawn = |x: f64, y: f64, rune: char, layer: components::DrawLayer| {
            world
                .create_entity()
                .with(components::Position { x, y })
                .with(components::Draw {
                    fg_r: 255,
                    fg_g: 255,
                    fg_b: 255,

                    bg_r: 255,
                    bg_g: 255,
                    bg_b: 255,

                    layer,

                    rune,
                })
                .build()
        };

        // Add a bunch extra to hedge against any potential hidden randomization
        for _ in 0..500 {
            spawn(
                camera_x as f64,
                camera_y as f64,
                lower_rune,
                components::DL_MAP,
            );
        }
        spawn(
            camera_x as f64,
            camera_y as f64,
            upper_rune,
            components::DL_ENTITY,
        );
        for _ in 0..500 {
            spawn(
                camera_x as f64,
                camera_y as f64,
                lower_rune,
                components::DL_MAP,
            );
        }

        let mut render = Render::new(mock_renderer);

        render.run_now(&world);
        world.maintain();

        assert_eq!(render.renderer.drew_rune.unwrap(), upper_rune);
        assert_eq!(render.renderer.drew_count, 1);
    }
}
