use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;
use crate::engine_v2::size::Size;

#[derive(Debug, Clone)]
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

    // Animation related
    pub fn reset(&mut self, start_tick_id: usize) {
        self.animation.reset(start_tick_id);
    }

    pub fn set_frame_id(&mut self, frame_id: usize) {
        self.animation.set_frame_id(frame_id);
    }

    pub fn advance(&mut self, tick_id: usize, moved: bool) {
        self.animation.advance(tick_id, moved);
    }
}
