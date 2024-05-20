use super::prelude::*;

pub fn y_first(idx: usize, width: usize, height: usize) -> (i32, i32) {
    ((idx % width) as i32 - width.div_ceil(2) as i32,
     (idx / width) as i32 - height.div_ceil(2) as i32)
}

// TODO: Broken... easy with symmetric maps, otherwise hard (but not impossible i guess?)
pub(crate) fn morton(idx: usize, width: usize, height: usize) -> (i32, i32) {
    let mut x = 0usize;
    let mut y = 0usize;

    let bits_x = (width - 1).next_power_of_two().trailing_zeros() as usize;
    let bits_y = (height - 1).next_power_of_two().trailing_zeros() as usize;
    let bits = std::cmp::max(bits_x, bits_y);

    for i in 0..bits {
        x |= ((idx >> (2 * i)) & 1) << i;
        y |= ((idx >> (2 * i + 1)) & 1) << i;
    }
    (x as i32 - width.div_ceil(2) as i32, y as i32 - height.div_ceil(2) as i32)
}
