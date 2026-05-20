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
    Paused,
    Running,
}

struct State {
    pub ecs: World,
    pub run_state: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.run_state {
            RunState::Paused => {
                ctx.cls();
                self.run_menu_systems(ctx);
                self.run_input_system(ctx);
            }
            RunState::Running => {
                ctx.cls();
                self.run_game_systems(ctx);
                self.run_input_system(ctx);
            }
        }
    }
}

impl State {
    fn run_menu_systems(&mut self, ctx: &mut BTerm) {
        MenuRenderSystem { ctx }.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn run_game_systems(&mut self, ctx: &mut BTerm) {
        MapRenderSystem { ctx }.run_now(&self.ecs);
        EntityRenderSystem { ctx }.run_now(&self.ecs);

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
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        run_state: RunState::Paused,
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
