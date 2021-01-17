use crate::{Block, Falling, Field, Position, RunState, RunStateHolder, FIELD_HEIGHT, FIELD_WIDTH};
use specs::prelude::*;

pub struct FieldClearingSystem {}

impl<'a> System<'a> for FieldClearingSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, RunStateHolder>,
        ReadExpect<'a, Field>,
        ReadStorage<'a, Block>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Falling>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut state, field, blocks, mut positions, falling) = data;
        if state.0 != RunState::ReadyToClear {
            return;
        }

        let mut cleared_lines = Vec::new();
        let mut top_line = FIELD_HEIGHT as i32;
        for y in 0..FIELD_HEIGHT as i32 {
            let mut line_cleared = true;
            for x in 0..FIELD_WIDTH as i32 {
                if !field.is_tile_blocked(x, y) {
                    line_cleared = false;
                    break;
                }
            }

            if line_cleared {
                cleared_lines.push(y);
                top_line = std::cmp::min(y, top_line);
            }
        }

        state.0 = RunState::ReadyToSpawn;

        let num_lines = cleared_lines.len() as i32;
        if num_lines < 1 {
            return;
        }

        for (entity, _block, position) in (&entities, &blocks, &mut positions).join() {
            if cleared_lines.contains(&position.y) {
                entities
                    .delete(entity)
                    .expect("Unable to delete cleared blocks");
            } else if position.y < top_line {
                position.y += num_lines;
            }
        }
    }
}
