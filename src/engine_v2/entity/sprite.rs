use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;
use crate::engine_v2::size::Size;

#[derive(Debug, Clone)]
pub struct Sprite {
    animation: SpriteAnimation,
    is_active: bool,
}

impl Sprite {
    pub fn new(animation: SpriteAnimation, is_active: bool) -> Self {
        Self {
            animation,
            is_active,
        }
    }

    pub fn current_frame(&self) -> &Frame {
        self.animation.current_frame()
    }

    // active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_activate(&mut self, state: bool) {
        self.is_active = state;
    }

    // Frame related
    pub fn size(&self) -> Size {
        Size::new(
            self.current_frame().get_width(),
            self.current_frame().get_height(),
        )
    }

    // Animation related
    pub fn reset(&mut self, start_tick_id: usize) {
        self.animation.reset(start_tick_id);
    }

    pub fn set_frame_id(&mut self, frame_id: usize) {
        self.animation.set_frame_id(frame_id);
    }

    pub fn advance(
        &mut self,
        tick_id: usize,
        moved_x: bool,
        moved_y: bool,
        is_movement_active: bool,
        movement_ended: bool,
    ) {
        self.animation.advance(
            tick_id,
            moved_x,
            moved_y,
            is_movement_active,
            movement_ended,
        );
    }
}
