use std::time::Duration;
use std::time::Instant;

use std::io::Write;
use std::io::stdout;

use crossterm::ExecutableCommand;
use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

use crate::engine_v2::collision::Collision;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::scene::Scene;
use crate::engine_v2::size::Size;
use crate::tools::get_terminal_size;

pub struct Engine {
    scene: Scene, // the scene containing all objects
    tick_duration: Duration,
    terminal_size: Size,
    tick_id: usize,
    ttl: usize, // Number of tick to live, 0 means infinite
    stop_on_sigint: bool,
    objects: Vec<ObjectRef>,
    collisions: Vec<Collision>,
    killed: bool,
    must_stop: bool,
}

impl Engine {
    pub fn new(objects: Vec<ObjectRef>, collisions: Vec<Collision>, ttl: usize) -> Self {
        let terminal_size = get_terminal_size();
        Self {
            scene: Scene::new(terminal_size),
            tick_duration: Duration::from_millis(5),
            terminal_size,
            tick_id: 0,
            ttl,
            stop_on_sigint: true,
            objects: objects.to_owned(),
            collisions,
            killed: false,
            must_stop: false,
        }
    }

    pub fn tick_id(&self) -> usize {
        self.tick_id
    }

    pub fn terminal_size(&self) -> Size {
        self.terminal_size
    }

    pub fn is_killed(&self) -> bool {
        self.killed
    }

    pub fn stop(&mut self) {
        self.must_stop = true;
    }

    pub fn objects_mut(&mut self) -> &mut Vec<ObjectRef> {
        &mut self.objects
    }

    pub fn run(&mut self) {
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen).unwrap(); // Go to alternate buffer
        //self.stopped = false;
        enable_raw_mode().unwrap();
        stdout.execute(Hide).unwrap(); // Hide cursor

        // calculate the duration of a single tick
        self.tick_id = 0;

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

            if self.must_stop {
                break;
            }

            while poll(Duration::from_millis(0)).unwrap() {
                if let Event::Key(key_event) = read().unwrap() {
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
                            self.scene.handle_input(key_event.code, &mut self.objects);
                        }
                    }
                }
            }
            if self.killed {
                break;
            }

            // update the scene (all objects, movements, animations, etc.)
            self.scene.update(self.tick_id, &mut self.objects);

            // collisions
            for i in (0..self.collisions.len()).rev() {
                let terminal_size = self.terminal_size;

                // Retirer temporairement l'élément
                let mut collision = self.collisions.remove(i);

                if collision.is_colliding(terminal_size) {
                    collision.trigger(self);
                }

                // Remettre à sa place
                self.collisions.insert(i, collision);
            }

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
