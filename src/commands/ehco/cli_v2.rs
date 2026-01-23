use crate::engine_v2::collision::LineOrientation;
use crate::engine_v2::collision::ScreenEdge;
use rand::rng;
use rand::seq::SliceRandom;

use crate::command::CommandV2;
use crate::commands::ehco::objects::get_object_parrot;
use crate::commands::ehco::objects::get_word_object;
use crate::engine_v2::collision::Collision;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::movement::Movement;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::position::Position;
use crate::engine_v2::position::XTermPosition;
use crate::engine_v2::position::YTermPosition;
use rand::Rng;

pub struct Ehco {}

impl CommandV2 for Ehco {
    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef> {
        Vec::new()
    }

    fn select_objects(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>) {
        let mut objects: Vec<ObjectRef> = Vec::new();
        let mut collisions: Vec<Collision> = Vec::new();
        let speed = 10;
        let start_x = 0;
        let start_y = 0;
        let max_y = 40;
        let start_x_delta = 3;

        let message: Vec<String> = args.collect();

        let mut words: Vec<String> = vec![];
        //
        for (index, word) in message.iter().enumerate() {
            if index == 0 {
                continue;
            }
            if word.starts_with('-') {
                continue;
            }

            // Scramble letters of each word and push them individually
            let scrambled: Vec<String> = word
                .split_whitespace()
                .map(|w| {
                    let mut chars: Vec<char> = w.chars().collect();
                    let mut rng = rng();
                    chars.shuffle(&mut rng);
                    chars.into_iter().collect::<String>()
                })
                .collect();

            words.extend(scrambled);
        }

        let parrot_object = get_object_parrot();
        let parrot_width = parrot_object.borrow().current_frame().get_width();

        for (index, word) in words.iter().enumerate() {
            let mut rng = rand::rng();

            let n: i32 = rng.random_range(0 - start_y..=max_y);
            let object = get_word_object(word.to_string());
            let y = start_y + n;
            let movement = Movement::new_linear(
                Position::new(
                    XTermPosition::Coord(
                        start_x + parrot_width as i32 - start_x_delta - word.len() as i32,
                    ),
                    YTermPosition::Coord(start_y + 1),
                    0,
                ),
                Position::new(XTermPosition::RightOut, YTermPosition::Coord(y), 10),
                speed,
            );
            object.borrow_mut().set_movement(movement);
            if index > 0 {
                object.borrow_mut().deactivate_movement();
                object.borrow_mut().set_visible(false);
            }

            objects.push(object);
        }

        let movement = Movement::new_stationary(
            Position::new(
                XTermPosition::Coord(start_x),
                YTermPosition::Coord(start_y),
                0,
            ),
            0,
        );
        parrot_object.borrow_mut().set_movement(movement);
        let parrot_id = parrot_object.borrow().id();

        for (index, word_object) in objects.iter().enumerate() {
            let word = words[index].clone();
            if index + 1 >= objects.len() {
                // Last object of the list
                let collision = Collision::new_line(
                    word_object.clone(),
                    start_x + parrot_width as i32 + word.len() as i32 + start_x_delta,
                    LineOrientation::Vertical,
                    move |_, _, _, counter, engine| {
                        if counter == word.len() * speed as usize {
                            for object in engine.objects_mut() {
                                if object.borrow().id() == parrot_id {
                                    object.borrow_mut().reset_animation(0);
                                    object.borrow_mut().deactivate_sprite();
                                }
                            }
                        }
                    },
                );
                collisions.push(collision);
                let collision = Collision::new_edge(
                    word_object.clone(),
                    ScreenEdge::RightWithObjectLeftSide,
                    move |_, counter, engine| {
                        if counter == 100 {
                            engine.stop();
                        }
                    },
                );
                collisions.push(collision);
            } else {
                // Any object of the list besises  the last one
                let next_object_shape_id = objects[index + 1].borrow().id();

                let collision = Collision::new_line(
                    word_object.clone(),
                    start_x + parrot_width as i32 + word.len() as i32 - start_x_delta + 1,
                    crate::engine_v2::collision::LineOrientation::Vertical,
                    move |_, _, _, counter, engine| {
                        if counter == 1 {
                            for object in engine.objects_mut() {
                                if object.borrow().id() == next_object_shape_id {
                                    object.borrow_mut().set_visible(true);
                                    object.borrow_mut().activate_movement();
                                }
                            }
                        }
                    },
                );
                collisions.push(collision);
            }
        }

        objects.push(parrot_object);
        (objects, collisions)
    }

    fn execute(&mut self) {
        // Collect all args except the binary name
        let (objects, collisions) = self.select_objects(std::env::args());
        let mut engine = Engine::new(objects, collisions, 0);
        engine.run();
    }
}
