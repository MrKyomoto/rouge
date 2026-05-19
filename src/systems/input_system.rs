use crate::components::*;
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct InputSystem<'a> {
    pub ctx: &'a mut BTerm,
}

impl<'a> System<'a> for InputSystem<'a> {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, PlayerInput>);

    fn run(&mut self, (players, mut inputs): Self::SystemData) {
        for (_, input) in (&players, &mut inputs).join() {
            input.dx = 0;
            input.dy = 0;
        }

        match self.ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                    for (_, input) in (&players, &mut inputs).join() {
                        input.dx = -1;
                    }
                }
                VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                    for (_, input) in (&players, &mut inputs).join() {
                        input.dx = 1;
                    }
                }
                VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                    for (_, input) in (&players, &mut inputs).join() {
                        input.dy = -1;
                    }
                }
                VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                    for (_, input) in (&players, &mut inputs).join() {
                        input.dy = 1;
                    }
                }
                _ => {}
            },
        }
    }
}
