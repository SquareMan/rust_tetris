use crate::gui;
use crate::{Block, Position};
use rltk::Rltk;
use specs::prelude::*;

pub fn render(world: &World, ctx: &mut Rltk) {
    let blocks = world.read_storage::<Block>();
    let positions = world.read_storage::<Position>();

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
