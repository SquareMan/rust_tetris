use crate::{Block, Center, Falling, Field, Position, RunState, RunStateHolder};
use rltk::RGB;
use specs::prelude::*;

pub struct BlockSpawningSystem {}
impl<'a> System<'a> for BlockSpawningSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, RunStateHolder>,
        WriteExpect<'a, rltk::RandomNumberGenerator>,
        ReadExpect<'a, Field>,
        WriteStorage<'a, Falling>,
        WriteStorage<'a, Block>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Center>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut state,
            mut rng,
            field,
            mut falling,
            mut blocks,
            mut positions,
            mut center,
        ) = data;

        if state.0 != RunState::ReadyToSpawn {
            return;
        }

        match rng.roll_dice(1, 7) {
            1 => {
                BlockSpawningSystem::spawn_block_i(
                    &entities,
                    &mut blocks,
                    &mut positions,
                    &mut falling,
                    &mut center,
                )
                .expect("Couldn't spawn new tetromino");
            }
            2 => {
                BlockSpawningSystem::spawn_block_j(
                    &entities,
                    &mut blocks,
                    &mut positions,
                    &mut falling,
                    &mut center,
                )
                .expect("Couldn't spawn new tetromino");
            }
            3 => {
                BlockSpawningSystem::spawn_block_l(
                    &entities,
                    &mut blocks,
                    &mut positions,
                    &mut falling,
                    &mut center,
                )
                .expect("Couldn't spawn new tetromino");
            }
            4 => {
                BlockSpawningSystem::spawn_block_o(
                    &entities,
                    &mut blocks,
                    &mut positions,
                    &mut falling,
                    &mut center,
                )
                .expect("Couldn't spawn new tetromino");
            }
            5 => {
                BlockSpawningSystem::spawn_block_s(
                    &entities,
                    &mut blocks,
                    &mut positions,
                    &mut falling,
                    &mut center,
                )
                .expect("Couldn't spawn new tetromino");
            }
            6 => {
                BlockSpawningSystem::spawn_block_t(
                    &entities,
                    &mut blocks,
                    &mut positions,
                    &mut falling,
                    &mut center,
                )
                .expect("Couldn't spawn new tetromino");
            }
            _ => {
                BlockSpawningSystem::spawn_block_z(
                    &entities,
                    &mut blocks,
                    &mut positions,
                    &mut falling,
                    &mut center,
                )
                .expect("Couldn't spawn new tetromino");
            }
        }

        state.0 = RunState::Falling;
    }
}

impl BlockSpawningSystem {
    fn spawn_block_i(
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        BlockSpawningSystem::create_block(5, -1, rltk::CYAN, entities, blocks, positions, falling)?;
        BlockSpawningSystem::create_block_center(
            5,
            -2,
            rltk::CYAN,
            entities,
            blocks,
            positions,
            falling,
            center,
        )?;
        BlockSpawningSystem::create_block(5, -3, rltk::CYAN, entities, blocks, positions, falling)?;
        BlockSpawningSystem::create_block(5, -4, rltk::CYAN, entities, blocks, positions, falling)
    }

    fn spawn_block_j(
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        BlockSpawningSystem::create_block(
            4,
            -2,
            rltk::DARK_BLUE,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block_center(
            5,
            -2,
            rltk::DARK_BLUE,
            entities,
            blocks,
            positions,
            falling,
            center,
        )?;
        BlockSpawningSystem::create_block(
            6,
            -2,
            rltk::DARK_BLUE,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block(
            6,
            -1,
            rltk::DARK_BLUE,
            entities,
            blocks,
            positions,
            falling,
        )
    }

    fn spawn_block_l(
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        BlockSpawningSystem::create_block(
            4,
            -2,
            rltk::ORANGE,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block_center(
            5,
            -2,
            rltk::ORANGE,
            entities,
            blocks,
            positions,
            falling,
            center,
        )?;
        BlockSpawningSystem::create_block(
            6,
            -2,
            rltk::ORANGE,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block(4, -1, rltk::ORANGE, entities, blocks, positions, falling)
    }

    fn spawn_block_o(
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        BlockSpawningSystem::create_block(
            4,
            -1,
            rltk::YELLOW,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block(
            5,
            -1,
            rltk::YELLOW,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block(
            4,
            -2,
            rltk::YELLOW,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block(5, -2, rltk::YELLOW, entities, blocks, positions, falling)
    }

    fn spawn_block_s(
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        BlockSpawningSystem::create_block(
            4,
            -3,
            rltk::GREEN,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block_center(
            4,
            -2,
            rltk::GREEN,
            entities,
            blocks,
            positions,
            falling,
            center,
        )?;
        BlockSpawningSystem::create_block(
            5,
            -2,
            rltk::GREEN,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block(5, -1, rltk::GREEN, entities, blocks, positions, falling)
    }

    fn spawn_block_t(
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        BlockSpawningSystem::create_block(
            5,
            -1,
            rltk::MAGENTA,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block(
            6,
            -2,
            rltk::MAGENTA,
            entities,
            blocks,
            positions,
            falling,
        )?;
        BlockSpawningSystem::create_block_center(
            5,
            -2,
            rltk::MAGENTA,
            entities,
            blocks,
            positions,
            falling,
            center,
        )?;
        BlockSpawningSystem::create_block(
            4,
            -2,
            rltk::MAGENTA,
            entities,
            blocks,
            positions,
            falling,
        )
    }

    fn spawn_block_z(
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        BlockSpawningSystem::create_block(5, -3, rltk::RED, entities, blocks, positions, falling)?;
        BlockSpawningSystem::create_block_center(
            5,
            -2,
            rltk::RED,
            entities,
            blocks,
            positions,
            falling,
            center,
        )?;
        BlockSpawningSystem::create_block(4, -2, rltk::RED, entities, blocks, positions, falling)?;
        BlockSpawningSystem::create_block(4, -1, rltk::RED, entities, blocks, positions, falling)
    }

    fn create_block(
        x: i32,
        y: i32,
        fg: (u8, u8, u8),
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
    ) -> Result<(), specs::error::Error> {
        let e = entities.create();
        blocks.insert(
            e,
            Block {
                fg: RGB::named(fg),
                bg: RGB::named(rltk::BLACK),
            },
        )?;
        positions.insert(e, Position { x, y })?;
        falling.insert(e, Falling {})?;
        Result::Ok(())
    }

    fn create_block_center(
        x: i32,
        y: i32,
        fg: (u8, u8, u8),
        entities: &Entities,
        blocks: &mut WriteStorage<Block>,
        positions: &mut WriteStorage<Position>,
        falling: &mut WriteStorage<Falling>,
        center: &mut WriteStorage<Center>,
    ) -> Result<(), specs::error::Error> {
        let e = entities.create();
        blocks.insert(
            e,
            Block {
                fg: RGB::named(fg),
                bg: RGB::named(rltk::BLACK),
            },
        )?;
        positions.insert(e, Position { x, y })?;
        falling.insert(e, Falling {})?;
        center.insert(e, Center {})?;
        Result::Ok(())
    }
}
