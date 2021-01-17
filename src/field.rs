use crate::Position;

pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 20;

pub struct Field {
    pub blocked_tiles: Vec<bool>,
    pub ghost_tiles: Vec<Position>,
    pub ghost_dirty: bool,
}

impl Field {
    pub fn new() -> Field {
        Field {
            blocked_tiles: vec![false; FIELD_WIDTH * FIELD_HEIGHT],
            ghost_tiles: Vec::new(),
            ghost_dirty: true,
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        y as usize * FIELD_WIDTH + x as usize
    }

    pub fn is_tile_blocked(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= FIELD_WIDTH as i32 || y >= FIELD_HEIGHT as i32 {
            return true;
        }
        if y < 0 {
            return false;
        }
        self.blocked_tiles[self.xy_idx(x, y)]
    }
}
