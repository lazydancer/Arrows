mod block;
mod board;
mod view;

use crate::block::{Block, BlockType, Direction};
use crate::board::Board;
use crate::view::{View, ViewSettings};

fn main() {
    let mut board = Board::new();

    let view = View {
        settings: ViewSettings { size: (3, 3) },
    };

    board.set(Block::new(BlockType::NotArrow(Direction::Right)), (0, 0));
    board.set(Block::new(BlockType::Arrow(Direction::Right)), (1, 0));

    view.draw_block(&board);

    for _ in 0..20 {
        board.step();
        view.draw_block(&board);
    }
}
