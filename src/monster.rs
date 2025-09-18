use crate::Renderable;

use super::{Map, Monster, Name, Position, Viewshed};
use rltk::{Point, RGB};
use specs::prelude::*;

pub struct MonsterSystem {}

impl<'a> System<'a> for MonsterSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, Position>, // to move the monster
        WriteExpect<'a, Map>,       // to understand how to move in the map
        WriteStorage<'a, Renderable>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Viewshed>, // see the player in a range, can be set to dirty
        ReadExpect<'a, Point>,      // for pathfinding
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut positions, mut map, mut renderable, monster, name, mut viewshed, player_pos) =
            data;

        for (viewshed, _monster, renderable, name, position) in (
            &mut viewshed,
            &monster,
            &mut renderable,
            &name,
            &mut positions,
        )
            .join()
        {
            let distance = rltk::DistanceAlg::Pythagoras
                .distance2d(Point::new(position.x, position.y), *player_pos);
            if distance < 1.5 {
                println!("Attacco!");
                return;
            }

            renderable.bg = RGB::named(rltk::BLACK);

            // if not close enough but sees the player
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("Monster {} sees the player", name.name);
                renderable.bg = RGB::named(rltk::RED);

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
