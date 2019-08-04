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

use logic::{Block, BlockType, Direction, Logic, Pos};

type Point2 = na::Point2<f32>;

const ICON_SIZE: i32 = 16;

fn radians(dir: Direction) -> f32 {
    // 90 Deg Rotation
    let turn = 3.14159 / 2.0;

    match dir {
        Direction::Right => 0.0,
        Direction::Down => turn * 1.0,
        Direction::Left => turn * 2.0,
        Direction::Up => turn * 3.0,
    }
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

    fn image(&mut self, block: &Block) -> &mut graphics::Image {
        match block.block_type {
            BlockType::Arrow(_) => {
                if block.active {
                    &mut self.arrow_active
                } else {
                    &mut self.arrow_inactive
                }
            }
            BlockType::NotArrow(_) => {
                if block.active {
                    &mut self.invert_active
                } else {
                    &mut self.invert_inactive
                }
            }
            BlockType::Split(_) => {
                if block.active {
                    &mut self.split_active
                } else {
                    &mut self.split_inactive
                }
            }
            BlockType::Empty => &mut self.split_active,
        }
    }
}

fn draw_arrow(
    assets: &mut Assets,
    ctx: &mut Context,
    arrow_coords: Point2,
    arrow: Block,
) -> GameResult {
    let image = assets.image(&arrow);
    let rotation = radians(arrow.get_direction());

    let drawparams = graphics::DrawParam::new()
        .dest(arrow_coords)
        .rotation(rotation)
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

struct MainState {
    logic: Logic,
    assets: Assets,
    display_size: (i32, i32),
    view_top_left: (i32, i32),
}

impl MainState {
    /// Load images and create meshes.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut logic = Logic::new();
        logic.set();

        let assets = Assets::new(ctx)?;

        let (width, height) = graphics::drawable_size(ctx);
        let display_size = (width as i32, height as i32);
        let view_top_left = (-1, -1);

        let s = MainState {
            logic,
            assets,
            display_size,
            view_top_left,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 2;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.logic.step();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let assets = &mut self.assets;

        for (pos, block) in self.logic.get_arrows() {
            let coord =
                pos_to_screen_coords(&self.display_size, &self.view_top_left, (pos.x, pos.y));
            if let Some(coord) = coord {
                draw_arrow(assets, ctx, coord, block);
            }
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
