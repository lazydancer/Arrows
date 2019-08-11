use std::env;
use std::path;
use std::time::Instant;

use cgmath;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};

use logic::{Block, BlockType, Board, Direction, Pos};

type Point2 = na::Point2<f32>;

const ICON_SIZE: i32 = 16;

pub fn start(board: Board) -> GameResult {
    // let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     path
    // } else {
    //     println!("Could not find path in CARGO_MANIFEST_DIR");
    //     //path::PathBuf::from("./resources")
    //     path::PathBuf::from("home/james/Dropbox/Arrows/src/View/resources")
    // };

    let resource_dir = path::PathBuf::from("/home/james/Dropbox/Arrows/src/view/resources");

    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx, board).unwrap();
    event::run(ctx, events_loop, state)
}

struct Assets {
    spritebatch: graphics::spritebatch::SpriteBatch,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let image = graphics::Image::new(ctx, "/spritesheet.png")?;
        let spritebatch = graphics::spritebatch::SpriteBatch::new(image);

        Ok(Assets { spritebatch })
    }

    fn draw_block(&mut self, block: &Block, coord: Point2) {
        let image_rect = self.spritesheet_loc(&block);
        let drawparams = graphics::DrawParam::new().src(image_rect).dest(coord);
        self.spritebatch.add(drawparams);
    }

    fn spritesheet_loc(&self, block: &Block) -> graphics::Rect {
        let (x, y) = match block.block_type {
            BlockType::Arrow(dir) => {
                if block.active {
                    match dir {
                        Direction::Up => (0.0, 0.0),
                        Direction::Right => (0.25, 0.0),
                        Direction::Down => (0.50, 0.0),
                        Direction::Left => (0.75, 0.0),
                    }
                } else {
                    match dir {
                        Direction::Up => (0.0, 0.20),
                        Direction::Right => (0.25, 0.20),
                        Direction::Down => (0.50, 0.20),
                        Direction::Left => (0.75, 0.20),
                    }
                }
            }

            BlockType::NotArrow(dir) => {
                if block.active {
                    match dir {
                        Direction::Up => (0.0, 0.40),
                        Direction::Right => (0.25, 0.40),
                        Direction::Down => (0.50, 0.40),
                        Direction::Left => (0.75, 0.40),
                    }
                } else {
                    match dir {
                        Direction::Up => (0.0, 0.60),
                        Direction::Right => (0.25, 0.60),
                        Direction::Down => (0.50, 0.60),
                        Direction::Left => (0.75, 0.60),
                    }
                }
            }

            BlockType::Split(dir) => {
                if block.active {
                    match dir {
                        Direction::Up => (0.25, 0.80),
                        Direction::Right => (0.0, 0.80),
                        Direction::Down => (0.25, 0.80),
                        Direction::Left => (0.0, 0.80),
                    }
                } else {
                    match dir {
                        Direction::Up => (0.75, 0.80),
                        Direction::Right => (0.5, 0.80),
                        Direction::Down => (0.75, 0.80),
                        Direction::Left => (0.5, 0.80),
                    }
                }
            }

            BlockType::Empty => (0.0, 0.0),
        };

        graphics::Rect::new(x, y, 0.25, 0.20)
    }
}

fn pos_to_screen(
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

pub struct MainState {
    board: Board,
    assets: Assets,
    display_size: (i32, i32),
    view_top_left: (i32, i32),
}

impl MainState {
    /// Load images and create meshes.
    pub fn new(ctx: &mut Context, board: Board) -> GameResult<MainState> {
        //let mut board = Board::new();
        let mut board = board;

        let assets = Assets::new(ctx)?;

        let (width, height) = graphics::drawable_size(ctx);
        let display_size = (width as i32, height as i32);
        let view_top_left = (-1, -1);

        let s = MainState {
            board,
            assets,
            display_size,
            view_top_left,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 3;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.board.step();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        graphics::clear(ctx, graphics::BLACK);

        let assets = &mut self.assets;

        for (pos, block) in self.board.get_arrows() {
            let coord = pos_to_screen(&self.display_size, &self.view_top_left, (pos.x, pos.y));
            if let Some(coord) = coord {
                assets.draw_block(&block, coord);
            }
        }

        let parm = graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
        graphics::draw(ctx, &assets.spritebatch, parm)?;
        assets.spritebatch.clear();

        graphics::present(ctx)?;
        println!("{}", now.elapsed().subsec_nanos());
        Ok(())
    }
}
