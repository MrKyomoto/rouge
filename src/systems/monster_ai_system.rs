use bracket_lib::prelude::{DistanceAlg, Point, a_star_search, console};
use specs::prelude::*;

use crate::{
    components::{Monster, Name, Player, Position, Viewshed},
    map::Map,
    utils::{flatten_index, index_2_xy},
};

pub struct MonsterAIStstem {}
impl<'a> System<'a> for MonsterAIStstem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, monster, mut position, player, mut viewshed, name) = data;
        let mut pp = Point::new(0, 0);
        for (_player, ppos) in (&player, &mut position).join() {
            pp = Point::new(ppos.x, ppos.y);
        }
        for (_monster, mpos, viewshed, name) in
            (&monster, &mut position, &mut viewshed, &name).join()
        {
            let distance = DistanceAlg::Pythagoras.distance2d(Point::new(mpos.x, mpos.y), pp);
            if distance < 1.5 {
                console::log(format!("Monster {} found player and shouted", name.name));
                continue;
            }

            if viewshed.visible_tiles.contains(&pp) {
                let path = a_star_search(
                    flatten_index(mpos.x, mpos.y),
                    flatten_index(pp.x, pp.y),
                    &mut *map,
                );
                if path.success && path.steps.len() > 1 {
                    if !map.blocked[path.steps[1]] {
                        let pre_idx = flatten_index(mpos.x, mpos.y);
                        map.blocked[pre_idx] = false;
                        (mpos.x, mpos.y) = index_2_xy(path.steps[1], map.width);
                        viewshed.dirty = true;
                    }
                }
            }
        }
    }
}
