use std::time::{Duration, Instant};

use std::io::{Stdout, Write, stdout};

use crossterm::event::{self, Event, KeyCode, KeyModifiers, poll, read};
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use crossterm::{ExecutableCommand, cursor::Show};
use crossterm::{
    cursor::Hide,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};

use crate::engine_v2::object::object::ObjectRef;
use crate::{
    engine_v2::{scene::Scene, size::Size},
    tools::get_terminal_size,
};

pub struct Engine {
    scene: Scene, // the scene containing all objects
    tick_duration: Duration,
    terminal_size: Size,
    tick_id: usize,
    ttl: usize, // Number of tick to live, 0 means infinite
    stop_on_sigint: bool,
    objects: Vec<ObjectRef>,
}

impl Engine {
    pub fn new(objects: &mut Vec<ObjectRef>, ttl: usize) -> Self {
        let terminal_size = get_terminal_size();
        Self {
            scene: Scene::new(terminal_size),
            tick_duration: Duration::from_millis(5),
            terminal_size: terminal_size,
            tick_id: 0,
            ttl,
            stop_on_sigint: true,
            objects: objects.to_owned(),
        }
    }

    pub fn run(&mut self) {
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen).unwrap(); // Go to alternate buffer
        //self.stopped = false;
        enable_raw_mode().unwrap();
        stdout.execute(Hide).unwrap(); // Hide cursor

        // calculate the duration of a single tick
        self.tick_id = 0;
        let mut killed = false;

        for object in self.objects.iter() {
            object
                .borrow_mut()
                .compute_predefined_path(self.terminal_size);
        }

        loop {
            let tick_start_time = Instant::now();
            self.tick_id += 1;
            if self.ttl > 0 && self.tick_id >= self.ttl {
                break;
            }

            while poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = read().unwrap() {
                    match key_event.code {
                        KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            if self.stop_on_sigint {
                                killed = true;
                                break;
                            }
                        }
                        _ => {
                            self.scene.handle_input(key_event.code, &mut self.objects);
                        }
                    }
                }
            }
            if killed {
                break;
            }

            // update the scene (all objects, movements, animations, collisions, etc.)
            self.scene.update(self.tick_id, &mut self.objects);

            // build the ASCII frame from the scene
            let screen = self.scene.build_screen(self.tick_id, &mut self.objects);

            // Print on screen
            let mut buffer = String::new();
            buffer.push_str("\x1B[H"); // Move cursor to the top
            for line in screen.iter() {
                buffer.push_str(&line.iter().collect::<String>());
            }

            stdout.write_all(buffer.as_bytes()).unwrap();
            stdout.flush().unwrap();

            //  wait for the end of the tick to maintain constant tick rate
            let elapsed = tick_start_time.elapsed();
            if elapsed < self.tick_duration {
                std::thread::sleep(self.tick_duration - elapsed);
            }
        }

        disable_raw_mode().unwrap();
        stdout.execute(Show).unwrap(); // Show cursor
        stdout.execute(LeaveAlternateScreen).unwrap(); // Go back to the normal terminal
    }
}
