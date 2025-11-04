use crate::{coord::Coord, frame::Frame};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationKind {
    Empty,
    Static,        // Just one frame
    TickBased,     // tick-based, non-looping
    MovementBased, // change frame quand le sprite bouge
}

// Animation classique
pub struct Animation {
    frames: Vec<Frame>,
    current_frame_id: usize,
    start_frame_id: usize,
    // Number of ticks that each frame will be showed
    default_ticks_per_frame: usize,
    kind: AnimationKind,
    duration: usize,
    is_loop: bool,
    is_done: bool,
    started_tick_id: Option<usize>,
}

impl Animation {
    pub fn new_empty() -> Self {
        Self {
            frames: vec![],
            current_frame_id: 0,
            start_frame_id: 0,
            default_ticks_per_frame: 0,
            kind: AnimationKind::Empty,
            duration: 0,
            is_loop: true,
            is_done: true,
            started_tick_id: None,
        }
    }
    pub fn new_static(frame: Frame) -> Self {
        Self {
            frames: vec![frame],
            current_frame_id: 0,
            start_frame_id: 0,
            default_ticks_per_frame: 0,
            kind: AnimationKind::Static,
            duration: 0,
            is_loop: true,
            is_done: false,
            started_tick_id: None,
        }
    }
    pub fn new_tick_based(
        frames: Vec<Frame>,
        start_frame_id: usize,
        default_ticks_per_frame: usize,
        duration: usize,
        is_loop: bool,
    ) -> Self {
        Self {
            frames,
            current_frame_id: start_frame_id,
            start_frame_id,
            default_ticks_per_frame,
            kind: AnimationKind::TickBased,
            duration,
            is_loop,
            is_done: false,
            started_tick_id: None,
        }
    }

    pub fn new_movement_based(frames: Vec<Frame>, start_frame_id: usize, is_loop: bool) -> Self {
        Self {
            frames,
            current_frame_id: start_frame_id,
            start_frame_id,
            default_ticks_per_frame: 0,
            kind: AnimationKind::MovementBased,
            duration: 0,
            is_loop,
            is_done: false,
            started_tick_id: None,
        }
    }

    pub fn frames(&self) -> Vec<&Frame> {
        self.frames.iter().collect()
    }

    pub fn current_frame_id(&self) -> usize {
        self.current_frame_id
    }

    pub fn kind(&self) -> AnimationKind {
        self.kind
    }

    pub fn is_loop(&self) -> bool {
        self.is_loop
    }

    fn default_ticks_per_frame(&self) -> usize {
        self.default_ticks_per_frame
    }

    pub fn is_done(&self) -> bool {
        match self.kind() {
            AnimationKind::Static => false,
            _ => self.is_done,
        }
    }

    pub fn reset(&mut self) {
        self.current_frame_id = 0;
        self.is_done = false;
        self.started_tick_id = None;
    }

    pub fn has_started(&self) -> bool {
        self.started_tick_id.is_some()
    }

    pub fn frame(&self) -> Frame {
        self.frames[self.current_frame_id].clone()
    }

    pub fn started_tick(&self) -> Option<usize> {
        self.started_tick_id
    }

    pub fn advance(
        &mut self,
        tick_id: usize,
        sprite_new_coord: Option<Coord>,
        sprite_next_coord: Option<Coord>,
    ) {
        if !self.has_started() {
            self.started_tick_id = Some(tick_id);
        }
        let start_tick = self.started_tick_id.unwrap(); // safe ici, jamais None
        if self.duration > 0 && tick_id - start_tick >= self.duration {
            self.is_done = true;
        }
        match self.kind {
            AnimationKind::Empty => {
                // Nothing to do
            }
            AnimationKind::Static => {
                // Nothing to do
            }
            AnimationKind::TickBased => {
                self.advance_tick(tick_id);
            }
            AnimationKind::MovementBased => {
                if let Some(new) = sprite_next_coord
                    && let Some(prev) = sprite_new_coord
                {
                    self.advance_movement(new, prev);
                } else {
                    panic!("DDD");
                }
            }
        }
    }

    fn get_total_ticks(&self) -> usize {
        match self.kind {
            AnimationKind::MovementBased => {
                panic!("Non sense")
            }
            _ => self
                .frames()
                .iter()
                .map(|f| {
                    if f.ticks() > 0 {
                        f.ticks()
                    } else {
                        self.default_ticks_per_frame()
                    }
                })
                .sum(),
        }
    }

    fn advance_tick(&mut self, tick_id: usize) {
        let start_tick = self.started_tick_id.unwrap(); // safe ici, jamais None
        let start_frame_offset = self.start_frame_id;
        // Initialise le tick de départ si ce n'est pas encore fait
        let elapsed_ticks = tick_id - start_tick;

        let total_ticks = self.get_total_ticks();

        if !self.is_loop && total_ticks > 0 && elapsed_ticks >= total_ticks {
            self.is_done = true;
            self.current_frame_id = self.frames.len() - 1;
            return;
        }

        // Décalage de départ en fonction du frame offset
        let offset_ticks: usize = self
            .frames
            .iter()
            .take(start_frame_offset)
            .map(|f| {
                if f.ticks() > 0 {
                    f.ticks()
                } else {
                    self.default_ticks_per_frame
                }
            })
            .sum();

        let mut acc_ticks = 0;
        let effective_tick = if total_ticks > 0 {
            let t = elapsed_ticks + offset_ticks;
            if self.is_loop {
                t % total_ticks
            } else {
                t.min(total_ticks - 1)
            }
        } else {
            elapsed_ticks
        };
        for (i, frame) in self.frames.iter().enumerate() {
            let frame_ticks = if frame.ticks() > 0 {
                frame.ticks()
            } else {
                self.default_ticks_per_frame
            };
            acc_ticks += frame_ticks;

            if effective_tick < acc_ticks {
                self.current_frame_id = i;
                return;
            }
        }

        self.current_frame_id = self.frames.len() - 1;
    }

    pub fn advance_movement(&mut self, sprite_new_coord: Coord, sprite_prev_coord: Coord) -> usize {
        if !self.is_loop && self.current_frame_id >= self.frames.len() - 1 {
            // Stay on the last frame
            self.is_done = true;
            return self.current_frame_id;
        }
        if sprite_new_coord.x() != sprite_prev_coord.x()
            || sprite_new_coord.y() != sprite_prev_coord.y()
        {
            if self.is_loop && self.current_frame_id == self.frames.len() - 1 {
                self.current_frame_id = 0;
            } else {
                self.current_frame_id += 1;
            }
            if !self.is_loop && self.current_frame_id >= self.frames.len() - 1 {
                self.is_done = true;
            }
        }
        self.current_frame_id
    }
}
