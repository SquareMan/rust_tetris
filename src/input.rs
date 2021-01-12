use specs::prelude::*;
use rltk::{Rltk,VirtualKeyCode};
use crate::{Block, Position, Field, Falling};

pub fn read_input(world: &mut World, ctx: &Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move(world, -1),
            VirtualKeyCode::Right => try_move(world, 1),
            _ => {}
        }
    }
}

fn try_move(world: &mut World, x: i32) {
    let field = world.fetch::<Field>();
    let blocks = world.read_storage::<Block>();
    let mut positions = world.write_storage::<Position>();
    let falling = world.read_storage::<Falling>();

    let mut legal_move = true;
    for (_blocks, _falling, pos) in (&blocks, &falling, &mut positions).join() {
        if field.is_tile_blocked(pos.x + x, pos.y) {
            legal_move = false;
            break;
        }
    }

    if legal_move {
        for(_blocks, _falling, pos) in (&blocks, &falling, &mut positions).join() {
            pos.x += x;
        }
    }
}