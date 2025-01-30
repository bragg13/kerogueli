use super::{xy_idx, Player, Position, State, TileType};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // this gains access to players and positions in the world (ecs)
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let new_x: i32 = pos.x + delta_x;
        let new_y: i32 = pos.y + delta_y;

        // solution 1 - more scalable
        match map[xy_idx(new_x, new_y)] {
            TileType::Wall => {}
            TileType::Floor => {
                pos.x = min(79, max(0, pos.x + delta_x));
                pos.y = min(49, max(0, pos.y + delta_y));
            }
        }

        // solution 2
        // if map[xy_idx(new_x, new_y)] == TileType::Floor {
        //     pos.x = min(79, max(0, pos.x + delta_x));
        //     pos.y = min(49, max(0, pos.y + delta_y));
        // }
    }
}

// qui per leggere la tastiera
pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {} // nothing happened
        Some(key) => match key {
            // movement
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),

            // boh
            VirtualKeyCode::Space => println!("SPACEBAR is useless rn"),

            // matchall
            _ => {}
        },
    }
}
