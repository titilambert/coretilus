use crate::coord::Coord;
use crate::coord::Position;
use crate::coord::XTermPosition;
use crate::coord::YTermPosition;
use crate::engine::Size;
use crate::sprite::SpriteRef;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Linear,
    Stationary,
    Relative,
    None,
}

#[derive(Clone)]
pub struct Trajectory {
    // private and calculated attributes
    current_coordinate_id: usize,
    path: Vec<Coord>,
    // Inputs
    start: Position,
    end: Position,
    direction: Direction,
    /// Number of ticks required for the sprite to move by one coordinate unit.
    /// A lower value means faster movement (fewer ticks per unit),
    /// while a higher value means slower movement.
    speed: i32,
    is_done: bool,
    offset: Coord,
    ttl: usize, // Number of tick to live, 0 means infinite
    parent_sprite: Option<SpriteRef>,
    started_tick_id: Option<usize>,
}

impl Trajectory {
    pub fn new_none(start: Position, end: Position, direction: Direction) -> Self {
        if direction == Direction::Stationary {
            panic!("Direction2::Static must be created using Trajectory::new_static()");
        }
        Self {
            current_coordinate_id: 0,
            path: vec![],
            start,
            end,
            direction,
            speed: 0,
            is_done: false,
            offset: Coord::new(0, 0),
            ttl: 0,
            parent_sprite: None,
            started_tick_id: None,
        }
    }

    pub fn new_linear(start: Position, end: Position, speed: i32) -> Self {
        Self {
            current_coordinate_id: 0,
            path: vec![],
            start,
            end,
            direction: Direction::Linear,
            speed,
            is_done: false,
            offset: Coord::new(0, 0),
            ttl: 0,
            parent_sprite: None,
            started_tick_id: None,
        }
    }

    pub fn new_stationary(position: Position, ttl: usize) -> Self {
        Self {
            current_coordinate_id: 0,
            path: vec![],
            start: position,
            end: position,
            direction: Direction::Stationary,
            speed: 0,
            is_done: false,
            offset: Coord::new(0, 0),
            ttl,
            parent_sprite: None,
            started_tick_id: None,
        }
    }

    pub fn new_relative(sprite: SpriteRef, offset: Coord) -> Self {
        Self {
            current_coordinate_id: 0,
            path: vec![],
            start: Position::new(XTermPosition::LeftOut, YTermPosition::BottomOut),
            end: Position::new(XTermPosition::LeftOut, YTermPosition::BottomOut),
            direction: Direction::Relative,
            speed: -1,
            is_done: false,
            offset,
            ttl: 0,
            parent_sprite: Some(sprite),
            started_tick_id: None,
        }
    }

    pub fn add_offset(&mut self, coord: Coord) {
        self.offset = self.offset + coord;
    }

