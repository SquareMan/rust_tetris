use specs::prelude::*;
use rltk::{Rltk,VirtualKeyCode};
use crate::{Block, Position, Field, Falling, FIELD_WIDTH, FIELD_HEIGHT, Center};

pub fn read_input(world: &mut World, ctx: &Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move(world, -1),
            VirtualKeyCode::Right => try_move(world, 1),
            VirtualKeyCode::RShift => try_rotate(world),
            _ => {}
        }
    }
}

fn try_rotate(world: &mut World) {
    let field = world.fetch::<Field>();
    let blocks = world.read_storage::<Block>();
    let mut positions = world.write_storage::<Position>();
    let falling = world.read_storage::<Falling>();
    let center = world.read_storage::<Center>();

    let mut falling_positions = Vec::new();
    let mut center_pos = None;
    for (_blocks, _falling, pos, center) in (&blocks, &falling, &mut positions, (&center).maybe()).join() {
        falling_positions.push(*pos);

        if let Some(_) = center {
            center_pos = Some(*pos);
        }
    }

    // 'O' block doesn't rotate
    if let Some(center_pos) = center_pos {
        for (_blocks, _falling, pos) in (&blocks, &falling, &mut positions).join() {
            let rel_x = pos.x - center_pos.x;
            let rel_y = pos.y - center_pos.y;

            pos.x = center_pos.x - rel_y;
            pos.y = center_pos.y + rel_x;
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