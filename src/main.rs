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

struct State {
    ecs: World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.run_systems(ctx);
    }
}

impl State {
    fn run_systems(&mut self, ctx: &mut BTerm) {
        MapRenderSystem { ctx }.run_now(&self.ecs);
        EntityRenderSystem { ctx }.run_now(&self.ecs);

        VisibilitySystem {}.run_now(&self.ecs);

        InputSystem { ctx }.run_now(&self.ecs);

        PlayerMovementSystem {}.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("ECS Roguelike")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<PlayerInput>();
    gs.ecs.register::<Viewshed>();

    let map = Map::new_map_rooms_and_corridors();
    let (init_x, init_y) = map.rooms[0].center();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        gs.ecs
            .create_entity()
            .with(Position { x: x, y: y })
            .with(Renderable {
                glyph: to_cp437('g'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
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

    // // 怪物
    // for i in 0..10 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20 })
    //         .with(Renderable {
    //             glyph: to_cp437('~'),
    //             fg: RGB::named(RED),
    //             bg: RGB::named(BLACK),
    //         })
    //         .with(LeftMover {})
    //         .build();
    // }
    //
    main_loop(context, gs)
}
