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
    spritebatch: graphics::spritebatch::SpriteBatch,

    arrow_up_active: graphics::Image,
    arrow_right_active: graphics::Image,
    arrow_down_active: graphics::Image,
    arrow_left_active: graphics::Image,
    arrow_up_inactive: graphics::Image,
    arrow_right_inactive: graphics::Image,
    arrow_down_inactive: graphics::Image,
    arrow_left_inactive: graphics::Image,
    invert_up_active: graphics::Image,
    invert_right_active: graphics::Image,
    invert_down_active: graphics::Image,
    invert_left_active: graphics::Image,
    invert_up_inactive: graphics::Image,
    invert_right_inactive: graphics::Image,
    invert_down_inactive: graphics::Image,
    invert_left_inactive: graphics::Image,
    split_horizontal_active: graphics::Image,
    split_vertical_active: graphics::Image,
    split_horizontal_inactive: graphics::Image,
    split_vertical_inactive: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let arrow_up_active = graphics::Image::new(ctx, "/arrow_up_active.png")?;
        let arrow_right_active = graphics::Image::new(ctx, "/arrow_right_active.png")?;
        let arrow_down_active = graphics::Image::new(ctx, "/arrow_down_active.png")?;
        let arrow_left_active = graphics::Image::new(ctx, "/arrow_left_active.png")?;

        let arrow_up_inactive = graphics::Image::new(ctx, "/arrow_up_inactive.png")?;
        let arrow_right_inactive = graphics::Image::new(ctx, "/arrow_right_inactive.png")?;
        let arrow_down_inactive = graphics::Image::new(ctx, "/arrow_down_inactive.png")?;
        let arrow_left_inactive = graphics::Image::new(ctx, "/arrow_left_inactive.png")?;

        let invert_up_active = graphics::Image::new(ctx, "/invert_up_active.png")?;
        let invert_right_active = graphics::Image::new(ctx, "/invert_right_active.png")?;
        let invert_down_active = graphics::Image::new(ctx, "/invert_down_active.png")?;
        let invert_left_active = graphics::Image::new(ctx, "/invert_left_active.png")?;

        let invert_up_inactive = graphics::Image::new(ctx, "/invert_up_inactive.png")?;
        let invert_down_inactive = graphics::Image::new(ctx, "/invert_down_inactive.png")?;
        let invert_right_inactive = graphics::Image::new(ctx, "/invert_right_inactive.png")?;
        let invert_left_inactive = graphics::Image::new(ctx, "/invert_left_inactive.png")?;

        let split_horizontal_active = graphics::Image::new(ctx, "/split_horizontal_active.png")?;
        let split_vertical_active = graphics::Image::new(ctx, "/split_vertical_active.png")?;

        let split_horizontal_inactive =
            graphics::Image::new(ctx, "/split_horizontal_inactive.png")?;
        let split_vertical_inactive = graphics::Image::new(ctx, "/split_vertical_inactive.png")?;

        let image = graphics::Image::new(ctx, "/spritesheet.png")?;
        let spritebatch = graphics::spritebatch::SpriteBatch::new(image);

        Ok(Assets {
            spritebatch,
            arrow_up_active,
            arrow_right_active,
            arrow_down_active,
            arrow_left_active,
            arrow_up_inactive,
            arrow_right_inactive,
            arrow_down_inactive,
            arrow_left_inactive,
            invert_up_active,
            invert_right_active,
            invert_down_active,
            invert_left_active,
            invert_up_inactive,
            invert_right_inactive,
            invert_down_inactive,
            invert_left_inactive,
            split_horizontal_active,
            split_vertical_active,
            split_horizontal_inactive,
            split_vertical_inactive,
        })
    }

    fn image(&mut self, block: &Block) -> &mut graphics::Image {
        match block.block_type {
            BlockType::Arrow(dir) => {
                if block.active {
                    match dir {
                        Direction::Up => &mut self.arrow_up_active,
                        Direction::Right => &mut self.arrow_right_active,
                        Direction::Down => &mut self.arrow_down_active,
                        Direction::Left => &mut self.arrow_left_active,
                    }
                } else {
                    match dir {
                        Direction::Up => &mut self.arrow_up_inactive,
                        Direction::Right => &mut self.arrow_right_inactive,
                        Direction::Down => &mut self.arrow_down_inactive,
                        Direction::Left => &mut self.arrow_left_inactive,
                    }
                }
            }

            BlockType::NotArrow(dir) => {
                if block.active {
                    match dir {
                        Direction::Up => &mut self.invert_up_active,
                        Direction::Right => &mut self.invert_right_active,
                        Direction::Down => &mut self.invert_down_active,
                        Direction::Left => &mut self.invert_left_active,
                    }
                } else {
                    match dir {
                        Direction::Up => &mut self.invert_up_inactive,
                        Direction::Right => &mut self.invert_right_inactive,
                        Direction::Down => &mut self.invert_down_inactive,
                        Direction::Left => &mut self.invert_left_inactive,
                    }
                }
            }

            BlockType::Split(dir) => {
                if block.active {
                    match dir {
                        Direction::Up => &mut self.split_vertical_active,
                        Direction::Right => &mut self.split_horizontal_active,
                        Direction::Down => &mut self.split_vertical_active,
                        Direction::Left => &mut self.split_horizontal_active,
                    }
                } else {
                    match dir {
                        Direction::Up => &mut self.split_vertical_inactive,
                        Direction::Right => &mut self.split_horizontal_inactive,
                        Direction::Down => &mut self.split_vertical_inactive,
                        Direction::Left => &mut self.split_horizontal_inactive,
                    }
                }
            }

            BlockType::Empty => &mut self.split_vertical_active,
        }
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
                        Direction::Up => (0.75, 0.0),
                        Direction::Right => (0.5, 0.0),
                        Direction::Down => (0.75, 0.0),
                        Direction::Left => (0.5, 0.0),
                    }
                }
            }

            BlockType::Empty => (0.0, 0.0),
        };

        graphics::Rect::new(x, y, 0.25, 0.20)
    }
}

fn draw_arrow(
    assets: &mut Assets,
    ctx: &mut Context,
    arrow_coords: Point2,
    arrow: Block,
) -> GameResult {
    let image = assets.image(&arrow);

    let drawparams = graphics::DrawParam::new()
        .dest(arrow_coords)
        .offset(Point2::new(0.5, 0.5));
    graphics::draw(ctx, image, drawparams)
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
        const DESIRED_FPS: u32 = 5;

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
                let image_rect = assets.spritesheet_loc(&block);
                let drawparams = graphics::DrawParam::new().src(image_rect).dest(coord);
                assets.spritebatch.add(drawparams);
                //draw_arrow(assets, ctx, coord, block);
            }
        }

        let parm = graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
        graphics::draw(ctx, &assets.spritebatch, parm)?;

        // Finished drawing, show it all on the screen!
        graphics::present(ctx)?;
        println!("{}", now.elapsed().subsec_nanos());
        Ok(())
    }
}
