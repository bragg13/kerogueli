use crate::{Map, Viewshed};

use super::{Player, Position};
use rltk::Point;
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // this gains access to players and positions in the world (ecs)
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viesheds = ecs.write_storage::<Viewshed>();
    let mut ppos = ecs.write_resource::<Point>();

    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viesheds).join() {
        let new_x: i32 = pos.x + delta_x;
        let new_y: i32 = pos.y + delta_y;

        if !map.blocked[map.xy_idx(new_x, new_y)] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            viewshed.dirty = true;

            // update the point on the ecs
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
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
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    // joino Position e Player, quindi difatto prendo
    // l'unica entita che ha entrambi, aka il player
    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        pos.x = map.rooms[room_index].center().0;
        pos.y = map.rooms[room_index].center().1;
        viewshed.dirty = true;
        println!("Player teleported")
    }
}
