use crate::map::Map;

use super::{Position, Viewshed};
use bracket_lib::prelude::{Point, field_of_view};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (map, mut viewshed, position): Self::SystemData) {
        for (viewshed, pos) in (&mut viewshed, &position).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
        }
    }
}
