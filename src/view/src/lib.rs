use std::path;

use ggez::event::{self, KeyCode, KeyMods, MouseButton, EventHandler};
use ggez::{conf, timer, graphics, ContextBuilder, GameResult, Context};

use glam::Vec2;

use logic::{Block, BlockType, Board, Direction, Pos};

mod assets;
use crate::assets::{Assets};


const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const ICON_SIZE: i32 = 16;


// pub fn start(_: Board) {
//     // Make a Context.
//     let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
//         .build()
//         .expect("aieee, could not create ggez context!");

//     // Create an instance of your event handler.
//     // Usually, you should provide it with the Context object to
//     // use when setting your game up.
//     let mut my_game = MyGame::new(&mut ctx);

//     // Run!
//     match event::run(&mut ctx, &mut event_loop, &mut my_game) {
//         Ok(_) => println!("Exited cleanly."),
//         Err(e) => println!("Error occured: {}", e)
//     }
// }

// struct MyGame {
//     // Your state here...
// }

// impl MyGame {
//     pub fn new(_ctx: &mut Context) -> MyGame {
//         // Load/create resources such as images here.
//         MyGame {
//             // ...
//         }
//     }
// }

// impl EventHandler for MyGame {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
//         // Update code here...
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
//         graphics::clear(ctx, graphics::WHITE);
//         // Draw code here...
//         graphics::present(ctx)
//     }
// }

pub fn start(board: Board) -> GameResult {
    let resource_dir = path::PathBuf::from("/home/james/Projects/Arrows/src/view/resources");

    // let cb = ggez::ContextBuilder::new("drawing", "ggez")
    //     .window_setup(conf::WindowSetup::default().title("Arrows!"))
    //     .window_mode(conf::WindowMode::default().min_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
    //     .add_resource_path(resource_dir);

    // let (ctx, events_loop) = &mut cb.build()?;

    // println!("{}", graphics::renderer_info(ctx)?);
    // let state = &mut MainState::new(ctx, board).unwrap();
    // event::run(ctx, events_loop, state)


    let (mut ctx, mut event_loop) = ContextBuilder::new("arrows", "jp")
        .add_resource_path(resource_dir)
        .build()
        .expect("could not create ggez context!");

    let mut state = MainState::new(&mut ctx, board);
        // .expect("could not create state!");

    event::run(ctx, event_loop, state)
}

pub struct MainState {
    board: Board,
    assets: Assets,
    display_size: (i32, i32),
    view_top_left: (i32, i32),
    clicked_arrow: Option<Block>,
}

impl MainState {
    pub fn new(ctx: &mut Context, board: Board) -> MainState {
        let assets = Assets::new(ctx)
            .expect("could not load assets");

        let (width, height) = graphics::drawable_size(ctx);
        let display_size = (width as i32, height as i32);
        let view_top_left = (-1, -1);
        let clicked_arrow = None;

        MainState {
            board,
            assets,
            display_size,
            view_top_left,
            clicked_arrow,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 3;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.board.step();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        let assets = &mut self.assets;

        for (pos, block) in self.board.get_arrows() {
            let coord = pos_to_screen(self.display_size, self.view_top_left, (pos.x, pos.y));
            if let Some(coord) = coord {
                assets.draw_block(block, coord);
            }
        }

        let parm = graphics::DrawParam::new().dest([0.0, 0.0]);
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
            KeyCode::R => {
                if let Some(x) = self.clicked_arrow {
                    self.clicked_arrow = Some(x.rotate());
                }
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
        if let Some(x) = toolbar_item(Vec2::new(x, y)) {
            self.clicked_arrow = Some(x);
            return;
        }

        let pos = screen_to_pos(self.view_top_left, Vec2::new(x, y));
        let pos = Pos { x: pos.0, y: pos.1 };

        println!("{:?}", pos);

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
) -> Option<[f32; 2]> {
    let pos = (pos.0 - view_top_left.0, pos.1 - view_top_left.1);
    let pos = (pos.0 * ICON_SIZE, pos.1 * ICON_SIZE);

    if pos.0 + ICON_SIZE > window_size.0 {
        return None;
    }

    if pos.1 + ICON_SIZE > window_size.1 {
        return None;
    }

    Some([pos.0 as f32, pos.1 as f32])
}

fn screen_to_pos(view_top_left: (i32, i32), point: Vec2) -> (i32, i32) {
    let x = point.x / ICON_SIZE as f32;
    let y = point.y / ICON_SIZE as f32;

    let x = x + view_top_left.0 as f32;
    let y = y + view_top_left.1 as f32;

    (x as i32, y as i32)
}

fn toolbar_item(pos: Vec2) -> Option<Block> {
    if pos.x > 372.0 && pos.x < 388.0 && pos.y > 568.0 && pos.y < 584.0 {
        return Some(Block::new(BlockType::Arrow(Direction::Right)));
    }
    if pos.x > 390.0 && pos.x < 406.0 && pos.y > 568.0 && pos.y < 584.0 {
        return Some(Block::new(BlockType::NotArrow(Direction::Right)));
    }
    if pos.x > 408.0 && pos.x < 424.0 && pos.y > 568.0 && pos.y < 584.0 {
        return Some(Block::new(BlockType::Split(Direction::Up)));
    }

    None
}
