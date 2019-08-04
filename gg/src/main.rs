use std::env;
use std::path;

use cgmath;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};

type Point2 = na::Point2<f32>;

const ICON_SIZE: i32 = 16;

enum Cast {
    Arrow,
    Invert,
    Split,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Arrow {
    cast: Cast,
    direction: Direction,
    active: bool,
}

struct Assets {
    arrow_active: graphics::Image,
    arrow_inactive: graphics::Image,
    invert_active: graphics::Image,
    invert_inactive: graphics::Image,
    split_active: graphics::Image,
    split_inactive: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let arrow_active = graphics::Image::new(ctx, "/arrow_active.png")?;
        let arrow_inactive = graphics::Image::new(ctx, "/arrow_inactive.png")?;
        let invert_active = graphics::Image::new(ctx, "/invert_active.png")?;
        let invert_inactive = graphics::Image::new(ctx, "/invert_inactive.png")?;
        let split_active = graphics::Image::new(ctx, "/split_active.png")?;
        let split_inactive = graphics::Image::new(ctx, "/split_inactive.png")?;

        Ok(Assets {
            arrow_active,
            arrow_inactive,
            invert_active,
            invert_inactive,
            split_active,
            split_inactive,
        })
    }

    fn image(&mut self, arrow: Arrow) -> &mut graphics::Image {
        match arrow {
            Arrow {
                cast: Cast::Arrow,
                active: true,
                ..
            } => &mut self.arrow_active,
            Arrow {
                cast: Cast::Arrow,
                active: false,
                ..
            } => &mut self.arrow_inactive,
            Arrow {
                cast: Cast::Invert,
                active: true,
                ..
            } => &mut self.invert_active,
            Arrow {
                cast: Cast::Invert,
                active: true,
                ..
            } => &mut self.invert_inactive,
            Arrow {
                cast: Cast::Split,
                active: true,
                ..
            } => &mut self.split_active,
            Arrow {
                cast: Cast::Split,
                active: true,
                ..
            } => &mut self.split_inactive,
            _ => &mut self.arrow_active,
        }
    }
}

struct MainState {
    assets: Assets,
    display_size: (i32, i32),
    view_top_left: (i32, i32),
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;

        let (width, height) = graphics::drawable_size(ctx);
        let display_size = (width as i32, height as i32);
        let view_top_left = (-10, -10);

        let s = MainState {
            assets,
            display_size,
            view_top_left,
        };

        Ok(s)
    }
}

fn draw_arrow(
    assets: &mut Assets,
    ctx: &mut Context,
    arrow_coords: Point2,
    arrow_type: u8,
) -> GameResult {
    let image = assets.image(Arrow {
        cast: Cast::Arrow,
        direction: Direction::Up,
        active: true,
    });
    let drawparams = graphics::DrawParam::new()
        .dest(arrow_coords)
        .rotation(3.14159 / 2.0)
        .offset(Point2::new(0.5, 0.5));
    graphics::draw(ctx, image, drawparams)
}

fn pos_to_screen_coords(
    window_size: &(i32, i32),
    view_top_left: &(i32, i32),
    pos: (i32, i32),
) -> Option<Point2> {
    // First translate in-game view, make view top left to 0,0
    let pos = (pos.0 - view_top_left.0, pos.1 - view_top_left.1);

    // Then 'grow' in-game position to window size
    let pos = (pos.0 * ICON_SIZE, pos.1 * ICON_SIZE);

    if pos.0 + ICON_SIZE > window_size.0 {
        return None;
    }

    if pos.1 + ICON_SIZE > window_size.1 {
        return None;
    }

    Some(Point2::new(pos.0 as f32, pos.1 as f32))
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let assets = &mut self.assets;

        let coord = pos_to_screen_coords(&self.display_size, &self.view_top_left, (1, 1));
        if let Some(coord) = coord {
            draw_arrow(assets, ctx, coord, 1);
        }
        // Finished drawing, show it all on the screen!
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, events_loop, state)
}
