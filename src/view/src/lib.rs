use std::path;

use ggez;
use ggez::conf;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use logic::{Block, BlockType, Board, Direction, Pos};

mod assets;
use crate::assets::{Assets, Point2};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const ICON_SIZE: i32 = 16;

pub fn start(board: Board) -> GameResult {
    let resource_dir = path::PathBuf::from("/home/james/Dropbox/Arrows/src/view/resources");

    let cb = ggez::ContextBuilder::new("drawing", "ggez")
        .window_setup(conf::WindowSetup::default().title("Arrows!"))
        .window_mode(conf::WindowMode::default().min_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);
    let state = &mut MainState::new(ctx, board).unwrap();
    event::run(ctx, events_loop, state)
}

pub struct MainState {
    board: Board,
    assets: Assets,
    display_size: (i32, i32),
    view_top_left: (i32, i32),
    clicked_arrow: Option<Block>,
}

impl MainState {
    pub fn new(ctx: &mut Context, board: Board) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;

        let (width, height) = graphics::drawable_size(ctx);
        let display_size = (width as i32, height as i32);
        let view_top_left = (-1, -1);
        let clicked_arrow = None;

        let s = MainState {
            board,
            assets,
            display_size,
            view_top_left,
            clicked_arrow,
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
        graphics::clear(ctx, graphics::BLACK);

        let assets = &mut self.assets;

        for (pos, block) in self.board.get_arrows() {
            let coord = pos_to_screen(self.display_size, self.view_top_left, (pos.x, pos.y));
            if let Some(coord) = coord {
                assets.draw_block(block, coord);
            }
        }

        let parm = graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
        graphics::draw(ctx, &assets.spritebatch, parm)?;
        assets.spritebatch.clear();

        assets.draw_toolbelt(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => {
                self.view_top_left.1 -= 1;
            }
            KeyCode::A => {
                self.view_top_left.0 -= 1;
            }
            KeyCode::S => {
                self.view_top_left.1 += 1;
            }
            KeyCode::D => {
                self.view_top_left.0 += 1;
            }
            KeyCode::Q => {
                self.clicked_arrow = None;
            }
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if let Some(x) = toolbar_item(Point2::new(x, y)) {
            println!("{:?}", x);
            self.clicked_arrow = Some(x);
            return;
        }

        let pos = screen_to_pos(self.display_size, self.view_top_left, Point2::new(x, y));
        let pos = Pos { x: pos.0, y: pos.1 };

        if let Some(x) = self.clicked_arrow {
            self.board.set(x, pos);
        } else {
            self.board.set(Block::new(BlockType::Empty), pos);
        }
    }
}

fn pos_to_screen(
    window_size: (i32, i32),
    view_top_left: (i32, i32),
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

fn screen_to_pos(window_size: (i32, i32), view_top_left: (i32, i32), point: Point2) -> (i32, i32) {
    let x = point.x / ICON_SIZE as f32;
    let y = point.y / ICON_SIZE as f32;

    let x = x + view_top_left.0 as f32;
    let y = y + view_top_left.0 as f32;

    return (x as i32, y as i32);
}

fn toolbar_item(pos: Point2) -> Option<Block> {
    println!("{:?}", pos);

    if pos.x > 372.0 && pos.x < 388.0 && pos.y > 568.0 && pos.y < 584.0 {
        return Some(Block::new(BlockType::Arrow(Direction::Right)));
    }
    if pos.x > 390.0 && pos.x < 406.0 && pos.y > 568.0 && pos.y < 584.0 {
        return Some(Block::new(BlockType::NotArrow(Direction::Right)));
    }
    if pos.x > 408.0 && pos.x < 424.0 && pos.y > 568.0 && pos.y < 584.0 {
        return Some(Block::new(BlockType::Split(Direction::Up)));
    }

    return None;
}
