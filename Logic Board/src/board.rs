use std::collections::HashMap;

use crate::block::{Block, Direction};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Board {
    pub blocks: HashMap<Pos, Block>,
    pub modified: Vec<Pos>,
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
        self.blocks.insert(loc, block);
        self.modified.push(loc);
    }
    /// Step the board to the next state
    pub fn step(&mut self) {
        // Changes are staged in 
        // toggle_staged - When completed all updates the toggle is applied      
        // next_modified - When completed all updates the modified list is replace with next_modified

        let mut next_modified = vec![];
        let mut toggle_staged = vec![];

        for modified_pos in &self.modified {
            if self.blocks.get(modified_pos).is_none() {
                continue;
            }

            // Check to see if anything will change will update
            let is_active_before = self.blocks[modified_pos].active;
            let is_active_after = self.calculate_block(*modified_pos);
            if is_active_before == is_active_after {
                continue; // No changes to block
            }

            // Stage to be toggled
            toggle_staged.push(*modified_pos);

            // Add blocks that will need an update to next modified
            let modified_dirs = self.blocks[modified_pos].influences();
            let to_calc = modified_dirs
                .into_iter()
                .map(|dir| self.get_surrounding(*modified_pos, dir))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap());

            next_modified.extend(to_calc);
        }

        // Update for next Loop
        Board::update_blocks(&mut self.blocks, toggle_staged);
        next_modified.dedup(); // Removes duplicates
        self.modified = next_modified;
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

        directions.into_iter().map(|dir| self.get_input(pos, dir)).collect()
    }

    fn get_input(&self, pos: Pos, dir: Direction) -> bool {
        let input_loc = match self.get_surrounding(pos, dir) {
            Some(pos) => pos,
            None => return false,
        };

        let input_block = match self.blocks.get(&input_loc) {
            Some(blk) => blk,
            None => return false,
        };

        let opposite = Direction::opposite(dir);

        input_block.output(opposite)
    }

    /// Gets the 4 directly surrounding from a block. Returns None if past the boundaries of the board
    fn get_surrounding(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        let step = match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        let (x, y) = (pos.x as i32 + step.0 as i32, pos.y as i32 + step.1 as i32);

        Some(Pos { x, y })
    }
}
