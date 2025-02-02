use crate::Map;

use super::{Player, Position, TileType};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // this gains access to players and positions in the world (ecs)
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let new_x: i32 = pos.x + delta_x;
        let new_y: i32 = pos.y + delta_y;

        // solution 1 - more scalable
        match map.tiles[map.xy_idx(new_x, new_y)] {
            TileType::Water => {}
            TileType::Ground => {
                pos.x = min(79, max(0, pos.x + delta_x));
                pos.y = min(49, max(0, pos.y + delta_y));
            }
        }

        // solution 2
        // if map[xy_idx(new_x, new_y)] == TileType::Water {
        //     pos.x = min(79, max(0, pos.x + delta_x));
        //     pos.y = min(49, max(0, pos.y + delta_y));
        // }
    }
}

pub fn move_to_random_room(ecs: &mut World) {
    // prendo in read mode le stanze
    let map = ecs.fetch::<Map>();
    let mut rng = rltk::RandomNumberGenerator::new();
    let room_index = rng.range(0, map.rooms.len());

    // prendo tutti i componenti Position e Player
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    // joino Position e Player, quindi difatto prendo
    // l'unica entita che ha entrambi, aka il player
    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = map.rooms[room_index].center().0;
        pos.y = map.rooms[room_index].center().1;
        println!("Player teleported")
    }
}
