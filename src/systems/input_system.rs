use crate::{RunState, components::*};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct InputSystem<'a> {
    pub ctx: &'a mut BTerm,
    pub run_state: &'a mut RunState,
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
                VirtualKeyCode::Escape => match *self.run_state {
                    RunState::Paused => *self.run_state = RunState::Running,
                    RunState::Running => *self.run_state = RunState::Paused,
                },
                VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                    if *self.run_state == RunState::Running {
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = -1;
                        }
                    }
                }
                VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                    if *self.run_state == RunState::Running {
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = 1;
                        }
                    }
                }
                VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                    if *self.run_state == RunState::Running {
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dy = -1;
                        }
                    }
                }
                VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                    if *self.run_state == RunState::Running {
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dy = 1;
                        }
                    }
                }
                _ => {}
            },
        }
    }
}
