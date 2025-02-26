use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}
impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    // run every tick
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blocks_tile) = data;
        map.populate_blocked();

        // set to blocked on the map all the positions where there is an entity with component BlocksTile
        for (position, _blocks) in (&position, &blocks_tile).join() {
            let idx = map.xy_idx(position.x, position.y);
            map.blocked[idx] = true;
        }
    }
}
