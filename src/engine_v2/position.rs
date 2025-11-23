use crate::engine_v2::{coords::Coords, size::Size};

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
    z: i32,
}

impl Position {
    pub fn new(x: XTermPosition, y: YTermPosition, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn resolve(&self, terminal_size: Size, sprite_size: Size) -> Coords {
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
        Coords::new(x, y, self.z)
    }
}
