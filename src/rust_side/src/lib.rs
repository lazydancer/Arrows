use logic;
use view;

use logic::{Block, BlockType, Board, Direction, Pos};

#[no_mangle]
pub extern "C" fn board_new() -> *mut logic::Board {
    let board = Box::new(logic::Board::new());
    Box::into_raw(board)
}

#[no_mangle]
pub extern "C" fn board_free(ptr: *mut logic::Board) {
    println!("Freeing the board");
    if ptr.is_null() {
        return;
    }
    println!("{:?}", ptr);
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

    println!("{:?}", board.blocks);

    Box::into_raw(board); // Don't free the board

    println!("After into raw")
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
// #[no_mangle]
// pub extern "C" fn doubl(x: i32) -> i32 {
//     x * 2
// }

// #[no_mangle]
// pub extern "C" fn start_sim() {
//     println!("{:?}", "starting sim");
//     view::start();
// }

// #[repr(C)]
// pub struct Pos {
//     pub x: i32,
//     pub y: i32,
// }

// #[no_mangle]
// pub extern "C" fn length(ptr: *const Pos) -> i32 {
//     let pos = unsafe {
//         assert!(!ptr.is_null());
//         &*ptr
//     };
//     4
// }

// #[no_mangle]
// pub extern "C" fn add_block(x: i32, y: i32, block_number: i32) {
//     print("Testing");
// }
