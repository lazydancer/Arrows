use ggez::graphics::{self, Color, DrawParam};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use logic::{Block, BlockType, Direction};

pub type Point2 = na::Point2<f32>;

pub struct Assets {
    pub spritebatch: graphics::spritebatch::SpriteBatch,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let image = graphics::Image::new(ctx, "/spritesheet.png")?;
        let spritebatch = graphics::spritebatch::SpriteBatch::new(image);

        Ok(Assets { spritebatch })
    }

    pub fn draw_block(&mut self, block: Block, coord: Point2) {
        let image_rect = Self::spritesheet_loc(block);
        let drawparams = graphics::DrawParam::new().src(image_rect).dest(coord);
        self.spritebatch.add(drawparams);
    }

    pub fn draw_toolbelt(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Rect::new(370.0, 566.0, 56.0, 20.0);
        let r1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            Color {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 1.0,
            },
        )?;
        graphics::draw(ctx, &r1, DrawParam::default())?;

        self.draw_block(
            Block::new(BlockType::Arrow(Direction::Right)),
            Point2::new(372.0, 568.0),
        );
        self.draw_block(
            Block::new(BlockType::NotArrow(Direction::Right)),
            Point2::new(390.0, 568.0),
        );
        self.draw_block(
            Block::new(BlockType::Split(Direction::Up)),
            Point2::new(408.0, 568.0),
        );

        let parm = graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
        graphics::draw(ctx, &self.spritebatch, parm)?;
        self.spritebatch.clear();

        Ok(())
    }

    fn spritesheet_loc(block: Block) -> graphics::Rect {
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
