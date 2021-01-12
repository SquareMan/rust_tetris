use rltk::{GameState, BTerm, RGB};
use specs::prelude::*;

mod components;
mod field;
mod gui;
mod input;
mod rendering_system;
mod falling_block_system;
mod field_indexing_system;
mod field_clearing_system;
mod block_spawning_system;

pub use components::*;
pub use field::*;
use crate::rendering_system::render;
use crate::falling_block_system::FallingBlockSystem;
use crate::gui::draw_gui;
use crate::field_indexing_system::FieldIndexingSystem;
use crate::field_clearing_system::FieldClearingSystem;
use crate::block_spawning_system::BlockSpawningSystem;
use crate::input::read_input;

#[derive(Default)]
pub struct DeltaTime(f32);

#[derive(Default)]
pub struct RunStateHolder(RunState);

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    Falling,
    ReadyToClear,
    ReadyToSpawn
}
impl Default for RunState {
    fn default() -> Self {
        RunState::Falling
    }
}

pub struct State {
    pub world: World,
    field_indexing_system: FieldIndexingSystem,
    field_clearing_system: FieldClearingSystem,
    falling_block_system: FallingBlockSystem,
    block_spawning_system: BlockSpawningSystem,
}
impl State {
    fn new() -> State {
        State {
            world: World::new(),
            field_indexing_system: FieldIndexingSystem {},
            field_clearing_system: FieldClearingSystem {},
            falling_block_system: FallingBlockSystem::new(),
            block_spawning_system: BlockSpawningSystem {},
        }
    }

    fn run_systems(&mut self) {
        self.field_indexing_system.run_now(&self.world);
        self.block_spawning_system.run_now(&self.world);
        self.field_clearing_system.run_now(&self.world);
        self.falling_block_system.run_now(&self.world);

        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        //Update delta-time
        *self.world.write_resource::<DeltaTime>() = DeltaTime(ctx.frame_time_ms);

        let state = self.world.fetch::<RunStateHolder>().0;
        match state {
            RunState::Falling => {
                read_input(&mut self.world, ctx);
                self.run_systems();
            }
            RunState::ReadyToClear => {
                self.run_systems();
            }
            RunState::ReadyToSpawn => {
                self.run_systems();
            }
        }

        draw_gui(ctx);
        render(&self.world, ctx);
    }
}

fn main() -> rltk::BError {
    let ctx = rltk::RltkBuilder::simple80x50()
        .with_title("Tetris")
        .build()?;
    let mut gs = State::new();

    gs.world.register::<Position>();
    gs.world.register::<Block>();
    gs.world.register::<Falling>();

    gs.world.insert(Field::new());
    gs.world.insert(DeltaTime(ctx.frame_time_ms));
    gs.world.insert(rltk::RandomNumberGenerator::new());
    gs.world.insert(RunStateHolder(RunState::ReadyToSpawn));

    rltk::main_loop(ctx, gs)
}
