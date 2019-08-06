use std::collections::HashMap;

mod block;
mod board;
mod view;

use crate::board::Board;
use crate::view::{View, ViewSettings};

pub use crate::block::{Block, BlockType, Direction};
pub use crate::board::Pos;

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
        self.board.set(
            Block::new(BlockType::Arrow(Direction::Down)),
            Pos { x: 1, y: 1 },
        );
        self.board.set(
            Block::new(BlockType::Arrow(Direction::Down)),
            Pos { x: 1, y: 2 },
        );
        self.board.set(
            Block::new(BlockType::Split(Direction::Right)),
            Pos { x: 1, y: 3 },
        );
        self.board.set(
            Block::new(BlockType::NotArrow(Direction::Down)),
            Pos { x: 2, y: 3 },
        );
        self.board.set(
            Block::new(BlockType::Split(Direction::Up)),
            Pos { x: 0, y: 3 },
        );
        self.board.set(
            Block::new(BlockType::Arrow(Direction::Up)),
            Pos { x: 0, y: 2 },
        );
        self.board.set(
            Block::new(BlockType::Arrow(Direction::Down)),
            Pos { x: 0, y: 4 },
        );
    }

    pub fn step(&mut self) {
        self.board.step();
    }

    pub fn get_arrows(&self) -> HashMap<Pos, Block> {
        self.board.blocks.clone()
    }
}
