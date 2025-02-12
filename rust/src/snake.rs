/// A simple position struct to track coordinates on the terminal.
#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

/// Possible movement directions.
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The Snake is represented by a vector of Positions (its body) and a current direction.
pub struct Snake {
    pub body: Vec<Position>,
    direction: Direction,
}

impl Snake {
    pub(crate) fn new(start: Position) -> Self {
        Self {
            body: vec![start],
            direction: Direction::Right,
        }
    }

    /// Returns the current head (first element) of the snake.
    pub(crate) fn head(&self) -> Position {
        self.body[0]
    }

    /// Moves the snake one step forward in its current direction.
    /// If `grow` is true, the snake's tail is not removed (thus growing the snake).
    pub(crate) fn move_forward(&mut self, grow: bool) {
        let head = self.head();
        let new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Position {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Position {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Position {
                x: head.x + 1,
                y: head.y,
            },
        };

        self.body.insert(0, new_head);
        if !grow {
            self.body.pop();
        }
    }

    /// Change the snake's direction. Prevents 180° reversal.
    pub(crate) fn change_direction(&mut self, new_direction: Direction) {
        // Prevent reverse direction (e.g., cannot go from Up to Down directly).
        match (&self.direction, &new_direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {}
            _ => self.direction = new_direction,
        }
    }

    /// Checks if a given position is occupied by the snake.
    pub(crate) fn is_colliding(&self, pos: &Position) -> bool {
        self.body.iter().any(|&p| p == *pos)
    }

    /// Returns `true` if the snake's head collides with any other part of its body.
    pub(crate) fn check_self_collision(&self) -> bool {
        let head = self.head();
        self.body.iter().skip(1).any(|&p| p == head)
    }
}
