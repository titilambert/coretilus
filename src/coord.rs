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
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    // Setter pour x
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    // Setter pour y
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum XTermPosition {
    Coord(i32),
    LeftOut,
    LeftIn,
    Middle,
    RightOut,
    RightIn,
}

#[derive(Clone, Copy, Debug)]
pub enum YTermPosition {
    Coord(i32),
    BottomOut,
    BottomIn,
    Middle,
    TopOut,
    TopIn,
}

/// A position in terminal space, which can be a coordinate or a relative position.
/// ```rust
/// use coretilus::engine::Size;
/// use coretilus::coord::{Coord, Position, XTermPosition, YTermPosition};
///
/// let position = Position::new(XTermPosition::Middle, YTermPosition::Middle);
/// let terminal_size = Size::new(80, 24);
/// let sprite_size = Size::new(10, 5);
/// let resolved_coord = position.resolve(terminal_size, sprite_size);
/// assert_eq!(resolved_coord.x(), 35);
/// assert_eq!(resolved_coord.y(), 9);
///
/// let position = Position::new(XTermPosition::LeftIn, YTermPosition::BottomIn);
/// let resolved_coord = position.resolve(terminal_size, sprite_size);
/// assert_eq!(resolved_coord.x(), 0);
/// assert_eq!(resolved_coord.y(), 0);
///
/// let position = Position::new(XTermPosition::RightOut, YTermPosition::TopOut);
/// let resolved_coord = position.resolve(terminal_size, sprite_size);
/// assert_eq!(resolved_coord.x(), 80);
/// assert_eq!(resolved_coord.y(), 24);
///
/// let position = Position::new(XTermPosition::LeftOut, YTermPosition::BottomOut);
/// let resolved_coord = position.resolve(terminal_size, sprite_size);
/// assert_eq!(resolved_coord.x(), -11);
/// assert_eq!(resolved_coord.y(), -6);
///
/// let position = Position::new(XTermPosition::RightIn, YTermPosition::TopIn);
/// let resolved_coord = position.resolve(terminal_size, sprite_size);
/// assert_eq!(resolved_coord.x(), 70);
/// assert_eq!(resolved_coord.y(), 19);
///
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: XTermPosition,
    y: YTermPosition,
}

impl Position {
    pub fn new(x: XTermPosition, y: YTermPosition) -> Self {
        Self { x, y }
    }

    pub fn resolve(&self, terminal_size: Size, sprite_size: Size) -> Coord {
        let x = match self.x {
            XTermPosition::Coord(n) => n,
            XTermPosition::LeftIn => 0,
            XTermPosition::LeftOut => -1 - sprite_size.width() as i32,
            XTermPosition::Middle => (terminal_size.width() - sprite_size.width()) as i32 / 2,
            XTermPosition::RightIn => (terminal_size.width() - sprite_size.width()) as i32,
            XTermPosition::RightOut => (terminal_size.width()) as i32,
        };
        let y = match self.y {
            YTermPosition::Coord(n) => n,
            YTermPosition::BottomIn => 0,
            YTermPosition::BottomOut => -1 - sprite_size.height() as i32,
            YTermPosition::Middle => (terminal_size.height() - sprite_size.height()) as i32 / 2,
            YTermPosition::TopIn => (terminal_size.height() - sprite_size.height()) as i32,
            YTermPosition::TopOut => terminal_size.height() as i32,
        };
        Coord::new(x, y)
    }
}
