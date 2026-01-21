use crate::engine_v2::entity::frame::Frame;

#[derive(Debug, Clone, Copy)]
pub enum AnimationType {
    Static,
    TickBased,
    MovementBased,
}

#[derive(Debug, Clone)]
pub struct SpriteAnimation {
    frames: Vec<Frame>,
    current_frame_index: usize,

    default_frame_ticks: usize, // default ticks per frame (TickBased)
    elapsed_ticks: usize,       // since last frame change

    animation_type: AnimationType,
    looping: bool,
    is_done: bool,
    started_tick_id: usize,
    only_when_movement_active: bool,
    stop_after_movement: bool,
}

impl SpriteAnimation {
    /// Constructor for a Static animation (single frame)
    pub fn new_static(frame: Frame) -> Self {
        Self {
            frames: vec![frame],
            current_frame_index: 0,
            default_frame_ticks: 1,
            elapsed_ticks: 0,
            animation_type: AnimationType::Static,
            looping: true,
            is_done: false,
            started_tick_id: 0,
            only_when_movement_active: false,
            stop_after_movement: false,
        }
    }

    /// Constructor for a TickBased animation
    pub fn new_tick_based(
        frames: Vec<Frame>,
        frame_ticks: usize,
        looping: bool,
        start_frame_id: Option<usize>,
        only_when_movement_active: bool,
        stop_after_movement: bool,
    ) -> Self {
        let current_frame_index = start_frame_id.unwrap_or_default();
        Self {
            frames,
            current_frame_index,
            default_frame_ticks: frame_ticks,
            elapsed_ticks: 0,
            animation_type: AnimationType::TickBased,
            looping,
            is_done: false,
            started_tick_id: 0,
            only_when_movement_active,
            stop_after_movement,
        }
    }

    /// Constructor for a MovementBased animation
    pub fn new_movement_based(frames: Vec<Frame>, looping: bool) -> Self {
        Self {
            frames,
            current_frame_index: 0,
            default_frame_ticks: 1, // not used for MovementBased
            elapsed_ticks: 0,
            animation_type: AnimationType::MovementBased,
            looping,
            is_done: false,
            started_tick_id: 0,
            only_when_movement_active: true,
            stop_after_movement: true,
        }
    }

    pub fn current_frame_index(&self) -> usize {
        self.current_frame_index
    }

    pub fn set_frame_id(&mut self, frame_id: usize) {
        self.current_frame_index = frame_id;
    }

    pub fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame_index]
    }

    pub fn reset(&mut self, start_tick_id: usize) {
        self.current_frame_index = 0;
        self.is_done = false;
        self.started_tick_id = start_tick_id;
    }

    pub fn has_started(&self) -> bool {
        self.started_tick_id > 0
    }

    pub fn started_tick(&self) -> usize {
        self.started_tick_id
    }

    pub fn advance(
        &mut self,
        tick_id: usize,
        moved_x: bool,
        moved_y: bool,
        is_movement_active: bool,
        movement_ended: bool,
    ) {
        match self.animation_type {
            AnimationType::Static => (),

            AnimationType::TickBased => {
                if self.stop_after_movement && movement_ended {
                    return;
                }
                if self.only_when_movement_active && !is_movement_active {
                    return;
                }
                self.elapsed_ticks = tick_id.saturating_sub(self.started_tick_id);
                if self.elapsed_ticks > 0
                    && self.elapsed_ticks.is_multiple_of(self.default_frame_ticks)
                {
                    self.next_frame();
                }
            }
            AnimationType::MovementBased => {
                if moved_x || moved_y {
                    self.next_frame();
                }
            }
        }
    }

    fn next_frame(&mut self) {
        if self.current_frame_index + 1 < self.frames.len() {
            self.current_frame_index += 1;
        } else if self.looping {
            self.current_frame_index = 0;
        } else {
            self.is_done = true;
        }
    }
}
