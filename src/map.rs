use crate::rect::Rect;
use crate::utils::*;
use bracket_lib::{
    prelude::{Algorithm2D, BaseMap, Point},
    random::RandomNumberGenerator,
};
use std::cmp::{max, min};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
impl BaseMap for Map {
    fn is_opaque(&self, _idx: usize) -> bool {
        self.tiles[_idx] == TileType::Wall
    }
}
impl Map {
    pub fn new_map_rooms_and_corridors() -> Self {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_SIZE],
            rooms: Vec::new(),
            width: COL as i32,
            height: ROW as i32,
            revealed_tiles: vec![false; MAP_SIZE],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let ok = !map.rooms.iter().any(|r| r.intersect(&new_room));
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(new_x, prev_x, prev_y);
                        map.apply_vertical_tunnel(new_y, prev_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(new_y, prev_y, prev_x);
                        map.apply_horizontal_tunnel(new_x, prev_x, new_y);
                    }
                }
                map.rooms.push(new_room);
            }
        }

        map
    }

    #[allow(unused)]
    /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
    /// look awful.
    pub fn new_map_test() -> Self {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_SIZE],
            rooms: Vec::new(),
            width: COL as i32,
            height: ROW as i32,
            revealed_tiles: vec![false; MAP_SIZE],
        };
        for x in 0..map.width {
            map.tiles[flatten_index(x as i32, 0)] = TileType::Wall;
            map.tiles[flatten_index(x as i32, (ROW - 1) as i32)] = TileType::Wall;
        }
        for y in 0..map.height {
            map.tiles[flatten_index(0, y as i32)] = TileType::Wall;
            map.tiles[flatten_index((COL - 1) as i32, y as i32)] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();
        for _i in 0..400 {
            let x = rng.roll_dice(1, (map.width - 1) as i32);
            let y = rng.roll_dice(1, (map.height - 1) as i32);
            let index = flatten_index(x, y);
            if index != flatten_index(40, 25) {
                map.tiles[index] = TileType::Wall;
            }
        }

        map
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                self.tiles[flatten_index(x, y)] = TileType::Floor;
            }
        }
    }
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = flatten_index(x, y);
            if idx > 0 && idx < MAP_SIZE {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = flatten_index(x, y);
            if idx > 0 && idx < MAP_SIZE {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
}
