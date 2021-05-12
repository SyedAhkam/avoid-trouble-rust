use crate::game::Position;

pub struct Obstacle {
   position: Position
}

impl Default for Obstacle {
    fn default() -> Obstacle {
        Self {
            position: Position::default()
        }
    }
}
