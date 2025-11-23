use crate::engine_v2::coords::Coords;
use crate::engine_v2::object::object::ObjectRef;
use crate::engine_v2::position::Position;
use crate::engine_v2::size::Size;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Linear,
    Stationary,
    Circular,
    Relative,
    Manual,
    None,
}

#[derive(Clone)]
pub struct Movement {
    // private and calculated attributes
    current_coordinate_id: usize,
    path: Vec<Coords>,
    // Inputs
    start: Position,
    end: Position,
    direction: Direction,
    /// Number of ticks required for the sprite to move by one coordinate unit.
    /// A lower value means faster movement (fewer ticks per unit),
    /// while a higher value means slower movement.
    speed: i32,
    is_done: bool,
    offset: Coords,
    ttl: usize, // Number of tick to live, 0 means infinite
    parent_object: Option<ObjectRef>,
    started_tick_id: Option<usize>,
    radius: usize,
}

impl Movement {
    pub fn new_none() -> Self {
        Self {
            current_coordinate_id: 0,
            path: vec![],
            start: Position::new(
                crate::engine_v2::position::XTermPosition::Coord(0),
                crate::engine_v2::position::YTermPosition::Coord(0),
                0,
            ),
            end: Position::new(
                crate::engine_v2::position::XTermPosition::Coord(0),
                crate::engine_v2::position::YTermPosition::Coord(0),
                0,
            ),
            direction: Direction::None,
            speed: 0,
            is_done: false,
            offset: Coords::new(0, 0, 0),
            ttl: 0,
            parent_object: None,
            started_tick_id: None,
            radius: 0,
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
            offset: Coords::new(0, 0, 0),
            ttl: 0,
            parent_object: None,
            started_tick_id: None,
            radius: 0,
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
            offset: Coords::new(0, 0, 0),
            ttl,
            parent_object: None,
            started_tick_id: None,
            radius: 0,
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn has_started(&self) -> bool {
        self.started_tick_id.is_some()
    }

    pub fn started_tick(&self) -> Option<usize> {
        self.started_tick_id
    }

    pub fn get_coordinate(&self, tick_id: usize) -> Coords {
        if self.current_coordinate_id >= self.path.len() {
            return self.path[self.path.len() - 1];
        }
        self.path[self.current_coordinate_id]
    }

    pub fn predefined_path(&self) -> Vec<Coords> {
        self.path.clone()
    }

    pub fn compute_predefined_path(&mut self, terminal_size: Size, sprite_size: Size) {
        self.path = match self.direction {
            Direction::Linear => {
                let path = bresenham_path(
                    self.start.resolve(terminal_size, sprite_size),
                    self.end.resolve(terminal_size, sprite_size),
                );
                self.add_speed(path)
            }
            Direction::Relative => {
                if let Some(ref object_ref) = self.parent_object {
                    let mut parent_object_mut = object_ref.borrow_mut();
                    parent_object_mut.compute_predefined_path(terminal_size);
                    parent_object_mut.predefined_path()
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
            Direction::Circular => {
                let path = arc_path(
                    self.start.resolve(terminal_size, sprite_size),
                    self.end.resolve(terminal_size, sprite_size),
                    self.radius as i32,
                );
                self.add_speed(path)
            }
            _ => vec![],
        };
        for coord in self.path.iter_mut() {
            coord.set_x(coord.x() + self.offset.x());
            coord.set_y(coord.y() + self.offset.y());
        }
    }

    fn add_speed(&self, path: Vec<Coords>) -> Vec<Coords> {
        let mut extended_path = Vec::new();
        for coord in path {
            for _ in 0..(self.speed as usize) {
                extended_path.push(coord);
            }
        }
        extended_path
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
            let coords = self.get_coordinate(tick_id);
            if coords.x() >= terminal_size.width() as i32
                || coords.y() >= terminal_size.height() as i32
            {
                self.is_done = true;
            } else if coords.x() <= 0 - sprite_size.width() as i32
                || coords.y() <= 0 - sprite_size.height() as i32
            {
                // handle out left and bottom
                self.is_done = true;
            }
        }

        // handle ends of path
        if self.direction() == Direction::Linear {
            let start_tick = self.started_tick_id.unwrap();
            self.is_done = tick_id - start_tick >= self.path.len();
        }
        if self.direction() == Direction::Circular {
            let start_tick = self.started_tick_id.unwrap();
            self.is_done = tick_id - start_tick >= self.path.len();
        }
    }
}

fn stationary_path(position: Coords, ttl: usize) -> Vec<Coords> {
    let mut points = Vec::new();
    let total_nb_frame = ttl;

    for _ in 0..total_nb_frame {
        points.push(position)
    }
    points
}

/// Génère un chemin discret le long d'un arc de cercle
/// Le centre est le milieu entre start et end
/// `radius` = rayon de la courbure (hauteur maximale de l'arc)
pub fn arc_path(start: Coords, end: Coords, radius: i32) -> Vec<Coords> {
    let mut path = Vec::new();

    // Vecteur start→end
    let dx = end.x() - start.x();
    let dy = end.y() - start.y();
    let dz = end.z() - start.z();

    // Nombre de steps = distance max
    let steps = dz.abs().max(dx.abs()).max(dy.abs()).max(1);

    for i in 0..=steps {
        let t = i as f32 / steps as f32;

        // Paramètre quadratique pour faire la courbure (sommet au milieu)
        let curve = 4.0 * t * (1.0 - t); // varie de 0 → 1 → 0

        let x = start.x() as f32 + dx as f32 * t;
        let y = start.y() as f32 + dy as f32 * t + curve * radius as f32;
        let z = start.z() as f32 + dz as f32 * t;

        let coord = Coords::new(x.round() as i32, y.round() as i32, z.round() as i32);

        if path.last().copied() != Some(coord) {
            path.push(coord);
        }
    }

    path
}

fn bresenham_path(start: Coords, end: Coords) -> Vec<Coords> {
    let mut points = Vec::new();

    let delta_x = (end.x() - start.x()).abs();
    let delta_y = (end.y() - start.y()).abs();
    let delta_z = (end.z() - start.z()).abs();

    let step_x = if start.x() < end.x() { 1 } else { -1 };
    let step_y = if start.y() < end.y() { 1 } else { -1 };
    let step_z = if start.z() < end.z() { 1 } else { -1 };

    // FIXME better handle z
    let mut error = delta_x - delta_y;

    let mut current = start;

    loop {
        points.push(current);

        if current.x() == end.x() && current.y() == end.y() && current.z() == end.z() {
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

        if error2 < delta_z {
            error += delta_z;
            current.set_z(current.z() + step_z);
        }
    }

    points
}
