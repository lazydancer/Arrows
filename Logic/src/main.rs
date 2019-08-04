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
        Pos { x: 0, y: 1 },
    );
    board.set(
        Block::new(BlockType::Arrow(Direction::Right)),
        Pos { x: 1, y: 1 },
    );
    view.draw_block(&board);


    board.step();



    view.draw_block(&board);

}
