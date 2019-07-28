use std::collections::HashMap;

use crate::block::{Block, Direction};

#[derive(Debug)]
pub struct Board {
    pub blocks: HashMap<(i32, i32), Block>,
    pub modified: Vec<(i32, i32)>,
}

impl Board {
    /// Create a new board full of Empty and empty modified
    pub fn new() -> Self {
        let blocks = HashMap::new();
        let modified = vec![];

        Board { blocks, modified }
    }
    /// Set the block on the board
    pub fn set(&mut self, block: Block, loc: (i32, i32)) {
        self.blocks.insert(loc, block);
        self.modified.push(loc);
    }
    /// Step the board to the next state
    pub fn step(&mut self) {
        // This is the meat of the logic. Goes through a modified list from the previous step
        // 1. Checks to see if we need to update, if nothing is there or it doesn't change
        // 2. Stages it to updated if updated
        // 3. Pushs the arrows that will change around it to the next_modified list for the next 'step'
        // When all modified list are done, update the change blocks and make the modfied for the next step

        let mut next_modified = vec![];
        let mut next_toggled = vec![];

        for m in &self.modified {
            // if block doesn't exist,  continue
            if self.blocks.get(m).is_none() {
                continue;
            }

            // Check to see if anything will change will update
            let is_active_before = self.blocks[m].active;
            let is_active_after = self.calculate_block(*m);
            if is_active_before == is_active_after {
                continue; // No changes to block
            }

            // Stage to be toggled
            next_toggled.push(*m);

            // Add blocks that will need an updat to next modified
            let modified_dirs = self.blocks[m].influences();
            let mut to_calc = vec![];
            for dir in modified_dirs {
                let elem = self.get_surrounding(*m, dir);
                if let Some(x) = elem {
                    to_calc.push(x);
                }
            }
            next_modified.extend(to_calc);
        }

        // Update for next Loop
        Board::update_blocks(&mut self.blocks, next_toggled);
        next_modified.dedup(); // Removes duplicates
        self.modified = next_modified;
    }

    fn update_blocks(blocks: &mut HashMap<(i32, i32), Block>, to_toggle: Vec<(i32, i32)>) {
        for loc in &to_toggle {
            if let Some(x) = blocks.get_mut(loc) {
                x.toggle();
            }
        }
    }

    fn calculate_block(&self, m: (i32, i32)) -> bool {
        let inputs = self.get_inputs(m);

        self.blocks.get(&m).unwrap().calc(inputs)
    }

    fn get_inputs(&self, m: (i32, i32)) -> Vec<bool> {
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        let mut inputs: Vec<bool> = Vec::new();
        for dir in &directions {
            inputs.push(if let Some(loc) = self.get_surrounding(m, dir.clone()) {
                if let Some(x) = self.blocks.get(&loc) {
                    let opposite = Direction::opposite(dir.clone());
                    x.output(opposite)
                } else {
                    false
                }
            } else {
                false
            });
        }
        inputs
    }

    /// Gets the 4 directly surrounding from a block. Returns None if past the boundaries of the board
    fn get_surrounding(&self, x: (i32, i32), dir: Direction) -> Option<(i32, i32)> {
        let step = match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        let result = (x.0 as i32 + step.0 as i32, x.1 as i32 + step.1 as i32);

        Some((result.0 as i32, result.1 as i32))
    }
}
