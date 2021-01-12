use specs::prelude::*;
use crate::{DeltaTime, Falling, Position, Block, Field, RunState, RunStateHolder};

const FALL_TIME : f32 = 500.0;

pub struct FallingBlockSystem {
    timer: f32
}
impl FallingBlockSystem {
    pub fn new() -> FallingBlockSystem {
        FallingBlockSystem{
            timer: 0.0
        }
    }
}

impl<'a> System<'a> for FallingBlockSystem {
    type SystemData = ( ReadExpect<'a, DeltaTime>,
                        ReadExpect<'a, Field>,
                        WriteExpect<'a, RunStateHolder>,
                        WriteStorage<'a, Falling>,
                        ReadStorage<'a, Block>,
                        WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (dt, field,mut state, mut falling, blocks, mut positions) = data;

        if falling.is_empty() { return; }

        // Advance falling timer
        self.timer += dt.0;
        if self.timer < FALL_TIME {return;}
        self.timer = 0.0;

        let mut stop_falling = false;
        for (_falling, _block, position) in (&falling, &blocks, &mut positions).join() {
            if field.is_tile_blocked(position.x, position.y+1) {
                stop_falling = true;
                break;
            }
        }

        if stop_falling {
            falling.clear();
            state.0 = RunState::ReadyToClear;
        }
        else {
            for (_falling, _block, position) in (&falling, &blocks, &mut positions).join() {
                position.y += 1;
            }
        }
    }
}