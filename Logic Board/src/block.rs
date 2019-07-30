#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(dir: Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum BlockType {
    Empty,
    Arrow(Direction),
    NotArrow(Direction),
    Split(Direction),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct Block {
    pub block_type: BlockType,
    pub active: bool,
}

impl Block {
    pub fn new(block_type: BlockType) -> Block {
        Block {
            block_type,
            active: false,
        }
    }

    /// Using internal state determine output in direction given
    pub fn output(self, dir: Direction) -> bool {
        match &self.block_type {
            BlockType::Arrow(n) if *n == dir => self.active,
            BlockType::NotArrow(n) if *n == dir => self.active,
            BlockType::Split(n) if *n == dir => self.active,
            BlockType::Split(n) if *n == Direction::opposite(dir) => self.active,
            _ => false,
        }
    }

    /// Using external outputs, calculate the block and return solution
    pub fn calc(self, inputs: Vec<bool>) -> bool {
        let is_any_surrounding = inputs.iter().any(|&x| x);

        match self.block_type {
            BlockType::NotArrow(_) => !is_any_surrounding,
            _ => is_any_surrounding,
        }
    }

    /// When value toggles what other blocks could be changed (influenced)
    pub fn influences(self) -> Vec<Direction> {
        match &self.block_type {
            BlockType::Arrow(d) => vec![*d],
            BlockType::NotArrow(d) => vec![*d],
            BlockType::Split(d) => vec![*d, Direction::opposite(*d)],
            BlockType::Empty => vec![],
        }
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
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
        };

        let inputs: Vec<bool> = vec![false, false, false, true];

        if arrow.active != arrow.calc(inputs) {
            arrow.toggle();
        }

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
        };

        let inputs: Vec<bool> = vec![false, false, true, false];

        if not.active != not.calc(inputs) {
            not.toggle();
        }

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
        };

        let inputs: Vec<bool> = vec![false, false, false, false];

        if split.active != split.calc(inputs) {
            split.toggle();
        }

        assert_eq!(false, split.output(Direction::Up));
        assert_eq!(false, split.output(Direction::Right));
        assert_eq!(false, split.output(Direction::Down));
        assert_eq!(false, split.output(Direction::Left));
        assert_eq!(split.active, false);
    }
}
