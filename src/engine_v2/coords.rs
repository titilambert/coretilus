use crate::engine::Size;
use std::ops::Add;

/// A coordinate in 2D space, represented by x and y values.
/// ```rust
/// use coretilus::coord::Coord;
///
/// let coord1 = Coord::new(4, 5);
/// assert_eq!(coord1.x(), 4);
/// assert_eq!(coord1.y(), 5);
/// let mut coord2 = Coord::new(0, 0);
/// coord2.set_x(3);
/// coord2.set_y(3);
/// let coord3 = coord1 + coord2;
/// assert_eq!(coord3.x(), 7);
/// assert_eq!(coord3.y(), 8);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords {
    x: i32,
    y: i32,
    z: i32, // Layer
}

impl Coords {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, other: Coords) -> Coords {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
