use crate::{Block, Center, Falling, Field, Position, Held, RunStateHolder, RunState};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

pub fn read_input(world: &mut World, ctx: &Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move(world, -1),
            VirtualKeyCode::Right => try_move(world, 1),
            VirtualKeyCode::RShift => try_rotate(world),
            VirtualKeyCode::RControl |
            VirtualKeyCode::LControl => try_hold(world),
            _ => {}
        },
    }
}

fn try_hold(world: &mut World) {
    let updater = world.fetch::<LazyUpdate>();
    let entities = world.entities();
    let falling_storage = world.read_storage::<Falling>();
    let held_storage = world.write_storage::<Held>();

    for (entity, falling, held) in (&entities, (&falling_storage).maybe(), (&held_storage).maybe()).join() {
        if held.is_some() {
            updater.remove::<Held>(entity);
            updater.insert(entity, Falling{});
        } else if falling.is_some() {
            updater.insert(entity, Held {});
            updater.remove::<Falling>(entity);
        }
    }

    updater.exec(move |world| {
        if world.read_storage::<Falling>().is_empty() {
            world.fetch_mut::<RunStateHolder>().0 = RunState::ReadyToSpawn;
        }
    })
}

fn try_rotate(world: &mut World) {
    let mut field = world.fetch_mut::<Field>();
    let blocks = world.read_storage::<Block>();
    let mut positions = world.write_storage::<Position>();
    let falling = world.read_storage::<Falling>();
    let center = world.read_storage::<Center>();

    let mut falling_positions = Vec::new();
    let mut center_pos = None;
    for (_blocks, _falling, pos, center) in
        (&blocks, &falling, &mut positions, (&center).maybe()).join()
    {
        falling_positions.push(*pos);

        if let Some(_) = center {
            center_pos = Some(*pos);
        }
    }

    // 'O' block doesn't rotate
    if let Some(center_pos) = center_pos {
        let mut target_positions = Vec::new();
        for (_blocks, _falling, pos) in (&blocks, &falling, &mut positions).join() {
            let rel_x = pos.x - center_pos.x;
            let rel_y = pos.y - center_pos.y;

            let target = Position {
                x: center_pos.x - rel_y,
                y: center_pos.y + rel_x,
            };

            // TODO: Wall-kicking
            if field.is_tile_blocked(target.x, target.y) {
                return;
            }
            target_positions.push((pos, target));
        }

        for (pos, new_pos) in target_positions {
            pos.x = new_pos.x;
            pos.y = new_pos.y;
        }
        field.ghost_dirty = true;
    }
}

fn try_move(world: &mut World, x: i32) {
    let mut field = world.fetch_mut::<Field>();
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
        for (_blocks, _falling, pos) in (&blocks, &falling, &mut positions).join() {
            pos.x += x;
        }
        field.ghost_dirty = true;
    }
}
