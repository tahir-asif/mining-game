use std::fmt::{self, Display};

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

#[derive(Copy, Clone, PartialEq)]
pub struct MapCoords {
    pub x: u8,
    pub z: u8,
}

impl Display for MapCoords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.z)
    }
}

impl MapCoords {
    pub const fn new(x: u8, z: u8) -> Self {
        MapCoords { x, z }
    }

    pub fn add(&mut self, to: (i8, i8)) -> Self {
        MapCoords {
            x: self.x.saturating_add_signed(to.0),
            z: self.z.saturating_add_signed(to.1),
        }
    }
}
