use crate::gui::FIELD_Y;
use crate::{Falling, Field, Position, RunState, RunStateHolder, FIELD_HEIGHT, Held};
use specs::prelude::*;

pub struct FieldIndexingSystem {}

impl<'a> System<'a> for FieldIndexingSystem {
    type SystemData = (
        ReadExpect<'a, RunStateHolder>,
        WriteExpect<'a, Field>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Falling>,
        ReadStorage<'a, Held>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (runstate, mut field, positions, falling, held) = data;

        for i in 0..field.blocked_tiles.len() {
            field.blocked_tiles[i] = false;
        }
        for (pos, (), _) in (&positions, !&falling, !&held).join() {
            if pos.y < 0 {
                continue;
            }
            let idx = field.xy_idx(pos.x, pos.y);
            field.blocked_tiles[idx] = true;
        }

        if field.ghost_dirty && runstate.0 == RunState::Falling {
            field.ghost_tiles.clear();
            let mut collision_distance = FIELD_HEIGHT as i32;
            for (pos, _) in (&positions, &falling).join() {
                for y in pos.y..FIELD_HEIGHT as i32 + 1 {
                    if field.is_tile_blocked(pos.x, y) {
                        collision_distance = std::cmp::min(collision_distance, y - pos.y - 1);
                        break;
                    }
                }
            }

            for (pos, _) in (&positions, &falling).join() {
                field.ghost_tiles.push(Position {
                    x: pos.x,
                    y: pos.y + collision_distance,
                });
            }
        }
    }
}
