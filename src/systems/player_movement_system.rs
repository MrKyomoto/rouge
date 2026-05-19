use crate::map::Map;
use crate::TileType;
use crate::components::*;
use crate::flatten_index;
use specs::prelude::*;
use std::cmp::{max, min};

pub struct PlayerMovementSystem {}
impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, PlayerInput>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (_, inputs, map, mut positions): Self::SystemData) {
        for (input, pos) in (&inputs, &mut positions).join() {
            let dest_x = min(79, max(0, pos.x + input.dx));
            let dest_y = min(49, max(0, pos.y + input.dy));
            if map.tiles[flatten_index(dest_x, dest_y)] == TileType::Floor {
                pos.x = dest_x;
                pos.y = dest_y;
            }
        }
    }
}
