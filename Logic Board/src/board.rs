use crate::block::{Block, BlockType, Direction};

#[derive(Debug)]
pub struct Board {
    pub board: Vec<Block>,
    pub size: (u32, u32),
    pub modified: Vec<(u32, u32)>,
}

impl Board {
    /// Create a new board full of Empty and empty modified
    pub fn new(size: (u32, u32)) -> Result<Board, &'static str> {
        let length = size.0 as usize * size.1 as usize;

        let board: Vec<Block> = vec![
            Block {
                block_type: BlockType::Empty,
                active: false,
                next_active: false,
            };
            length
        ];
        let modified: Vec<(u32, u32)> = Vec::new();

        Ok(Board {
            board,
            size,
            modified,
        })
    }
    /// Set the block on the board
    pub fn set(&mut self, block: Block, loc: (u32, u32)) {
        // Check if loc on board
        if loc.0 >= self.size.0 || loc.1 >= self.size.1 {
            println!("{:?} not on board", loc);
            return;
        }

        let pos = loc.0 + loc.1 * self.size.0;

        self.board[pos as usize] = block;
        self.modified.push(loc);
    }

    /// Step the board to the next state
    pub fn step(&mut self) {
        println!("{:?}", self.modified);

        let mut to_calculate: Vec<(u32, u32)> = Vec::new();
        let mut to_toggle_state: Vec<(u32, u32)> = Vec::new();

        for m in &self.modified {          
            let is_active_before = self.board[(m.0 + m.1 * self.size.0) as usize].active;
      
            let is_active_after = self.calculate_block(*m);

            if is_active_before == is_active_after {
                continue; // No changes to block
            }

            let modified_list = self.board[(m.0 + m.1 * self.size.0) as usize].influences();

            let mut to_calc: Vec<(u32, u32)> = vec![];
            for dir in modified_list {
                let elem = self.get_surrounding(*m, dir);
                if let Some(x) = elem {
                    to_calc.push(x);
                }
            }
            
            to_calculate.append(&mut to_calc);
            to_toggle_state.push(*m);
        }

        // Update for next Loop
        for loc in to_toggle_state {
            self.board[(loc.0 + loc.1 * self.size.0) as usize].toggle();
        }

        self.modified = to_calculate;
        println!("{:?}", self.modified);
    }


    fn calculate_block(&self, m: (u32, u32)) -> bool {
        let inputs = self.get_inputs(m);


        self.board[(m.0 + m.1 * self.size.0) as usize].calc(inputs)
    }

    fn get_inputs(&self, m: (u32, u32)) -> Vec<bool> {
        let directions = vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ];

        let mut inputs: Vec<bool> = Vec::new();
        for dir in &directions {
            inputs.push(
                if let Some((x, y)) = self.get_surrounding(m, dir.clone()) {
                    let opposite = Direction::opposite(dir.clone());
                    self.board[(x + y * self.size.0) as usize].output(opposite)
                } else {
                    false
                },
            );
        }
        inputs
    }

    /// Gets the 4 directly surrounding from a block. Returns None if past the boundaries of the board
    pub fn get_surrounding(&self, x: (u32, u32), dir: Direction) -> Option<(u32, u32)> {
        let step = match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        let result = (x.0 as i32 + step.0 as i32, x.1 as i32 + step.1 as i32);

        if result.0 == -1 || result.1 == -1 {
            return None;
        }

        if result.0 >= self.size.0 as i32 || result.1 >= self.size.1 as i32 {
            return None;
        }

        Some((result.0 as u32, result.1 as u32))
    }
}
