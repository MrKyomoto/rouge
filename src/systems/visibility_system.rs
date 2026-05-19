use crate::{components::Player, map::Map, utils::flatten_index};

use super::{Position, Viewshed};
use bracket_lib::prelude::{Point, field_of_view};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        Entities<'a>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut map, mut viewshed, position, entity, player): Self::SystemData) {
        for (ent, viewshed, pos) in (&entity, &mut viewshed, &position).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                if let Some(_p) = player.get(ent) {
                    for vis in &viewshed.visible_tiles {
                        let idx = flatten_index(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
