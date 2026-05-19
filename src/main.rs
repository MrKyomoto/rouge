use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

mod map;
mod rect;
mod utils;

use crate::map::TileType;
use crate::utils::*;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component, Debug)]
struct Player {}

#[derive(Component, Default)]
struct PlayerInput {
    dx: i32,
    dy: i32,
}

// #[derive(Component, Debug)]
struct InputSystem<'a> {
    ctx: &'a mut BTerm,
}

struct PlayerMovementSystem {}
impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, PlayerInput>,
        ReadExpect<'a, Vec<TileType>>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (_, inputs, map, mut positions): Self::SystemData) {
        for (input, pos) in (&inputs, &mut positions).join() {
            let dest_x = min(79, max(0, pos.x + input.dx));
            let dest_y = min(49, max(0, pos.y + input.dy));
            if map[flatten_index(dest_x, dest_y)] == TileType::Floor {
                pos.x = dest_x;
                pos.y = dest_y;
            }
        }
    }
}

struct MapRenderSystem<'a> {
    ctx: &'a mut BTerm,
}

impl<'a> System<'a> for MapRenderSystem<'a> {
    type SystemData = ReadExpect<'a, Vec<TileType>>;

    fn run(&mut self, map: Self::SystemData) {
        let mut x = 0;
        let mut y = 0;
        for tile in map.iter() {
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
            x += 1;
            if x > COL - 1 {
                x = 0;
                y += 1;
            }
        }
    }
}

struct EntityRenderSystem<'a> {
    ctx: &'a mut BTerm,
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

struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

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

        InputSystem { ctx }.run_now(&self.ecs);

        LeftWalker {}.run_now(&self.ecs);
        PlayerMovementSystem {}.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("ECS Roguelike")
        .build()?;

    let mut gs = State { ecs: World::new() };

    let (map, rooms) = map::new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_init_x, player_init_y) = rooms[0].center();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<PlayerInput>();

    // 玩家
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_init_x,
            y: player_init_y,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .with(PlayerInput::default())
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
