#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Empty,
    Arrow(Direction),
    NotArrow(Direction),
    Split(Direction),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub active: bool,
    pub next_active: bool,
}

impl Block {
    pub fn new(block_type: BlockType) -> Block {
        Block {
            block_type,
            active: false,
            next_active: false,
        }
    }
    /// Given a direction tells what the output will be, None if not attached
    /// NotArrow will output true if active and false is not active (Acts like arrow on output)
    pub fn output(&self, direction: Direction) -> bool {
        if let BlockType::Arrow(d) = &self.block_type {
            if d == &direction {
                return self.active;
            }
        }

        if let BlockType::NotArrow(d) = &self.block_type {
            if d == &direction {
                return self.active;
            }
        }

        let opposite = Direction::opposite(direction.clone());
        if let BlockType::Split(d) = &self.block_type {
            if d == &direction || d == &opposite {
                return self.active;
            }
        }

        false
    }

    pub fn next_output(&self, direction: Direction) -> bool {
        if let BlockType::Arrow(d) = &self.block_type {
            if d == &direction {
                return self.next_active;
            }
        }

        if let BlockType::NotArrow(d) = &self.block_type {
            if d == &direction {
                return self.next_active;
            }
        }

        let opposite = Direction::opposite(direction.clone());
        if let BlockType::Split(d) = &self.block_type {
            if d == &direction || d == &opposite {
                return self.next_active;
            }
        }

        false
    }

    /// Calculates the block, updating internal active property
    pub fn calc(&mut self, inputs: Vec<bool>) {
        let is_any_surrounding = inputs.iter().any(|&x| x);

        self.next_active = match self.block_type {
            BlockType::NotArrow(_) => !is_any_surrounding,
            _ => is_any_surrounding,
        };
    }

    pub fn apply(&mut self) {
        self.active = self.next_active;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_outputs() {
        let arrow = Block {
            block_type: BlockType::Arrow(Direction::Right),
            active: true,
            next_active: false,
        };

        assert_eq!(arrow.output(Direction::Up), false);
        assert_eq!(arrow.output(Direction::Right), true);
        assert_eq!(arrow.output(Direction::Down), false);
        assert_eq!(arrow.output(Direction::Left), false);
    }

    #[test]
    fn not_arrow_outputs() {
        let arrow = Block {
            block_type: BlockType::NotArrow(Direction::Up),
            active: true,
            next_active: false,
        };

        assert_eq!(arrow.output(Direction::Up), true);
        assert_eq!(arrow.output(Direction::Right), false);
        assert_eq!(arrow.output(Direction::Down), false);
        assert_eq!(arrow.output(Direction::Left), false);
    }

    #[test]
    fn split_outputs() {
        let arrow = Block {
            block_type: BlockType::Split(Direction::Up),
            active: true,
            next_active: false,
        };

        assert_eq!(arrow.output(Direction::Up), true);
        assert_eq!(arrow.output(Direction::Right), false);
        assert_eq!(arrow.output(Direction::Down), true);
        assert_eq!(arrow.output(Direction::Left), false);
    }

    #[test]
    fn arrow_calc() {
        let mut arrow = Block {
            block_type: BlockType::Arrow(Direction::Right),
            active: false,
            next_active: false,
        };

        let inputs: Vec<bool> = vec![false, false, false, true];

        arrow.calc(inputs);
        arrow.apply();

        assert_eq!(false, arrow.output(Direction::Up));
        assert_eq!(true, arrow.output(Direction::Right));
        assert_eq!(false, arrow.output(Direction::Down));
        assert_eq!(false, arrow.output(Direction::Left));
        assert_eq!(arrow.active, true);
    }

    #[test]
    fn not_calc() {
        let mut not = Block {
            block_type: BlockType::NotArrow(Direction::Right),
            active: true,
            next_active: false,
        };

        let inputs: Vec<bool> = vec![false, false, true, false];

        not.calc(inputs);
        not.apply();

        assert_eq!(false, not.output(Direction::Up));
        assert_eq!(false, not.output(Direction::Right));
        assert_eq!(false, not.output(Direction::Down));
        assert_eq!(false, not.output(Direction::Left));
        assert_eq!(not.active, false);
    }

    #[test]
    fn split_calc() {
        let mut split = Block {
            block_type: BlockType::Split(Direction::Right),
            active: false,
            next_active: false,
        };

        let inputs: Vec<bool> = vec![false, false, false, false];

        split.calc(inputs);

        assert_eq!(false, split.output(Direction::Up));
        assert_eq!(false, split.output(Direction::Right));
        assert_eq!(false, split.output(Direction::Down));
        assert_eq!(false, split.output(Direction::Left));
        assert_eq!(split.active, false);
    }
}
