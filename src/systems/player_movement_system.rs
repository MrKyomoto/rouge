use crate::TileType;
use crate::components::*;
use crate::flatten_index;
use crate::map::Map;
use bracket_lib::prelude::console;
use specs::prelude::*;
use std::cmp::{max, min};

pub struct PlayerMovementSystem {}
impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, PlayerInput>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, CombatStats>,
    );

    fn run(
        &mut self,
        (player, inputs, map, mut positions, mut viewshed, combatstates): Self::SystemData,
    ) {
        for (_player, input, pos, viewshed) in
            (&player, &inputs, &mut positions, &mut viewshed).join()
        {
            let dest_x = min(79, max(0, pos.x + input.dx));
            let dest_y = min(49, max(0, pos.y + input.dy));
            let dest_idx = flatten_index(dest_x, dest_y);
            for potential_target in map.tile_content[dest_idx].iter() {
                if let Some(_t) = combatstates.get(*potential_target) {
                    console::log(format!("From Hell's Heart, I stab thee!"));
                    return;
                }
            }

            if !map.blocked[dest_idx] {
                pos.x = dest_x;
                pos.y = dest_y;

                viewshed.dirty = true;
            }
        }
    }
}
