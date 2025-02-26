use super::{Map, Monster, Name, Position, TileType, Viewshed};
use rltk::{Point, RandomNumberGenerator};
use specs::prelude::*;

pub struct MonsterSystem {}

impl<'a> System<'a> for MonsterSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, Position>, // to move the monster
        WriteExpect<'a, Map>,       // to understand how to move in the map
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Viewshed>, // see the player in a range, can be set to dirty
        ReadExpect<'a, Point>,      // for pathfinding
    );

    fn run(&mut self, data: Self::SystemData) {
        // let mut rng = RandomNumberGenerator::new();
        let (mut positions, mut map, monster, name, mut viewshed, player_pos) = data;

        // monsters with a viewshed, a name, a position (and a monster component clearly)
        for (mut viewshed, _monster, name, mut position) in
            (&mut viewshed, &monster, &name, &mut positions).join()
        {
            // if sees the player
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("Monster {} sees the player", name.name);

                // chase the player
                let path = rltk::a_star_search(
                    map.xy_idx(position.x, position.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map, // &map would be a reference to the smart pointer `WriteExpect<Map>`, and not to the object itself
                );

                if path.success && path.steps.len() > 1 {
                    position.x = path.steps[1] as i32 % map.width;
                    position.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
