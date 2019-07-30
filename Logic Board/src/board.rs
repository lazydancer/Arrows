use std::collections::HashMap;

use crate::block::{Block, BlockType, Direction};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn neighbour(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Pos {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Pos {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub blocks: HashMap<Pos, Block>,
    pub modified: Vec<(Pos, Block)>,
}

impl Board {
    /// Create a new board full of Empty and empty modified
    pub fn new() -> Self {
        let blocks = HashMap::new();
        let modified = vec![];

        Board { blocks, modified }
    }
    /// Set the block on the board
    pub fn set(&mut self, block: Block, loc: Pos) {
       
        // Find if set is already in modified. If so update it with this new value

        match self.blocks.get(&loc) {
            Some(from) => self.modified.push((loc, *from)),
            None => self.modified.push((loc, Block::new(BlockType::Empty))),
        };

        self.blocks.insert(loc, block);
    }

    /// Step the board to the next state
    /// Blocks are calculated on original state
    /// Changes are applied on exit
    pub fn step(&mut self) {
        let mut modified_staged = vec![];
        let mut toggle_staged = vec![];

        for (modified_pos, prev_block) in &self.modified {
            let curr_active = match self.blocks.get(&modified_pos) {
                Some(block) => block.active,
                None => continue,
            };

            let next_active = self.calculate_block(*modified_pos);

            if curr_active != next_active {
                // Stage to be toggled
                toggle_staged.push(*modified_pos);

                // Add blocks that will need an update to next modified
                let modified_dirs = self.blocks[modified_pos].influences();
                let to_calc = modified_dirs
                    .into_iter()
                    .map(|dir| modified_pos.neighbour(dir));


                let blocks = to_calc.clone().into_iter().map(|pos| {
                    match self.blocks.get(&pos) {
                        Some(x) => *x,
                        None => Block::new(BlockType::Empty),
                    }
                });

                let to_calc = to_calc.into_iter().zip(blocks);

                modified_staged.extend(to_calc);
            }
        }

        Board::update_blocks(&mut self.blocks, toggle_staged);
        modified_staged.dedup();
        self.modified = modified_staged;
    }

    fn update_blocks(blocks: &mut HashMap<Pos, Block>, to_toggle: Vec<Pos>) {
        for loc in &to_toggle {
            if let Some(x) = blocks.get_mut(loc) {
                x.toggle();
            }
        }
    }

    fn calculate_block(&self, m: Pos) -> bool {
        let inputs = self.get_inputs(m);

        self.blocks.get(&m).unwrap().calc(inputs)
    }

    fn get_inputs(&self, pos: Pos) -> Vec<bool> {
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        directions
            .into_iter()
            .map(|dir| self.get_input(pos, dir))
            .collect()
    }

    fn get_input(&self, pos: Pos, dir: Direction) -> bool {
        let input_loc = pos.neighbour(dir);

        let input_block = match self.blocks.get(&input_loc) {
            Some(blk) => blk,
            None => return false,
        };

        let opposite = Direction::opposite(dir);

        input_block.output(opposite)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbour_pos() {
        let pos = Pos { x: 1, y: 1 };
        let expected_pos = Pos { x: 2, y: 1 };

        assert_eq!(pos.neighbour(Direction::Right), expected_pos);
    }

    #[test]
    fn new_board() {
        Board::new();
    }

    #[test]
    fn set_board() {
        let mut board = Board::new();
        board.set(
            Block::new(BlockType::NotArrow(Direction::Right)),
            Pos { x: 0, y: 2 },
        );

        let expected_block = Block {
            block_type: BlockType::NotArrow(Direction::Right),
            active: false,
        };

        assert_eq!(board.blocks[&Pos { x: 0, y: 2 }], expected_block);

        assert_eq!(board.modified, vec![(Pos { x: 0, y: 2 }, Block { block_type: BlockType::Empty, active: false })]);
    }

    #[test]
    fn set_twice_same_location() {
        let mut board = Board::new();
        board.set(
            Block::new(BlockType::NotArrow(Direction::Right)),
            Pos { x: 0, y: 2 },
        );
        board.set(
            Block::new(BlockType::Arrow(Direction::Left)),
            Pos { x: 0, y: 2 },
        );


        assert_eq!(board.blocks.get(&Pos { x: 0, y: 2 }), Some(Block::new(BlockType::Arrow(Direction::Left))));
        assert_eq!(board.modified, vec![(Pos { x: 0, y: 2 }, Block { block_type: BlockType::Empty, active: false })]);
    }

    #[test]
    fn add_then_remove() {
        let mut board = Board::new();
        board.set(
            Block::new(BlockType::NotArrow(Direction::Right)),
            Pos { x: 0, y: 2 },
        );
        board.set(
            Block::new(BlockType::Empty),
            Pos { x: 0, y: 2 },
        );


        assert_eq!(board.blocks.get(&Pos { x: 0, y: 2 }), None);
        assert_eq!(board.modified, vec![]);
    }

    #[test]
    fn add_empty_over_empty_board() {
        let mut board = Board::new();
        board.set(
            Block::new(BlockType::Empty),
            Pos { x: 0, y: 2 },
        );

        assert_eq!(board.blocks.get(&Pos { x: 0, y: 2 }), None);
        assert_eq!(board.modified, vec![]);
    }

    #[test]
    fn add_empty_over_arrow_board() {
        let mut board = Board::new();
        board.set(
            Block::new(BlockType::NotArrow(Direction::Right)),
            Pos { x: 0, y: 2 },
        );
        board.step();
        board.set(
            Block::new(BlockType::Empty),
            Pos { x: 0, y: 2 },
        );

        assert_eq!(board.blocks.get(&Pos { x: 0, y: 2 }), None);
        assert_eq!(board.modified, vec![(Pos { x: 0, y: 2 }, Block { block_type: BlockType::NotArrow(Direction::Right), active: false })]);
    }

    #[test]
    fn step_board() {
        let mut board = Board::new();
        board.set(
            Block::new(BlockType::NotArrow(Direction::Right)),
            Pos { x: 0, y: 1 },
        );
        board.set(
            Block::new(BlockType::Arrow(Direction::Right)),
            Pos { x: 1, y: 1 },
        );

        assert_eq!(board.blocks[&Pos { x: 0, y: 1 }].active, false);
        assert_eq!(board.blocks[&Pos { x: 1, y: 1 }].active, false);

        board.step();

        assert_eq!(board.blocks[&Pos { x: 0, y: 1 }].active, true);
        assert_eq!(board.blocks[&Pos { x: 1, y: 1 }].active, false);

        board.step();

        assert_eq!(board.blocks[&Pos { x: 0, y: 1 }].active, true);
        assert_eq!(board.blocks[&Pos { x: 1, y: 1 }].active, true);

        board.step();

        assert_eq!(board.blocks[&Pos { x: 0, y: 1 }].active, true);
        assert_eq!(board.blocks[&Pos { x: 1, y: 1 }].active, true);
    }
}
