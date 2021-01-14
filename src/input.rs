use specs::prelude::*;
use rltk::{Rltk,VirtualKeyCode};
use crate::{Block, Position, Field, Falling, FIELD_WIDTH, FIELD_HEIGHT};

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

    let mut falling_positions = Vec::new();
    for (_blocks, _falling, pos) in (&blocks, &falling, &mut positions).join() {
        falling_positions.push(*pos);
    }

    let mut left= FIELD_WIDTH as i32;
    let mut right = 0;
    let mut top = FIELD_HEIGHT as i32;
    let mut bottom = 0;
    for pos in falling_positions.iter() {
        left = std::cmp::min(left,pos.x);
        right = std::cmp::max(right, pos.x);
        top = std::cmp::min(top, pos.y);
        bottom = std::cmp::max(bottom, pos.y);
    }
    let mid_x = (left + right)/2;
    let mid_y = (top + bottom)/2;

    for (_blocks, _falling, pos) in (&blocks, &falling, &mut positions).join() {
        let rel_x = pos.x - mid_x;
        let rel_y = pos.y - mid_y;

        pos.x = mid_x - rel_y;
        pos.y = mid_y + rel_x;
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