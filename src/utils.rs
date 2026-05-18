pub const COL: usize = 80;
#[allow(unused)]
pub const ROW: usize = 50;
#[allow(unused)]
pub const MAP_SIZE: usize = COL * ROW;

pub fn flatten_index(x: i32, y: i32) -> usize {
    (y as usize * COL) + x as usize
}
