use crate::{gui, Field, Held};
use crate::{Block, Position};
use rltk::{Rltk, RGB};
use specs::prelude::*;

pub fn render(world: &World, ctx: &mut Rltk) {
    let field = world.fetch::<Field>();
    let blocks = world.read_storage::<Block>();
    let positions = world.read_storage::<Position>();
    let held = world.read_storage::<Held>();

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
    for (block, position, held) in (&blocks, &positions, (&held).maybe()).join() {
        if held.is_some() {
            ctx.set(
                position.x + gui::HELD_X as i32,
                position.y + gui::HELD_Y as i32,
                block.fg,
                block.bg,
                rltk::to_cp437('#'),
            );
        }
        else {
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
}
