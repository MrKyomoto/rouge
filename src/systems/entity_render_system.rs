use crate::components::*;
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct EntityRenderSystem<'a> {
    pub ctx: &'a mut BTerm,
}
impl<'a> System<'a> for EntityRenderSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (pos, render): Self::SystemData) {
        for (pos, render) in (&pos, &render).join() {
            self.ctx
                .set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
