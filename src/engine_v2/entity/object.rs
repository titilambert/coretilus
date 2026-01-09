use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;
use std::rc::Rc;

use crossterm::event::KeyCode;
use uuid::Uuid;

use crate::engine_v2::collision::Collider;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::movement::Direction;
use crate::engine_v2::entity::movement::Movement;
use crate::engine_v2::entity::sprite::Sprite;
use crate::engine_v2::size::Size;

pub type ObjectRef = Rc<RefCell<Object>>;
type ObjectAction = Box<dyn Fn(ObjectRef)>;

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
    collider: Collider,
}

impl Object {
    pub fn new(
        tdid: u64,
        tdname: String,
        sprites: Vec<Sprite>,
        collider: Option<Collider>,
    ) -> ObjectRef {
        /*let trajectory = Trajectory::new_none(
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            Direction::None,
        );*/
        let id = Uuid::new_v4();
        let col: Collider = match collider {
            Some(c) => c,
            None => {
                let size = sprites[0].size();
                Collider::new(Coords::new(0, 0, 0), size, true)
            }
        };
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
            collider: col,
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
    pub fn set_frame_id(&mut self, frame_id: usize) {
        self.sprites[self.active_sprite].set_frame_id(frame_id);
    }

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
        //self.coords = self.movement.get_coordinate(tick_id);

        let tdid = self.tdid; // Capturer avant le catch_unwind
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            self.movement.get_coordinate(tick_id)
        }));

        self.coords = match result {
            Ok(coords) => coords,
            Err(_) => {
                eprintln!("Panic dans get_coordinate pour objet tdid: {}", tdid);
                eprintln!(
                    "Panic dans get_coordinate pour objet tdid: {:#?}",
                    self.movement().direction()
                );
                panic!("Erreur dans get_coordinate pour tdid: {}", tdid);
            }
        };

        self.coords = self.coords + self.movement.offset();
        let mut moved = true;
        if !self.coords_history.is_empty() {
            moved = self.coords.x() != (self.coords_history[self.coords_history.len() - 1].x())
                || (self.coords.y() != self.coords_history[self.coords_history.len() - 1].y());
        }
        sprite.advance(tick_id, moved);

        /*
        let moved = self.coords != previous_coord;
        self.sprite_animations[self.active_animation].advance(delta_ticks, moved);
        */
    }
    // Movement
    pub fn compute_predefined_path(&mut self, terminal_size: Size) {
        if self.movement.direction() == Direction::Linear {
            //panic!("FFFF {}", self.tdid);
        }
        self.movement
            .compute_predefined_path(terminal_size, self.size());
    }

    pub fn predefined_path(&self) -> Vec<Coords> {
        self.movement.predefined_path()
    }

    pub fn set_movement(&mut self, movement: Movement) {
        self.movement = movement;
    }

    pub fn movement(&mut self) -> &mut Movement {
        &mut self.movement
    }

    // Animation
    pub fn reset_animation(&mut self, start_tick_id: usize) {
        self.sprites[0].reset(start_tick_id);
    }

    // Collision
    pub fn collider(&self) -> &Collider {
        &self.collider
    }

    // Visible
    pub fn visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, value: bool) {
        self.visible = value;
    }

    // Key event
    // Used to add action based on an event
    pub fn on_key(object: &ObjectRef, key: KeyCode, action: impl Fn(ObjectRef) + 'static) {
        object
            .borrow_mut()
            .input_actions
            .insert(key, Box::new(action));
    }
}
