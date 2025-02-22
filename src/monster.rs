use crate::Viewshed;

use super::{Map, Monster, Position, TileType};
use rltk::RandomNumberGenerator;
use specs::prelude::*;

pub struct MonsterSystem;

impl<'a> System<'a> for MonsterSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut rng = RandomNumberGenerator::new();
        let (mut positions, map, monster, _viewshed) = data;

        let do_move = rng.range(0, 101);
        for (pos, rand_mov) in (&mut positions, &monster).join() {
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
        }
    }
}
