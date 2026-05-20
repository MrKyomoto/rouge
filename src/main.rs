use bracket_lib::prelude::*;
use specs::prelude::*;

mod components;
mod map;
mod rect;
mod systems;
mod utils;

use crate::components::*;
use crate::map::{Map, TileType};
use crate::systems::*;
use crate::utils::*;

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    MainMenu,
    Paused,
    Running,
}

struct State {
    pub ecs: World,
    pub run_state: RunState,
    pub frame_time: f32,
    pub fixed_dt: f32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.frame_time += ctx.frame_time_ms;
        while self.frame_time >= self.fixed_dt * 1000.0 {
            match self.run_state {
                RunState::MainMenu => {
                    self.run_input_system(ctx);

                    // NOTE: ensure that when first come into game player has viewshed
                    if self.run_state == RunState::Paused {
                        VisibilitySystem {}.run_now(&self.ecs);
                    }
                }
                RunState::Paused => {
                    self.run_input_system(ctx);
                }
                RunState::Running => {
                    // self.run_input_system(ctx);
                    self.run_game_logic_systems(ctx);
                    self.run_state = RunState::Paused;
                }
            }
            self.frame_time -= self.fixed_dt * 1000.0;
        }

        ctx.cls();
        match self.run_state {
            RunState::Paused => {
                self.run_game_render_systems(ctx);
            }
            RunState::Running => {
                self.run_game_render_systems(ctx);
            }
            RunState::MainMenu => {
                self.run_menu_render_systems(ctx);
            }
        }
    }
}

impl State {
    fn run_menu_render_systems(&mut self, ctx: &mut BTerm) {
        MenuRenderSystem { ctx }.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn run_game_render_systems(&mut self, ctx: &mut BTerm) {
        MapRenderSystem { ctx }.run_now(&self.ecs);
        EntityRenderSystem { ctx }.run_now(&self.ecs);
        self.ecs.maintain();
    }
    fn run_game_logic_systems(&mut self, _ctx: &mut BTerm) {
        VisibilitySystem {}.run_now(&self.ecs);

        PlayerMovementSystem {}.run_now(&self.ecs);

        MonsterAIStstem {}.run_now(&self.ecs);

        self.ecs.maintain();
    }
    fn run_input_system(&mut self, ctx: &mut BTerm) {
        InputSystem {
            ctx: ctx,
            run_state: &mut self.run_state,
        }
        .run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("ECS Roguelike")
        .with_fps_cap(60.0)
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        run_state: RunState::MainMenu,
        frame_time: 0.,
        fixed_dt: 1. / 60.,
    };

    gs.ecs.register::<Text>();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<PlayerInput>();
    gs.ecs.register::<Monster>();

    gs.ecs
        .create_entity()
        .with(Position { x: 20, y: 40 })
        .with(Text {
            text: "Press Esc to start".to_string(),
            fg: RGB::named(RED),
            bg: RGB::named(BLACK),
        })
        .build();

    let map = Map::new_map_rooms_and_corridors();
    let (init_x, init_y) = map.rooms[0].center();

    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        let glyph: FontCharType;
        let fg: RGB;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = to_cp437('g');
                fg = RGB::named(RED);
            }
            _ => {
                glyph = to_cp437('o');
                fg = RGB::named(BLUE);
            }
        }
        gs.ecs
            .create_entity()
            .with(Position { x: x, y: y })
            .with(Renderable {
                glyph: glyph,
                fg: fg,
                bg: RGB::named(BLACK),
            })
            .with(Monster {})
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    }

    gs.ecs.insert(map);

    // 玩家
    gs.ecs
        .create_entity()
        .with(Position {
            x: init_x,
            y: init_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(PlayerInput::default())
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    main_loop(context, gs)
}
