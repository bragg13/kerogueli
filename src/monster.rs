use super::{Map, Monster, Name, Position, TileType, Viewshed};
use rltk::{Point, RandomNumberGenerator};
use specs::prelude::*;

pub struct MonsterSystem;

impl<'a> System<'a> for MonsterSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Viewshed>,
        ReadExpect<'a, Point>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut rng = RandomNumberGenerator::new();
        let (mut positions, map, monster, name, viewshed, player_pos) = data;

        let do_move = rng.range(0, 101);
        for (pos, rand_mov, name, viewshed) in (&mut positions, &monster, &name, &viewshed).join() {
            // move monster
            if do_move <= rand_mov.probability {
                let offset_x = rng.range(-1, 2);
                let offset_y = rng.range(-1, 2);

                match map.tiles[map.xy_idx(pos.x + offset_x, pos.y + offset_y)] {
                    TileType::Water => {}
                    TileType::Ground => {
                        pos.x += offset_x;
                        pos.y += offset_y;
                    }
                }
            }

            // see the player
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("Monster {} sees the player", name.name);
            }
        }
    }
}
