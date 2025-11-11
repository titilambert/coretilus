use crate::{coord::Coord, engine::Size};
use std::rc::Rc;

use crate::{engine::RenderEngine, sprite::SpriteRef};

pub fn process_collisions(engine: &mut RenderEngine, collisions: &mut [Collision]) {
    for col in collisions.iter_mut() {
        if col.is_colliding(engine.terminal_size()) {
            col.trigger(engine);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Collider {
    offset: Coord,
    size: Size,
    is_active: bool,
}

impl Collider {
    pub fn new(offset: Coord, size: Size, is_active: bool) -> Self {
        Self {
            offset,
            size,
            is_active,
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn set_active(&mut self, enabled: bool) {
        self.is_active = enabled;
    }
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn is_null(&self) -> bool {
        self.size.height() == 0 && self.size.width() == 0
    }
    pub fn min(&self, sprite_pos: Coord) -> Coord {
        sprite_pos + self.offset
    }
    pub fn max(&self, sprite_pos: Coord) -> Coord {
        sprite_pos + self.offset + Coord::new(self.size.width() as i32, self.size.height() as i32)
    }
}

pub enum ScreenEdge {
    Top,
    Left,
    Bottom,
    Right,
}

type SpriteCollisionCallback = Box<dyn FnMut(&SpriteRef, &SpriteRef, usize, &mut RenderEngine)>;
type EdgeCollisionCallback = Box<dyn FnMut(&SpriteRef, usize, &mut RenderEngine)>;

pub enum Collision {
    Sprite {
        a: SpriteRef,
        b: SpriteRef,
        counter: usize,
        callback: SpriteCollisionCallback,
    },
    Edge {
        a: SpriteRef,
        b: ScreenEdge,
        counter: usize,
        callback: EdgeCollisionCallback,
    },
}

impl Collision {
    pub fn new_sprite(
        a: SpriteRef,
        b: SpriteRef,
        callback: impl FnMut(&SpriteRef, &SpriteRef, usize, &mut RenderEngine) + 'static,
    ) -> Self {
        Collision::Sprite {
            a,
            b,
            counter: 0,
            callback: Box::new(callback),
        }
    }

    pub fn new_edge(
        a: SpriteRef,
        b: ScreenEdge,
        callback: impl FnMut(&SpriteRef, usize, &mut RenderEngine) + 'static,
    ) -> Self {
        Collision::Edge {
            a,
            b,
            counter: 0,
            callback: Box::new(callback),
        }
    }

    pub fn counter(&self) -> usize {
        match self {
            Collision::Sprite { counter, .. } => *counter,
            Collision::Edge { counter, .. } => *counter,
        }
    }

    pub fn is_colliding(&mut self, terminal_size: Size) -> bool {
        match self {
            Collision::Sprite { a, b, counter, .. } => {
                if Rc::ptr_eq(a, b) {
                    panic!("Same sprite twice used in the collision handler");
                }
                if !(a.borrow_mut().collider().is_active() && b.borrow_mut().collider().is_active())
                {
                    return false;
                }
                let (a_coord, a_collider) = {
                    let mut a = a.borrow_mut();
                    (a.current_coordinate(), *a.collider())
                };
                let (b_coord, b_collider) = {
                    let mut b = b.borrow_mut();
                    (b.current_coordinate(), *b.collider())
                };
                let a_min = a_collider.min(a_coord);
                let b_min = b_collider.min(b_coord);
                let a_max = a_collider.max(a_coord);
                let b_max = b_collider.max(b_coord);

                let colliding = a_min.x() <= b_max.x()
                    && a_max.x() > b_min.x()
                    && a_min.y() < b_max.y()
                    && a_max.y() > b_min.y();

                if colliding {
                    *counter += 1;
                }
                colliding
            }
            Collision::Edge { a, b, counter, .. } => {
                if !a.borrow_mut().collider().is_active() {
                    return false;
                }

                let (a_coord, a_collider) = {
                    let mut a = a.borrow_mut();
                    (a.current_coordinate(), *a.collider())
                };

                let a_min = a_collider.min(a_coord);
                let a_max = a_collider.max(a_coord);

                let colliding = match b {
                    ScreenEdge::Bottom => a_min.y() == 0,
                    ScreenEdge::Left => a_min.x() == 0,
                    ScreenEdge::Top => a_max.y() == terminal_size.height() as i32,
                    ScreenEdge::Right => a_max.x() == terminal_size.width() as i32,
                };
                if colliding {
                    *counter += 1;
                }
                colliding
            }
        }
    }

    pub fn trigger(&mut self, engine: &mut RenderEngine) {
        match self {
            Collision::Sprite {
                a,
                b,
                counter,
                callback,
            } => (callback)(a, b, *counter, engine),
            Collision::Edge {
                a,
                counter,
                callback,
                ..
            } => (callback)(a, *counter, engine),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::animation::Animation;
    use crate::coord::{Position, XTermPosition, YTermPosition};
    use crate::engine::Size;
    use crate::frame::Frame;
    use crate::sprite::Sprite;
    use crate::trajectory::Trajectory;

    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_collision_not_detected() {
        let frames_a = Frame::new("ascii\nggg");
        let sprite_a = Sprite::new(1, String::from("test"), 10);
        let anim = Animation::new_static(frames_a);
        sprite_a.borrow_mut().set_animation(anim);
        let trajectory = Trajectory::new_stationary(
            Position::new(XTermPosition::Coord(30), YTermPosition::Coord(30)),
            300,
        );
        let terminal_size = Size::new(30, 30);
        sprite_a.borrow_mut().set_trajectory(trajectory);
        sprite_a.borrow_mut().compute_path(terminal_size);
        let frames_b = Frame::new("ascii");
        let sprite_b = Sprite::new(1, String::from("test"), 10);
        let anim = Animation::new_static(frames_b);
        sprite_b.borrow_mut().set_animation(anim);

        let triggered = Rc::new(RefCell::new(false));
        let triggered2 = triggered.clone();
        let collision = Collision::new_sprite(sprite_a.clone(), sprite_b.clone(), {
            move |_, _, _, _| {
                *triggered2.borrow_mut() = true;
            }
        });
        let mut engine = RenderEngine::new(0);
        let mut collisions = vec![collision];
        process_collisions(&mut engine, &mut collisions);
        assert!(!*triggered.borrow());
    }

    #[test]
    fn test_collision_detected() {
        let frames_a = Frame::new("ascii\nline2");
        let sprite_a = Sprite::new(1, String::from("test"), 10);
        let anim = Animation::new_static(frames_a);
        sprite_a.clone().borrow_mut().set_animation(anim);
        let trajectory = Trajectory::new_stationary(
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            300,
        );
        let terminal_size = Size::new(30, 30);
        sprite_a.borrow_mut().set_trajectory(trajectory);
        sprite_a.borrow_mut().compute_path(terminal_size);
        let frames_b = Frame::new("ascii");
        let sprite_b = Sprite::new(1, String::from("test"), 10);
        let anim = Animation::new_static(frames_b);
        sprite_b.clone().borrow_mut().set_animation(anim);

        let triggered = Rc::new(RefCell::new(false));

        let triggered2 = triggered.clone();
        let collision = Collision::new_sprite(sprite_a.clone(), sprite_b.clone(), {
            move |_, _, _, _| {
                *triggered2.borrow_mut() = true;
            }
        });
        let mut engine = RenderEngine::new(0);
        let mut collisions = vec![collision];
        process_collisions(&mut engine, &mut collisions);

        assert!(*triggered.borrow());

        *triggered.borrow_mut() = false;
        sprite_a.borrow_mut().collider().set_active(false);
        assert_eq!(collisions[0].counter(), 1);
        assert!(!sprite_a.borrow_mut().collider().is_active());
        process_collisions(&mut engine, &mut collisions);
        assert!(!*triggered.borrow());
        sprite_a.borrow_mut().collider().set_active(true);
        assert!(sprite_a.borrow_mut().collider().is_active());
    }

    #[test]
    #[should_panic(expected = "Same sprite twice used in the collision handler")]
    fn test_collision_same_sprite_panics() {
        let frames_a = Frame::new("ascii\nline2");
        let sprite_a = Sprite::new(1, String::from("test"), 10);
        let anim = Animation::new_static(frames_a);
        sprite_a.borrow_mut().set_animation(anim);

        let collision =
            Collision::new_sprite(sprite_a.clone(), sprite_a.clone(), move |_, _, _, _| {});
        let mut engine = RenderEngine::new(0);
        let mut collisions = vec![collision];
        process_collisions(&mut engine, &mut collisions);
    }
}
