use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{Position, Text};

pub struct MenuRenderSystem<'a> {
    pub ctx: &'a mut BTerm,
}
impl<'a> System<'a> for MenuRenderSystem<'a> {
    type SystemData = (ReadStorage<'a, Text>, ReadStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (text, pos) = data;
        for (text, pos) in (&text, &pos).join() {
            self.ctx
                .print_color(pos.x, pos.y, text.fg, text.bg, &text.text);
        }
    }
}
