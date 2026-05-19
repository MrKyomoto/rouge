use crate::components::*;
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct EntityRenderSystem<'a> {
    pub ctx: &'a mut BTerm,
}
impl<'a> System<'a> for EntityRenderSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Player>,
        Entities<'a>,
    );

    fn run(&mut self, (pos, render, viewshed, player, entity): Self::SystemData) {
        for (pos, render, ent) in (&pos, &render, &entity).join() {
            // NOTE: render player directly
            if let Some(_p) = player.get(ent) {
                self.ctx
                    .set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            } else {
                // NOTE: render other renderable only when it lies in player's viewshed tiles
                for (_player, viewshed) in (&player, &viewshed).join() {
                    let pt = Point::new(pos.x, pos.y);
                    if viewshed.visible_tiles.contains(&pt) {
                        self.ctx
                            .set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                        break;
                    }
                }
            }
        }
    }
}
