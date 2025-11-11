use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crossterm::event::KeyCode;

use crate::animation::{Animation, AnimationKind};
use crate::collision::Collider;
use crate::coord::{Coord, Position, XTermPosition, YTermPosition};
use crate::engine::Size;
use crate::frame::Frame;
use crate::trajectory::{Direction, Trajectory};
use uuid::Uuid;

pub type SpriteRef = Rc<RefCell<Sprite>>;
type SpriteAction = Box<dyn Fn(&SpriteRef)>;

pub struct Sprite {
    id: Uuid,
    tdid: u64,
    tdname: String,
    trajectory: Trajectory,
    animation: Animation,
    visible: bool,
    pub layer: i32,
    // map key -> action
    pub input_actions: HashMap<KeyCode, SpriteAction>,
    // collider
    collider: Collider,
}

impl Sprite {
    pub fn new(tdid: u64, tdname: String, layer: i32) -> SpriteRef {
        let trajectory = Trajectory::new_none(
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            Direction::None,
        );
        let id = Uuid::new_v4();
        Rc::new(RefCell::new(Self {
            id,
            tdid,
            tdname,
            trajectory,
            animation: Animation::new_static(Frame::new("")),
            visible: true,
            layer,
            input_actions: HashMap::new(),
            collider: Collider::new(Coord::new(0, 0), Size::new(0, 0), true),
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

    // State
    pub fn is_done(&self) -> bool {
        if !self.visible {
            return false;
        }
        match self.trajectory.direction() {
            Direction::Linear => self.trajectory.is_done(),
            Direction::Stationary => {
                if self.animation.kind() == AnimationKind::Static {
                    return true;
                }
                if self.animation.kind() == AnimationKind::TickBased {
                    return self.animation.is_done();
                }
                self.animation.is_done()
            }
            Direction::Relative => self.trajectory.is_done(),
            _ => self.animation.is_done(),
        }
    }

    // Go to the next tick
    pub fn advance(&mut self, tick_id: usize, terminal_size: Size) {
        if self.tdid() == 16 {
            print!("FFF");
        }
        let new_coord = self.get_coordinate(tick_id);
        let next_coord = self.get_coordinate(tick_id + 1);
        self.trajectory.advance(tick_id, terminal_size, self.size());
        self.animation
            .advance(tick_id, Some(new_coord), Some(next_coord));
    }

    // Frame related
    pub fn size(&self) -> Size {
        Size::new(self.get_max_frame_width(), self.get_max_frame_height())
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
    pub fn set_visible(&mut self, is_visible: bool) {
        self.visible = is_visible;
    }

    pub fn current_frame(&self) -> Frame {
        self.animation.frame()
    }

    pub fn get_max_frame_width(&self) -> u32 {
        self.animation
            .frames()
            .iter()
            .map(|frame| frame.get_width())
            .max()
            .unwrap()
    }

    pub fn get_max_frame_height(&self) -> u32 {
        self.animation
            .frames()
            .iter()
            .map(|frame| frame.get_height())
            .max()
            .unwrap()
    }

    // Trajectory and position related
    fn get_coordinate(&mut self, tick_id: usize) -> Coord {
        self.trajectory.get_coordinate(tick_id)
    }
    pub fn current_coordinate(&self) -> Coord {
        self.trajectory.current_coordinate()
    }

    pub fn set_trajectory(&mut self, trajectory: Trajectory) {
        self.trajectory = trajectory;
    }

    pub fn trajectory(&mut self) -> &mut Trajectory {
        &mut self.trajectory
    }

    pub fn compute_path(&mut self, terminal_size: Size) {
        self.trajectory.compute_path(terminal_size, self.size());
    }
    // Animation
    pub fn set_animation(&mut self, animation: Animation) {
        self.animation = animation;
        // If there is no collider set, we create a default one based on the animation frames
        if self.collider.is_null() {
            let height = self.get_max_frame_height();
            let width = self.get_max_frame_width();
            self.collider = Collider::new(Coord::new(0, 0), Size::new(width, height), true);
        }
    }

    pub fn animation(&self) -> &Animation {
        &self.animation
    }

    pub fn reset_animation(&mut self) {
        self.animation.reset()
    }
    // Key event
    // Used to add action based on an event
    pub fn on_key(sprite: &SpriteRef, key: KeyCode, action: impl Fn(&SpriteRef) + 'static) {
        sprite
            .borrow_mut()
            .input_actions
            .insert(key, Box::new(action));
    }

    // Used in the main loop
    pub fn handle_input(sprite: &SpriteRef, key: KeyCode) {
        let maybe_action = {
            let sprite_borrow = sprite.borrow();
            sprite_borrow.input_actions.get(&key).map(|a| a as *const _)
        };
        if let Some(action_ptr) = maybe_action {
            let action: &dyn Fn(&SpriteRef) = unsafe { &*action_ptr };
            action(sprite);
        }
    }

    // Collider
    pub fn collider(&mut self) -> &mut Collider {
        &mut self.collider
    }

    pub fn set_collider(&mut self, collider: Collider) {
        self.collider = collider;
    }
}
