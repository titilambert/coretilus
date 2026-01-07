use crate::engine_v2::coords::Coords;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::size::Size;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct Collider {
    offset: Coords,
    size: Size,
    is_active: bool,
}

impl Collider {
    pub fn new(offset: Coords, size: Size, is_active: bool) -> Self {
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
    pub fn min(&self, object_pos: Coords) -> Coords {
        object_pos + self.offset
    }
    pub fn max(&self, object_pos: Coords) -> Coords {
        object_pos
            + self.offset
            + Coords::new(self.size.width() as i32, self.size.height() as i32, 0)
    }
}

/// Represents the different edges of the screen that can be used for collision detection.
#[derive(Clone)]
pub enum ScreenEdge {
    /// The top edge of the screen aligned with the object's top side.
    TopWithObjectTopSide,
    /// The top edge of the screen aligned with the object's bottom side.
    TopWithObjectBottomSide,
    /// The left edge of the screen aligned with the object's left side.
    LeftWithObjectLeftSide,
    /// The left edge of the screen aligned with the object's right side.
    LeftWithObjectRightSide,
    /// The bottom edge of the screen aligned with the object's bottom side.
    BottomWithObjectBottomSide,
    /// The bottom edge of the screen aligned with the object's top side.
    BottomWithObjectTopSide,
    /// The right edge of the screen aligned with the object's right side.
    RightWithObjectRightSide,
    /// The right edge of the screen aligned with the object's left side.
    RightWithObjectLeftSide,
}

type SpriteCollisionCallback = Box<dyn FnMut(&ObjectRef, &ObjectRef, usize, &mut Engine)>;
type EdgeCollisionCallback = Box<dyn FnMut(&ObjectRef, usize, &mut Engine)>;

pub enum Collision {
    Object {
        a: ObjectRef,
        b: ObjectRef,
        counter: usize,
        callback: SpriteCollisionCallback,
    },
    Edge {
        a: ObjectRef,
        b: ScreenEdge,
        counter: usize,
        callback: EdgeCollisionCallback,
    },
}

impl Collision {
    pub fn new_sprite(
        a: ObjectRef,
        b: ObjectRef,
        callback: impl FnMut(&ObjectRef, &ObjectRef, usize, &mut Engine) + 'static,
    ) -> Self {
        Collision::Object {
            a,
            b,
            counter: 0,
            callback: Box::new(callback),
        }
    }

    pub fn new_edge(
        a: ObjectRef,
        b: ScreenEdge,
        callback: impl FnMut(&ObjectRef, usize, &mut Engine) + 'static,
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
            Collision::Object { counter, .. } => *counter,
            Collision::Edge { counter, .. } => *counter,
        }
    }

    pub fn is_colliding(&self, terminal_size: Size) -> bool {
        match self {
            Collision::Object { a, b, .. } => {
                if Rc::ptr_eq(a, b) {
                    panic!("Same object twice used in the collision handler");
                }
                if !(a.borrow_mut().collider().is_active() && b.borrow_mut().collider().is_active())
                {
                    return false;
                }
                let (a_coords, a_collider) = {
                    let a = a.borrow_mut();
                    (a.coords(), *a.collider())
                };
                let (b_coords, b_collider) = {
                    let b = b.borrow_mut();
                    (b.coords(), *b.collider())
                };
                let a_min = a_collider.min(a_coords);
                let b_min = b_collider.min(b_coords);
                let a_max = a_collider.max(a_coords);
                let b_max = b_collider.max(b_coords);

                a_min.x() <= b_max.x()
                    && a_max.x() > b_min.x()
                    && a_min.y() < b_max.y()
                    && a_max.y() > b_min.y()
            }
            Collision::Edge { a, b, .. } => {
                if !a.borrow_mut().collider().is_active() {
                    return false;
                }

                let (a_coords, a_collider) = {
                    let a = a.borrow_mut();
                    (a.coords(), *a.collider())
                };

                let a_min = a_collider.min(a_coords);
                let a_max = a_collider.max(a_coords);

                match b {
                    ScreenEdge::BottomWithObjectBottomSide => a_min.y() == 0,
                    ScreenEdge::BottomWithObjectTopSide => a_max.y() == 0,
                    ScreenEdge::LeftWithObjectLeftSide => a_min.x() == 0,
                    ScreenEdge::LeftWithObjectRightSide => a_min.x() == 0,
                    ScreenEdge::TopWithObjectTopSide => a_max.y() == terminal_size.height() as i32,
                    ScreenEdge::TopWithObjectBottomSide => {
                        a_max.x() == terminal_size.height() as i32
                    }
                    ScreenEdge::RightWithObjectRightSide => {
                        a_max.x() == terminal_size.width() as i32
                    }
                    ScreenEdge::RightWithObjectLeftSide => {
                        a_min.x() == terminal_size.width() as i32
                    }
                }
            }
        }
    }

    pub fn trigger(&mut self, engine: &mut Engine) {
        match self {
            Collision::Object {
                a,
                b,
                counter,
                callback,
            } => {
                *counter += 1;
                (callback)(a, b, *counter, engine);
            }
            Collision::Edge {
                a,
                counter,
                callback,
                ..
            } => {
                *counter += 1;
                (callback)(a, *counter, engine);
            }
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::animation::Animation;
    use crate::engine::Size;
    use crate::engine_v2::coords::Coords;
    //:{Position, XTermPosition, YTermPosition};
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
            Position::new(XTermPosition::Coords(30), YTermPosition::Coords(30)),
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
        let mut engine = Scene::new(0);
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
            Position::new(XTermPosition::Coords(0), YTermPosition::Coords(0)),
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
        let mut engine = Scene::new(0);
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
        let mut engine = Scene::new(0);
        let mut collisions = vec![collision];
        process_collisions(&mut engine, &mut collisions);
    }
}
*/
