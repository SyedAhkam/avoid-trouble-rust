use crate::game::Position;

pub struct Player {
    position: Position,
    is_moving: bool
}

impl Default for Player {
    fn default() -> Player {
        Self {
            position: Position::default(), 
            is_moving: false 
        }
    }
}
