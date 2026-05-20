use bracket_lib::prelude::{Point, console};
use specs::prelude::*;

use crate::components::{Monster, Player, Position, Viewshed};

pub struct MonsterAIStstem {}
impl<'a> System<'a> for MonsterAIStstem {
    type SystemData = (
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (monster, player, position, viewshed) = data;
        for (_monster, mpos, viewshed) in (&monster, &position, &viewshed).join() {
            for (_player, ppos) in (&player, &position).join() {
                let pp = Point::new(ppos.x, ppos.y);
                if viewshed.visible_tiles.contains(&pp) {
                    console::log(format!("Monster found player and shouted"));
                }
            }
        }
    }
}
