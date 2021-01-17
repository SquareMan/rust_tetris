use crate::{gui, Field};
use crate::{Block, Position};
use rltk::{Rltk, RGB};
use specs::prelude::*;

pub fn render(world: &World, ctx: &mut Rltk) {
    let field = world.fetch::<Field>();
    let blocks = world.read_storage::<Block>();
    let positions = world.read_storage::<Position>();

    // Render ghosts
    for pos in field.ghost_tiles.iter() {
        ctx.set(
            pos.x + gui::FIELD_X as i32,
            pos.y + gui::FIELD_Y as i32,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::CYAN),
            rltk::to_cp437('.'),
        )
    }

    //Render blocks
    for (block, position) in (&blocks, &positions).join() {
        if position.y < 0 {
            continue;
        }
        ctx.set(
            position.x + gui::FIELD_X as i32,
            position.y + gui::FIELD_Y as i32,
            block.fg,
            block.bg,
            rltk::to_cp437('#'),
        );
    }
}
