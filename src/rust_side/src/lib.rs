use logic;
use view;

use logic::{Block, BlockType, Board, Direction, Pos};

#[no_mangle]
pub extern "C" fn board_new() -> *mut logic::Board {
    let board = Box::new(logic::Board::default());
    Box::into_raw(board)
}

#[no_mangle]
pub extern "C" fn board_free(ptr: *mut logic::Board) {
    println!("Freeing the board");
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn board_add_block(board: *mut Board, x: i32, y: i32, block_type: i32) {
    if board.is_null() {
        return;
    }
    let mut board = unsafe { Box::from_raw(board) };

    let block = python_block_to_rust(block_type);

    board.set(block, Pos { x, y });

    Box::into_raw(board); // Don't free the board
}

#[no_mangle]
pub extern "C" fn board_start(board: *mut Board) {
    if board.is_null() {
        return;
    }
    let mut board = unsafe { Box::from_raw(board) };

    view::start(*board.clone()); // Cloning on an init, see how it works out

    Box::into_raw(board);
}

fn python_block_to_rust(x: i32) -> Block {
    match x {
        0 => Block::new(BlockType::Arrow(Direction::Left)),
        1 => Block::new(BlockType::Arrow(Direction::Up)),
        2 => Block::new(BlockType::Arrow(Direction::Right)),
        3 => Block::new(BlockType::Arrow(Direction::Down)),
        4 => Block::new(BlockType::NotArrow(Direction::Left)),
        5 => Block::new(BlockType::NotArrow(Direction::Up)),
        6 => Block::new(BlockType::NotArrow(Direction::Right)),
        7 => Block::new(BlockType::NotArrow(Direction::Down)),
        8 => Block::new(BlockType::Split(Direction::Left)),
        9 => Block::new(BlockType::Split(Direction::Up)),
        10 => Block::new(BlockType::Split(Direction::Right)),
        11 => Block::new(BlockType::Split(Direction::Down)),
        12 => Block::new(BlockType::Empty),
        _ => panic!("{:?} is out of known range", x),
    }
}
