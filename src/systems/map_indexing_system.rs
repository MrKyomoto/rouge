use specs::prelude::*;

use crate::{
    components::{BlocksTile, Position},
    map::Map,
    utils::flatten_index,
};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entity) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (pos, ent) in (&position, &entity).join() {
            let idx = flatten_index(pos.x, pos.y);

            if let Some(_p) = blockers.get(ent) {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(ent);
        }
    }
}
