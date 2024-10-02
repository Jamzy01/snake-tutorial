use max7219_canvas::layer::CanvasLayer;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }

    pub fn add_movement(
        &self,
        position: (usize, usize),
        max_x: usize,
        max_y: usize
    ) -> Option<(usize, usize)> {
        match self {
            Self::Up =>
                match position.1 < 1 {
                    true => None,
                    false => Some((position.0, position.1 - 1)),
                }
            Self::Down =>
                match position.1 >= max_y {
                    true => None,
                    false => Some((position.0, position.1 + 1)),
                }
            Self::Left =>
                match position.0 < 1 {
                    true => None,
                    false => Some((position.0 - 1, position.1)),
                }
            Self::Right =>
                match position.0 >= max_x {
                    true => None,
                    false => Some((position.0 + 1, position.1)),
                }
        }
    }

    pub fn from_joystick(x: f32, y: f32) -> Option<Self> {
        if x > 0.5 {
            Some(Self::Right)
        } else if x < -0.5 {
            Some(Self::Left)
        } else if y > 0.5 {
            Some(Self::Up)
        } else if y < -0.5 {
            Some(Self::Down)
        } else {
            None
        }
    }
}

pub struct Snake<const L: usize> {
    position: (usize, usize),
    tail: [Direction; L],
    snake_length: usize,
}

impl<const L: usize> Snake<L> {
    pub fn new(position_x: usize, position_y: usize, snake_length: usize) -> Self {
        Self {
            position: (position_x, position_y),
            tail: [Direction::default(); L],
            snake_length,
        }
    }

    pub fn self_collides(&self) -> bool {
        let mut current_position = self.position;
        let mut active_positions: [(usize, usize); L] = [(usize::MAX, usize::MAX); L];

        for i in 0..self.snake_length {
            active_positions[i] = current_position;

            if let Some(next_position) = self.tail[i].add_movement(current_position, 7, 7) {
                if active_positions.contains(&next_position) {
                    return true;
                }

                current_position = next_position;
            }
        }

        false
    }

    pub fn next_position(&self, direction: &Direction) -> Option<(usize, usize)> {
        if self.self_collides() {
            return None;
        }

        direction.add_movement(self.position, 7, 7)
    }

    pub fn move_snake(&mut self, direction: &Direction, extend: bool) -> bool {
        if let Some(new_position) = self.next_position(direction) {
            self.position = new_position;

            if extend {
                self.snake_length += 1;
            }

            for i in (0..self.snake_length).rev() {
                self.tail[i] = match i {
                    0 => direction.opposite(),
                    _ => self.tail[i - 1],
                };
            }

            false
        } else {
            true
        }
    }

    pub fn as_layer(&self) -> CanvasLayer<1> {
        let mut layer = CanvasLayer::new();

        let mut current_position = self.position;

        for i in 0..self.snake_length {
            layer.set_pixel(current_position.0, current_position.1, true);

            if let Some(next_position) = self.tail[i].add_movement(current_position, 7, 7) {
                current_position = next_position;
            } else {
                break;
            }
        }

        layer
    }
}
