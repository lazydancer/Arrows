extern crate colored;

use colored::*;

use crate::block::{Block, BlockType, Direction};
use crate::board::Board;

/// Stores board view settings
pub struct ViewSettings {
    pub size: (u32, u32),
}

/// Stores visual information
pub struct View {
    pub settings: ViewSettings,
}

impl View {
    /// Draws board
    pub fn draw(&self, board: &Board) {
        let blocks = &board.board;

        let mut draw_block: Vec<ColoredString> = Vec::new();

        for blk in blocks {
            let cara = match blk.block_type {
                BlockType::Empty => ' ',
                BlockType::Arrow(Direction::Up) => '↑',
                BlockType::Arrow(Direction::Right) => '→',
                BlockType::Arrow(Direction::Down) => '↓',
                BlockType::Arrow(Direction::Left) => '←',
                BlockType::NotArrow(Direction::Up) => '^',
                BlockType::NotArrow(Direction::Right) => '>',
                BlockType::NotArrow(Direction::Down) => 'v',
                BlockType::NotArrow(Direction::Left) => '<',
                BlockType::Split(Direction::Left) => '↔',
                BlockType::Split(Direction::Right) => '↔',
                BlockType::Split(Direction::Up) => '↕',
                BlockType::Split(Direction::Down) => '↕',
            };

            let cara = if blk.active {
                cara.to_string().yellow()
            } else {
                cara.to_string().normal()
            };

            draw_block.push(cara);
        }

        for (i, blk) in draw_block.iter().enumerate() {
            if i % self.settings.size.0 as usize == 0 {
                print!("\n ");
            }

            print!("{}", blk);
        }

        println!()
    }
}
