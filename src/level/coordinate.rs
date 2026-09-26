pub struct CameraCoords {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl CameraCoords {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        CameraCoords { x, y, z }
    }
}

#[derive(Copy, Clone)]
pub struct MapCoords {
    pub x: usize,
    pub z: usize,
}

impl MapCoords {
    pub const fn new(x: usize, z: usize) -> Self {
        MapCoords { x, z }
    }

    pub fn add(&mut self, to: (isize, isize)) -> (usize, usize) {
        (
            self.x.saturating_add_signed(to.0),
            self.z.saturating_add_signed(to.1),
        )
    }
}