    pub fn offset(&self) -> Coord {
        self.offset
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn speed(&self) -> i32 {
        self.speed
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    pub fn path(&self) -> Vec<Coord> {
        self.path.clone()
    }

    pub fn has_started(&self) -> bool {
        self.started_tick_id.is_some()
    }

    pub fn started_tick(&self) -> Option<usize> {
        self.started_tick_id
    }

    pub fn advance(&mut self, tick_id: usize, terminal_size: Size, sprite_size: Size) {
        if !self.has_started() {
            self.started_tick_id = Some(tick_id);
        }
        self.current_coordinate_id += 1;
        if self.ttl > 0 && tick_id >= self.ttl {
            self.is_done = true;
        }
        if self.direction() == Direction::Relative {
            // Handle out right and top
            let coord = self.get_coordinate(tick_id);
            if coord.x() >= terminal_size.width() as i32
                || coord.y() >= terminal_size.height() as i32
            {
                self.is_done = true;
            } else if coord.x() <= 0 - sprite_size.width() as i32
                || coord.y() <= 0 - sprite_size.height() as i32
            {
                // handle out left and bottom
                self.is_done = true;
            }
        }

        // handle ends of path
        if self.direction() == Direction::Linear {
            let start_tick = self.started_tick_id.unwrap(); // safe ici, jamais None
            self.is_done = tick_id - start_tick >= self.path.len();
        }
    }

    pub fn compute_path(&mut self, terminal_size: Size, sprite_size: Size) {
        self.path = match self.direction {
            Direction::Linear => {
                let path = bresenham_path(
                    self.start.resolve(terminal_size, sprite_size),
                    self.end.resolve(terminal_size, sprite_size),
                );
                self.add_speed(path)
            }
            Direction::Relative => {
                if let Some(ref sprite_ref) = self.parent_sprite {
                    let parent_sprite_size = sprite_ref.borrow().size();
                    let mut parent_sprite_mut = sprite_ref.borrow_mut();
                    parent_sprite_mut
                        .movement()
                        .compute_path(terminal_size, parent_sprite_size);
                    //panic!("{:#?}", self.offset().x());
                    parent_sprite_mut.movement().path()
                } else {
                    panic!("No parent sprite")
                }
            }
            Direction::Stationary => {
                let start_coord = self.start.resolve(terminal_size, sprite_size);
                let mut path = vec![start_coord];
                if self.ttl > 0 {
                    path = stationary_path(start_coord, self.ttl);
                }
                path
            }
            _ => vec![],
        };
        for coord in self.path.iter_mut() {
            coord.set_x(coord.x() + self.offset.x());
            coord.set_y(coord.y() + self.offset.y());
        }
    }

    pub fn get_coordinate(&self, tick_id: usize) -> Coord {
        match self.direction {
            Direction::Stationary => self.path[0],
            _ => {
                if !self.has_started() {
                    return self.path[0];
                }
                let start_tick_id = self.started_tick_id.unwrap();
                if tick_id - start_tick_id >= self.path.len() {
                    return self.path[self.path.len() - 1];
                }
                self.path[tick_id - start_tick_id]
            }
        }
    }

    fn add_speed(&self, path: Vec<Coord>) -> Vec<Coord> {
        let mut extended_path = Vec::new();
        for coord in path {
            for _ in 0..(self.speed as usize) {
                extended_path.push(coord);
            }
        }
        extended_path
    }

    pub fn current_coordinate(&self) -> Coord {
        if self.direction() == Direction::Relative
            && let Some(ref sprite_ref) = self.parent_sprite
        {
            let parent_coord = sprite_ref.borrow().current_coordinate();
            let coord = parent_coord;
            return coord + self.offset;
        }

        if self.path.is_empty() {
            // FIXME, use start.resolve
            return Coord::new(0, 0);
        }
        self.path[std::cmp::min(self.current_coordinate_id, self.path.len() - 1)]
    }
}

fn stationary_path(position: Coord, ttl: usize) -> Vec<Coord> {
    let mut points = Vec::new();
    let total_nb_frame = ttl;

    for _ in 0..total_nb_frame {
        points.push(position)
    }
    points
}
fn bresenham_path(start: Coord, end: Coord) -> Vec<Coord> {
    let mut points = Vec::new();

    let delta_x = (end.x() - start.x()).abs();
    let delta_y = (end.y() - start.y()).abs();

    let step_x = if start.x() < end.x() { 1 } else { -1 };
    let step_y = if start.y() < end.y() { 1 } else { -1 };

    let mut error = delta_x - delta_y;

    let mut current = start;

    loop {
        points.push(current);

        if current.x() == end.x() && current.y() == end.y() {
            break;
        }

        let error2 = 2 * error;

        if error2 > -delta_y {
            error -= delta_y;
            current.set_x(current.x() + step_x);
        }

        if error2 < delta_x {
            error += delta_x;
            current.set_y(current.y() + step_y);
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::Animation;
    use crate::coord::{Coord, Position, XTermPosition, YTermPosition};
    use crate::engine::Size;
    use crate::frame::Frame;
    use crate::sprite::Sprite;

    #[test]
    fn test_bresenham_path_horizontal() {
        let start = Coord::new(0, 0);
        let end = Coord::new(5, 0);
        let path = bresenham_path(start, end);
        assert_eq!(path.len(), 6);
        assert!(
            path.windows(2)
                .all(|w| w[1].x() == w[0].x() + 1 && w[1].y() == w[0].y())
        );
    }

    #[test]
    fn test_bresenham_path_vertical() {
        let start = Coord::new(0, 0);
        let end = Coord::new(0, 5);
        let path = bresenham_path(start, end);
        assert_eq!(path.len(), 6);
        assert!(
            path.windows(2)
                .all(|w| w[1].y() == w[0].y() + 1 && w[1].x() == w[0].x())
        );
    }

    #[test]
    fn test_bresenham_path_diagonal() {
        let start = Coord::new(0, 0);
        let end = Coord::new(5, 5);
        let path = bresenham_path(start, end);
        assert_eq!(path.len(), 6);
        assert!(
            path.windows(2)
                .all(|w| w[1].x() == w[0].x() + 1 || w[1].y() == w[0].y() + 1)
        );
    }

    #[test]
    fn test_stationary_path() {
        let position = Coord::new(10, 20);
        let ttl = 5;
        let path = stationary_path(position, ttl);
        assert_eq!(path.len(), 5);
        assert!(path.iter().all(|coord| coord.x() == position.x()));
        assert!(path.iter().all(|coord| coord.y() == position.y()));
    }

    #[test]
    fn test_trajectory_new_linear() {
        let start = Position::new(XTermPosition::Middle, YTermPosition::Middle);
        let end = Position::new(XTermPosition::RightOut, YTermPosition::BottomOut);
        let trajectory = Trajectory::new_linear(start, end, 1);
        assert_eq!(trajectory.direction(), Direction::Linear);
        assert_eq!(trajectory.speed(), 1);
    }

    #[test]
    fn test_trajectory_new_stationary() {
        let position = Position::new(XTermPosition::Middle, YTermPosition::Middle);
        let trajectory = Trajectory::new_stationary(position, 10);
        assert_eq!(trajectory.direction(), Direction::Stationary);
        assert_eq!(trajectory.ttl, 10);
    }

    #[test]
    fn test_trajectory_new_relative() {
        let terminal_size = Size::new(20, 20);
        let start = Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0));
        let end = Position::new(XTermPosition::Coord(10), YTermPosition::Coord(10));

        let trajectory = Trajectory::new_linear(start, end, 1);
        let sprite = Sprite::new(1, "test", 10);
        sprite.borrow_mut().set_movement(trajectory.clone());
        let anim = Animation::new_movement_based(
            vec![Frame::new("FAKEFRAME10"), Frame::new("FAKEFRAME11")],
            0,
            true,
        );
        sprite.borrow_mut().set_animation(anim);

        let trajectory_relative = Trajectory::new_relative(sprite.clone(), Coord::new(4, 2));
        let sprite_rel = Sprite::new(1, "test", 10);
        sprite_rel
            .borrow_mut()
            .set_movement(trajectory_relative.clone());

        let anim_rel = Animation::new_movement_based(
            vec![Frame::new("FAKEFRAME20"), Frame::new("FAKEFRAME21")],
            0,
            true,
        );
        sprite_rel.borrow_mut().set_animation(anim_rel);

        sprite.borrow_mut().compute_path(terminal_size);
        let coord_4 = sprite.borrow().current_coordinate();
        sprite_rel.borrow_mut().compute_path(terminal_size);
        let rel_coord_4 = sprite_rel.borrow().current_coordinate();
        assert_eq!(coord_4.x() + 4, rel_coord_4.x());
        assert_eq!(coord_4.y() + 2, rel_coord_4.y());
    }

    /*
    #[test]
    fn test_trajectory_current_coordinate_relative() {
        let sprite = Sprite::new(1, "test", vec![], 10);
        let mut trajectory = Trajectory::new_relative(sprite.clone(), Coord::new(1, 1));
        let parent_coord = Coord::new(10, 10);
        let mut sprite_mut = sprite.borrow_mut();
        sprite_mut.current_coordinate = Some(parent_coord);
        let coord = trajectory.current_coordinate();
        assert_eq!(coord.x(), 11);
        assert_eq!(coord.y(), 11);
    }*/
}
