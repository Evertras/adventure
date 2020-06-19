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
    fn draw_at(&mut self, x: u16, y: u16, fg: &Color, bg: &Color, rune: char);

    fn size(&self) -> (u16, u16);

    fn flush(&mut self);
}

pub struct Render<T: Renderer> {
    renderer: T,
}

impl<T: Renderer> Render<T> {
    pub fn new(renderer: T) -> Render<T> {
        Render { renderer }
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

        for (pos, draw) in (&pos, &draw).join() {
            let tile_x = if pos.x < 0.0 {
                (pos.x - 1.0) as i32
            } else {
                pos.x as i32
            };
            let tile_y = if pos.y < 0.0 {
                (pos.y - 1.0) as i32
            } else {
                pos.y as i32
            };

            if tile_x < min_x || tile_x >= max_x || tile_y < min_y || tile_y >= max_y {
                continue;
            }

            self.renderer.draw_at(
                (tile_x - offset_x) as u16,
                (tile_y - offset_y) as u16,
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

        self.renderer.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::{Builder, RunNow, World, WorldExt};

    struct MockRenderer {
        width: u16,
        height: u16,

        drew_at_x: Option<u16>,
        drew_at_y: Option<u16>,
        drew_fg: Option<Color>,
        drew_bg: Option<Color>,
        drew_rune: Option<char>,
        drew_count: u16,
        flush_count: u16,
    }

    impl MockRenderer {
        fn new(width: u16, height: u16) -> MockRenderer {
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
        fn draw_at(&mut self, x: u16, y: u16, fg: &Color, bg: &Color, rune: char) {
            self.drew_at_x = Some(x);
            self.drew_at_y = Some(y);
            self.drew_fg = Some(fg.clone());
            self.drew_bg = Some(bg.clone());
            self.drew_rune = Some(rune);
            self.drew_count += 1;
        }

        fn size(&self) -> (u16, u16) {
            (self.width, self.height)
        }

        fn flush(&mut self) {
            self.flush_count += 1;
        }
    }

    #[test]
    fn draws() {
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
        let expected_x = 1;
        let expected_y = 1;
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

                    rune,
                })
                .build()
        };

        // This should be visible
        spawn(visible_x, visible_y);

        // Off to the right
        spawn(5.0, visible_y);

        // Off to the left
        spawn(-0.001, visible_y);

        // Above
        spawn(visible_x, -46.9);

        // Below
        spawn(visible_x, -52.00001);

        let mut render = Render {
            renderer: mock_renderer,
        };

        render.run_now(&world);
        world.maintain();

        // Should only have one thing drawn, the others should be culled
        assert_eq!(render.renderer.drew_count, 1);

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

        assert_eq!(render.renderer.flush_count, 1);
    }
}
