use super::{Map, Position, RandomMovement, TileType};
use rltk::RandomNumberGenerator;
use specs::prelude::*;

pub struct RandomMovementSystem;

impl<'a> System<'a> for RandomMovementSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, RandomMovement>,
    );

    fn run(&mut self, (mut positions, map, random_movement): Self::SystemData) {
        let mut rng = RandomNumberGenerator::new();

        let do_move = rng.range(0, 101);
        for (pos, rand_mov) in (&mut positions, &random_movement).join() {
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
