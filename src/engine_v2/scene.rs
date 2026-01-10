//use std::io::stdout;

use crossterm::event::KeyCode;

//use crate::engine_v2::collision::Collision;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::size::Size;

pub struct Scene {
    terminal_size: Size,
}

impl Scene {
    pub fn new(terminal_size: Size) -> Self {
        Self { terminal_size }
    }
    pub fn update(
        &mut self,
        tick_id: usize,
        objects: &mut [ObjectRef],
        //collisions: &mut [Collision],
    ) {
        // Hooks
        //self.hooks.process(&self.objects);
        for object in objects.iter() {
            object.borrow_mut().update(tick_id, self.terminal_size);
        }
        // collisions
        //for col in collisions.iter_mut() {
        //    if col.is_colliding(self.terminal_size) {
        //        col.trigger(self);
        //    }
        //}
        // Hooks
        //self.hooks.process(&self.objects);
    }

    pub fn handle_input(&self, key: KeyCode, objects: &mut [ObjectRef]) {
        for object in objects.iter() {
            let maybe_action = {
                let object_borrow = object.borrow();
                object_borrow.input_actions.get(&key).map(|a| a as *const _)
            };
            if let Some(action_ptr) = maybe_action {
                let action: &dyn Fn(ObjectRef) = unsafe { &*action_ptr };
                action(object.clone()); // ✅ Clone le Rc et passe par valeur
            }
        }
    }

    pub fn build_screen(&self, _tick_id: usize, objects: &mut [ObjectRef]) -> Vec<Vec<char>> {
        //let mut stdout = stdout();

        let mut screen: Vec<Vec<char>> = vec![
            vec![' '; self.terminal_size.width() as usize];
            self.terminal_size.height() as usize
        ];

        // Order objects by z-coordinate
        objects.sort_by_key(|object| object.borrow().coords().z());
        /*panic!(
            "{} - {} - {}\n{}- {}\n{} - {} - {}\n{} - {} - {}\n{} - {} - {}\n{} - {} - {}\n{}- {}\n{}- {}\n{}- {}\n{}- {}\n",
            objects.len(),
            objects[0].borrow().tdid(),
            objects[0].borrow().coords().z(),
            objects[1].borrow().tdid(),
            objects[1].borrow().coords().z(),
            objects[2].borrow().tdid(),
            objects[2].borrow().coords().x(),
            objects[2].borrow().coords().z(),
            objects[3].borrow().tdid(),
            objects[3].borrow().coords().x(),
            objects[3].borrow().coords().z(),
            objects[4].borrow().tdid(),
            objects[4].borrow().coords().x(),
            objects[4].borrow().coords().z(),
            objects[5].borrow().tdid(),
            objects[5].borrow().coords().x(),
            objects[5].borrow().coords().z(),
            objects[6].borrow().tdid(),
            objects[6].borrow().coords().z(),
            objects[7].borrow().tdid(),
            objects[7].borrow().coords().z(),
            objects[8].borrow().tdid(),
            objects[8].borrow().coords().z(),
            objects[9].borrow().tdid(),
            objects[9].borrow().coords().z(),
        );*/

        for objectref in objects.iter() {
            /*
            sprite.compute_path(self.terminal_size);
            if !sprite.is_visible() {
                continue;
            }

            sprite.advance(tick_id, self.terminal_size);
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
            }*/

            let object = objectref.borrow();
            let object_x = object.coords().x();
            let object_y = object.coords().y();
            let frame = object.current_frame();
            //let object_y = object_mut.coords().y().clone();

            if !object.visible() {
                continue;
            }
            // Prepare printing
            for (dy, line) in frame.get_lines().iter().rev().enumerate() {
                let screen_y = self.terminal_size.height() as i32 - 1 - (object_y + dy as i32);
                // Above the screen
                if screen_y >= self.terminal_size.height() as i32 {
                    continue;
                }
                // Below the screen
                if screen_y < 0 {
                    continue;
                }
                let chars: Vec<char> = line.chars().collect();
                for (dx, &char) in chars.iter().enumerate() {
                    let screen_x = object_x + dx as i32;
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

        screen
    }
}
