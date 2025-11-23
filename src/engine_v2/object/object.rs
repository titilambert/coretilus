use std::{cell::RefCell, collections::HashMap, os::unix::raw::uid_t, rc::Rc};

use crossterm::event::KeyCode;
use uuid::Uuid;

use crate::engine_v2::{
    coords::Coords,
    object::{frame::Frame, movement::Movement, sprite::Sprite},
    size::Size,
};

pub type ObjectRef = Rc<RefCell<Object>>;
type ObjectAction = Box<dyn Fn(&ObjectRef)>;

pub struct Object {
    id: Uuid,
    tdid: u64,
    tdname: String,
    movement: Movement,
    //trajectory: Trajectory,
    //animation: Animation,
    sprites: Vec<Sprite>,
    active_sprite: usize,
    coords: Coords,
    coords_history: Vec<Coords>,
    visible: bool,
    // map key -> action
    pub input_actions: HashMap<KeyCode, ObjectAction>,
    // collider
    //collider: Collider,
}

impl Object {
    pub fn new(tdid: u64, tdname: String, sprites: Vec<Sprite>) -> ObjectRef {
        /*let trajectory = Trajectory::new_none(
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            Direction::None,
        );*/
        let id = Uuid::new_v4();
        Rc::new(RefCell::new(Self {
            id,
            tdid,
            tdname,
            coords: Coords::new(0, 0, 0),
            coords_history: vec![],
            movement: Movement::new_none(),
            //trajectory,
            //animation: Animation::new_static(Frame::new("")),
            sprites,
            active_sprite: 0,
            visible: true,
            input_actions: HashMap::new(),
            //collider: Collider::new(Coord::new(0, 0), Size::new(0, 0), true),
        }))
    }
    // Id
    pub fn id(&self) -> Uuid {
        self.id
    }
    // TypoDex
    pub fn tdid(&self) -> u64 {
        self.tdid
    }
    pub fn tdname(&self) -> String {
        self.tdname.clone()
    }
    // Coords
    pub fn coords(&self) -> Coords {
        self.coords
    }

    pub fn set_coords(&mut self, coords: Coords) {
        self.coords = coords;
    }
    // Sprites
    pub fn sprite(&mut self) -> &Sprite {
        &self.sprites[self.active_sprite]
    }

    // Frame
    pub fn current_frame(&self) -> &Frame {
        self.sprites[self.active_sprite].current_frame()
    }

    pub fn size(&self) -> Size {
        let frame = self.current_frame();
        Size::new(frame.get_width(), frame.get_height())
    }

    pub fn update(&mut self, tick_id: usize, terminal_size: Size) {
        let sprite = &mut self.sprites[self.active_sprite];
        self.movement.advance(tick_id, terminal_size, sprite.size());
        self.coords_history.push(self.coords);
        self.coords = self.movement.get_coordinate(tick_id);
        let mut moved = false;
        if self.coords_history.len() >= 11 {
            moved = self.coords.x() != self.coords_history[self.coords_history.len() - 1].x();
        }
        sprite.advance(tick_id, moved);

        /*
        let moved = self.coords != previous_coord;
        self.sprite_animations[self.active_animation].advance(delta_ticks, moved);
        */
    }
    // Movement
    pub fn compute_predefined_path(&mut self, terminal_size: Size) {
        self.movement
            .compute_predefined_path(terminal_size, self.size());
    }

    pub fn predefined_path(&self) -> Vec<Coords> {
        self.movement.predefined_path()
    }

    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
    }
}
