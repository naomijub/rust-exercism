// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{position: (x, y), direction: d}
    }

    pub fn turn_right(self) -> Self {
        let d = match self.direction {
            Direction::North =>  Direction::East,
            Direction::East =>  Direction::South,
            Direction::South =>  Direction::West,
            Direction::West =>  Direction::North,
        };
        Robot{position: self.position, direction: d}
    }

    pub fn turn_left(self) -> Self {
        let d = match self.direction {
            Direction::North =>  Direction::West,
            Direction::East =>  Direction::North,
            Direction::South =>  Direction::East,
            Direction::West =>  Direction::South,
        };
        Robot{position: self.position, direction: d}    
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North => 
                Robot {position: (self.position.0, self.position.1 + 1), direction: self.direction},
            Direction::East => 
                Robot {position: (self.position.0 + 1, self.position.1), direction: self.direction},
            Direction::South => 
                Robot {position: (self.position.0, self.position.1 - 1), direction: self.direction},
            Direction::West => 
                Robot {position: (self.position.0 - 1, self.position.1), direction: self.direction}
        }  
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars()
            .fold(self, |robot, x| {
                match x {
                    'A' => robot.advance(),
                    'R' => robot.turn_right(),
                    'L' => robot.turn_left(),
                    _ => robot,
                }
            })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
