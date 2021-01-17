use crate::{Falling, Field, Position};
use specs::prelude::*;

pub struct FieldIndexingSystem {}

impl<'a> System<'a> for FieldIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Field>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Falling>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut field, positions, falling) = data;

        for i in 0..field.blocked_tiles.len() {
            field.blocked_tiles[i] = false;
        }
        for (pos, ()) in (&positions, !&falling).join() {
            if pos.y < 0 {
                continue;
            }
            let idx = field.xy_idx(pos.x, pos.y);
            field.blocked_tiles[idx] = true;
        }
    }
}
