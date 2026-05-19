use crate::rect::Rect;
use crate::utils::*;
use bracket_lib::random::RandomNumberGenerator;
use std::cmp::{max, min};

#[derive(PartialEq, Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn new_map_rooms_and_corridors() -> (Vec<TileType>, Vec<Rect>) {
    let mut map = vec![TileType::Wall; 80 * 50];

    let mut rooms: Vec<Rect> = Vec::new();
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
        let ok = !rooms.iter().any(|r| r.intersect(&new_room));
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, new_x, prev_x, prev_y);
                    apply_vertical_tunnel(&mut map, new_y, prev_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, new_y, prev_y, prev_x);
                    apply_horizontal_tunnel(&mut map, new_x, prev_x, new_y);
                }
            }
            rooms.push(new_room);
        }
    }

    (map, rooms)
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[flatten_index(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = flatten_index(x, y);
        if idx > 0 && idx < MAP_SIZE {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = flatten_index(x, y);
        if idx > 0 && idx < MAP_SIZE {
            map[idx as usize] = TileType::Floor;
        }
    }
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; COL * ROW];
    for x in 0..COL {
        map[flatten_index(x as i32, 0)] = TileType::Wall;
        map[flatten_index(x as i32, (ROW - 1) as i32)] = TileType::Wall;
    }
    for y in 0..ROW {
        map[flatten_index(0, y as i32)] = TileType::Wall;
        map[flatten_index((COL - 1) as i32, y as i32)] = TileType::Wall;
    }

    let mut rng = RandomNumberGenerator::new();
    for _i in 0..400 {
        let x = rng.roll_dice(1, (COL - 1) as i32);
        let y = rng.roll_dice(1, (ROW - 1) as i32);
        let index = flatten_index(x, y);
        if index != flatten_index(40, 25) {
            map[index] = TileType::Wall;
        }
    }

    map
}
