use crate::collision::{Collision, process_collisions};
use crate::sprite::{Sprite, SpriteRef};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
};
use crossterm::{
    event::{Event, KeyCode, KeyModifiers, poll, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{Stdout, Write, stdout};
use std::time::{Duration, Instant};
use terminal_size::{Height, Width, terminal_size};

#[derive(Debug, Clone, Copy)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

// Tools functions
/*fn clear_console() {
    // ANSI escape code to clear the screen and move cursor to (1,1)
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap(); // Ensure output is flushed immediately
}*/

pub fn get_terminal_size() -> (u16, u16) {
    match terminal_size() {
        Some((Width(w), Height(h))) => (w, h),
        None => {
            // No terminal (CI, pre-commit, etc.)
            (100, 50) // default dimensions
        }
    }
}

pub struct RenderEngine {
    tick_duration: Duration,
    terminal_size: Size,
    stop_on_sigint: bool,
    stopped: bool,
    killed: bool,
    sprites: Vec<SpriteRef>,
    tick_id: usize,
    ttl: usize, // Number of tick to live, 0 means infinite
}

impl RenderEngine {
    pub fn new(ttl: usize) -> Self {
        let (terminal_width, terminal_height) = get_terminal_size();
        let terminal_size = Size::new(terminal_width as u32, terminal_height as u32);
        Self {
            tick_duration: Duration::from_millis(5),
            terminal_size,
            stop_on_sigint: true,
            stopped: false,
            killed: false,
            sprites: vec![],
            tick_id: 0,
            ttl,
        }
    }

    pub fn killed(&self) -> bool {
        self.killed
    }

    pub fn stopped(&self) -> bool {
        self.stopped
    }

    pub fn stop(&mut self) {
        self.stopped = true
    }

    pub fn sprites(&self) -> Vec<SpriteRef> {
        self.sprites.clone()
    }

    pub fn terminal_size(&self) -> Size {
        self.terminal_size
    }

    pub fn render(&mut self, sprites: &mut [SpriteRef], collisions: &mut [Collision]) {
        self.tick_id = 0;
        self.killed = false;
        self.stopped = false;
        self.sprites = sprites.to_owned();
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen).unwrap(); // Go to alternate buffer
        self.stopped = false;
        enable_raw_mode().unwrap();
        stdout.execute(Hide).unwrap(); // Hide cursor

        self.render_scene(&mut stdout, sprites, collisions);

        loop {
            let tick_start_time = Instant::now();
            self.tick_id += 1;

            self.render_scene(&mut stdout, sprites, collisions);
            let frame_time = tick_start_time.elapsed();
            if frame_time < self.tick_duration {
                let remaining = self.tick_duration - frame_time;

                // Attend soit un événement clavier, soit la fin du tick
                if poll(remaining).unwrap()
                    && let Event::Key(key_event) = read().unwrap()
                {
                    match key_event.code {
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            if self.stop_on_sigint {
                                self.killed = true;
                                break;
                            }
                        }
                        _ => {
                            for sprite in sprites.iter() {
                                Sprite::handle_input(sprite, key_event.code);
                            }
                        }
                    }
                }
            }

            // Check if all sprites are done
            let all_sprites_are_done = sprites
                .iter()
                .filter(|s| s.borrow().is_visible())
                .all(|s| s.borrow().is_done());

            if self.stopped || (self.ttl > 0 && self.tick_id >= self.ttl) || all_sprites_are_done {
                break;
            }
        }
        disable_raw_mode().unwrap();
        stdout.execute(Show).unwrap(); // Show cursor
        stdout.execute(LeaveAlternateScreen).unwrap(); // Go back to the normal terminal
    }

    pub fn render_scene(
        &mut self,
        stdout: &mut Stdout,
        sprites: &mut [SpriteRef],
        collisions: &mut [Collision],
    ) {
        sprites.sort_by_key(|s| s.borrow().layer);
        // Create screen buffer with spaces
        let mut screen: Vec<Vec<char>> =
            vec![vec![' '; self.terminal_size.width as usize]; self.terminal_size.height as usize];

        // Order and filter and sprites
        for spriteref in sprites.iter() {
            let mut sprite = spriteref.borrow_mut();

            sprite.compute_path(self.terminal_size);
            if !sprite.is_visible() {
                continue;
            }

            sprite.advance(self.tick_id, self.terminal_size);
            let frame = sprite.animation().frame();

            let sprite_coord = sprite.current_coordinate();

            let sprite_x = sprite_coord.x();
            let sprite_y = sprite_coord.y();

            // Prepare printing
            for (dy, line) in frame.get_lines().iter().rev().enumerate() {
                let screen_y = self.terminal_size.height as i32 - 1 - (sprite_y + dy as i32);
                // Above the screen
                if screen_y >= self.terminal_size.height as i32 {
                    continue;
                }
                // Below the screen
                if screen_y < 0 {
                    continue;
                }
                let chars: Vec<char> = line.chars().collect();
                for (dx, &char) in chars.iter().enumerate() {
                    let screen_x = sprite_x + dx as i32;
                    // On the right of the screen
                    if screen_x >= self.terminal_size.width() as i32 {
                        continue;
                    }
                    // On the left of the screen
                    if screen_x < 0 {
                        continue;
                    }

                    if char != ' ' {
                        screen[screen_y as usize][screen_x as usize] = char;
                    }
                }
            }
        }

        // Collision
        process_collisions(self, collisions);

        // Print on screen
        let mut buffer = String::new();
        buffer.push_str("\x1B[H"); // Move cursor to the top
        for line in screen.iter() {
            buffer.push_str(&line.iter().collect::<String>());
        }

        stdout.write_all(buffer.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}
