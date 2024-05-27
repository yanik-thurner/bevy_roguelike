#[allow(dead_code)]
pub trait Encoding2D {
    fn new(dimensions: (usize, usize), offset: (i32, i32)) -> Self;
    fn assert_possible(dimensions: (usize, usize));
    fn encode(&self, x: i32, y: i32) -> usize;
    fn encode_xy(&self, xy: (i32, i32)) -> usize {
        self.encode(xy.0, xy.1)
    }
    fn decode(&self, idx: usize) -> (i32, i32);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct YFirstEncoding {
    pub dimensions: (usize, usize),
    pub offset: (i32, i32),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZCurveEncoding {
    pub dimensions: (usize, usize),
    pub offset: (i32, i32),
}

impl Encoding2D for YFirstEncoding {
    fn new(dimensions: (usize, usize), offset: (i32, i32)) -> Self {
        Self::assert_possible(dimensions);
        YFirstEncoding {
            dimensions,
            offset,
        }
    }

    fn assert_possible(dimensions: (usize, usize)) {
        assert!(dimensions.0 > 0);
        assert!(dimensions.1 > 0);
    }

    fn encode(&self, x: i32, y: i32) -> usize {
        (y - self.offset.1) as usize * self.dimensions.0 + (x - self.offset.0) as usize
    }

    fn decode(&self, idx: usize) -> (i32, i32) {
        ((idx % self.dimensions.0) as i32 + self.offset.0,
         (idx / self.dimensions.0) as i32 + self.offset.1)
    }
}

impl Encoding2D for ZCurveEncoding {
    fn new(dimensions: (usize, usize), offset: (i32, i32)) -> Self {
        Self::assert_possible(dimensions);
        ZCurveEncoding {
            dimensions,
            offset,
        }
    }

    fn assert_possible(dimensions: (usize, usize)) {
        assert_eq!(dimensions.0, dimensions.1);
        assert_eq!(dimensions.0, dimensions.0.next_power_of_two());
    }

    fn encode(&self, x: i32, y: i32) -> usize {
        let x = (x - self.offset.0) as usize;
        let y = (y - self.offset.1) as usize;
        let mut z = 0;
        for i in 0..(std::mem::size_of::<usize>() * 8 / 2) {
            z |= ((x >> i) & 1) << (2 * i);
            z |= ((y >> i) & 1) << (2 * i + 1);
        }
        z
    }

    fn decode(&self, idx: usize) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        for i in 0..(std::mem::size_of::<usize>() * 8 / 2) {
            x |= ((idx >> (2 * i)) & 1) << i;
            y |= ((idx >> (2 * i + 1)) & 1) << i;
        }
        (x as i32 + self.offset.0, y as i32 + self.offset.1)
    }
}
