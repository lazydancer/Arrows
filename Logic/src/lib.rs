mod block;
mod board;
mod view;

use crate::block::{Block, BlockType, Direction};
use crate::board::{Board, Pos};
use crate::view::{View, ViewSettings};

pub struct Logic {
    board: Board,
}

impl Logic {
    pub fn new() -> Self {
        Logic {
            board: Board::new(),
        }
    }

    pub fn say_hi() {
        println!("Hello From Logic");
    }

    pub fn set(&mut self) {
        self.board.set(
            Block::new(BlockType::NotArrow(Direction::Right)),
            Pos { x: 0, y: 1 },
        );
    }

    pub fn step(&mut self) {
        self.board.step()
    }
}
