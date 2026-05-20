pub const COL: usize = 80;
#[allow(unused)]
pub const ROW: usize = 50;
#[allow(unused)]
pub const MAP_SIZE: usize = COL * ROW;

pub fn flatten_index(x: i32, y: i32) -> usize {
    (y as usize * COL) + x as usize
}
pub fn index_2_xy(idx: usize, width: i32) -> (i32, i32) {
    let x = idx as i32 % width;
    let y = idx as i32 / width;
    (x, y)
}
