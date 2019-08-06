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
    pub fn draw(&self, blocks: &[Block]) {
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

    // Draw directly from the hashmap!
    pub fn draw_block(&self, to_draw: &Board) {
        let length = self.settings.size.0 as usize * self.settings.size.1 as usize;
        let mut board: Vec<Block> = vec![
            Block {
                block_type: BlockType::Empty,
                active: false,
            };
            length
        ];

        for (pos, blk) in &to_draw.blocks {
            board[(pos.x + pos.y * self.settings.size.0 as i32) as usize] = *blk;
        }

        self.draw(&board);
    }
}
