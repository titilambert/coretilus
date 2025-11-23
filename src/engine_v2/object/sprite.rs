use std::thread::current;

use crate::engine_v2::{
    object::{frame::Frame, sprite_animation::SpriteAnimation},
    size::Size,
};

#[derive(Clone)]
pub struct Sprite {
    animation: SpriteAnimation,
}

impl Sprite {
    pub fn new(animation: SpriteAnimation) -> Self {
        Self { animation }
    }

    pub fn current_frame(&self) -> &Frame {
        self.animation.current_frame()
    }

    // Frame related
    pub fn size(&self) -> Size {
        Size::new(
            self.current_frame().get_width(),
            self.current_frame().get_height(),
        )
    }

    pub fn advance(&mut self, tick_id: usize, moved: bool) {
        self.animation.advance(tick_id, moved);
    }
}
