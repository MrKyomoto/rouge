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

        match *self.run_state {
            RunState::MainMenu => match self.ctx.key {
                None => {}
                Some(key) => match key {
                    VirtualKeyCode::Escape => {
                        *self.run_state = RunState::Paused;
                    }
                    _ => {}
                },
            },
            RunState::Running => {}
            RunState::Paused => match self.ctx.key {
                None => {}
                Some(key) => match key {
                    VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = -1;
                        }
                    }
                    VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = 1;
                        }
                    }
                    VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dy = -1;
                        }
                    }
                    VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dy = 1;
                        }
                    }

                    // Diagonals
                    VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = -1;
                            input.dy = -1;
                        }
                    }

                    VirtualKeyCode::Numpad7 | VirtualKeyCode::U => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = 1;
                            input.dy = -1;
                        }
                    }

                    VirtualKeyCode::Numpad3 | VirtualKeyCode::N => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = 1;
                            input.dy = 1;
                        }
                    }

                    VirtualKeyCode::Numpad1 | VirtualKeyCode::B => {
                        *self.run_state = RunState::Running;
                        for (_, input) in (&players, &mut inputs).join() {
                            input.dx = -1;
                            input.dy = 1;
                        }
                    }
                    _ => {}
                },
            },
        }
    }
}
