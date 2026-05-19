use crate::COL;
use crate::TileType;
use crate::components::Player;
use crate::components::Viewshed;
use crate::map::Map;
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct MapRenderSystem<'a> {
    pub ctx: &'a mut BTerm,
}

impl<'a> System<'a> for MapRenderSystem<'a> {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(&mut self, (map, player, viewshed): Self::SystemData) {
        for (_player, viewshed) in (&player, &viewshed).join() {
            let mut x = 0;
            let mut y = 0;
            for tile in map.tiles.iter() {
                let pt = Point::new(x, y);
                if viewshed.visible_tiles.contains(&pt) {
                    match tile {
                        TileType::Floor => {
                            self.ctx.set(
                                x,
                                y,
                                RGB::from_f32(0.5, 0.5, 0.5),
                                RGB::from_f32(0., 0., 0.),
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            self.ctx.set(
                                x,
                                y,
                                RGB::from_f32(0., 1., 0.),
                                RGB::from_f32(0., 0., 0.),
                                to_cp437('#'),
                            );
                        }
                    }
                }
                x += 1;
                if x > map.width - 1 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
}
