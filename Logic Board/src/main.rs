mod block;
mod board;
mod view;

use crate::block::{Block, BlockType, Direction};
use crate::board::{Board, Pos};
use crate::view::{View, ViewSettings};

fn main() {
    let mut board = Board::new();

    let view = View {
        settings: ViewSettings { size: (3, 3) },
    };

    board.set(
        Block::new(BlockType::NotArrow(Direction::Right)),
        Pos { x: 0, y: 2 },
    );
    board.set(
        Block::new(BlockType::Arrow(Direction::Right)),
        Pos { x: 1, y: 2 },
    );
    board.set(
        Block::new(BlockType::Split(Direction::Up)),
        Pos { x: 2, y: 2 },
    );

    view.draw_block(&board);

    for _ in 0..10 {
        board.step();
        view.draw_block(&board);
    }
}
