use bracket_lib::prelude::console;
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
        let (monster, player, pos, viewshed) = data;
        for (_monster, pos, viewshed) in (&monster, &pos, &viewshed).join() {
            console::log("Monster considers its own existence");
        }
    }
}
