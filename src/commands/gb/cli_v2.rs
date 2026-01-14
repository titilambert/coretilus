use std::vec;

use crossterm::event::KeyCode;

use crate::commands::gb::objects::get_object_bottom;
use crate::commands::gb::objects::get_object_gameboy;
use crate::commands::gb::objects::get_object_shape1;
use crate::commands::gb::objects::get_object_shape2;
use crate::commands::gb::objects::get_object_shape3;
use crate::commands::gb::objects::get_object_shape4;
use crate::commands::gb::objects::get_object_shape5;
use crate::commands::gb::objects::get_object_shape6;
use crate::commands::gb::objects::get_object_shape7;
use crate::engine_v2::collision::Collision;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::movement::Movement;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::position::Position;
use crate::engine_v2::position::XTermPosition;
use crate::engine_v2::position::YTermPosition;

use crate::command::CommandV2;

pub struct Gb {}

impl CommandV2 for Gb {
    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef> {
        vec![get_object_gameboy]
    }

    fn select_objects(
        &mut self,
        _args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>) {
        let mut objects: Vec<ObjectRef> = Vec::new();
        let mut shape_objects: Vec<ObjectRef> = Vec::new();
        let mut collisions: Vec<Collision> = Vec::new();
        // background
        let gb_object = get_object_gameboy();
        let movement = Movement::new_stationary(
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0), 0),
            0,
        );
        gb_object.borrow_mut().set_movement(movement);

        // bottom
        let gb_bottom_object = get_object_bottom();
        let trajectory = Movement::new_stationary(
            Position::new(XTermPosition::Coord(10), YTermPosition::Coord(21), -10),
            0,
        );
        gb_bottom_object.borrow_mut().set_movement(trajectory);

        let speed = 100;
        for shape_type in 1..4 {
            //let shape_type: u8 = rng.random_range(1..=7);
            let object_shape = match shape_type {
                1 => get_object_shape1(),
                2 => get_object_shape2(),
                3 => get_object_shape3(),
                4 => get_object_shape4(),
                5 => get_object_shape5(),
                6 => get_object_shape6(),
                7 => get_object_shape7(),
                _ => unreachable!(),
            };
            let movement = Movement::new_linear(
                Position::new(XTermPosition::Coord(22), YTermPosition::Coord(32), 0),
                Position::new(XTermPosition::Coord(22), YTermPosition::Coord(22), 0),
                speed,
            );
            object_shape.borrow_mut().set_movement(movement);
            if !objects.is_empty() {
                object_shape.borrow_mut().set_visible(false);
                object_shape.borrow_mut().deactivate_movement();
            }
            // shape control
            Object::on_key(&object_shape, KeyCode::Char('d'), |s| {
                let mut object = s.borrow_mut();
                if object.is_movement_active() {
                    let x_offset = object.movement().offset().x();
                    if !object.movement().is_done() && x_offset < 11 {
                        object.movement().add_offset(Coords::new(2, 0, 0))
                    }
                }
            });
            Object::on_key(&object_shape, KeyCode::Right, |s| {
                let mut object = s.borrow_mut();
                if object.is_movement_active() {
                    let x_offset = object.movement().offset().x();
                    if !object.movement().is_done() && x_offset < 11 {
                        object.movement().add_offset(Coords::new(2, 0, 0))
                    }
                }
            });
            Object::on_key(&object_shape, KeyCode::Char('a'), |s| {
                let mut object = s.borrow_mut();
                if object.is_movement_active() {
                    let x_offset = object.movement().offset().x();
                    if !object.movement().is_done() && x_offset > -12 {
                        object.movement().add_offset(Coords::new(-2, 0, 0));
                    }
                }
            });
            Object::on_key(&object_shape, KeyCode::Left, |s| {
                let mut object = s.borrow_mut();
                if object.is_movement_active() {
                    let x_offset = object.movement().offset().x();
                    if !object.movement().is_done() && x_offset > -12 {
                        object.movement().add_offset(Coords::new(-2, 0, 0));
                    }
                }
            });
            objects.push(object_shape.clone());
            shape_objects.push(object_shape.clone());
        }

        for (index, shape_object) in shape_objects.iter().enumerate() {
            if index + 1 >= shape_objects.len() {
                break;
            }
            let next_object_shape_id = shape_objects[index + 1].borrow().id();

            let collision1 = Collision::new_object(
                shape_object.clone(),
                gb_bottom_object.clone(),
                move |moving_object, _, counter, engine| {
                    if counter == 100 {
                        for object in engine.objects_mut() {
                            if object.borrow().id() == next_object_shape_id {
                                object.borrow_mut().set_visible(true);
                                object.borrow_mut().activate_movement();
                                break;
                            }
                        }
                        moving_object.borrow_mut().deactivate_movement();
                    }
                },
            );
            collisions.push(collision1);
        }

        for (index, _) in shape_objects.iter().enumerate() {
            let shape_object_1 = shape_objects[index].clone();
            for n_index in (index + 1)..shape_objects.len() {
                let shape_object_2 = shape_objects[n_index].clone();
                let mut next_shape_object_2 = shape_objects[n_index].clone();
                if n_index < shape_objects.len() - 1 {
                    next_shape_object_2 = shape_objects[1 + n_index].clone();
                }
                let collision = Collision::new_object(
                    shape_object_1.clone(),
                    shape_object_2.clone(),
                    move |object_shape_1, shape_object_2, _counter, engine| {
                        let _shape_object_2_id = shape_object_2.borrow().id();
                        if !shape_object_2.borrow().visible() {
                            return;
                        }
                        if (object_shape_1.borrow().is_movement_active()
                            && !shape_object_2.borrow().is_movement_active())
                            || (!object_shape_1.borrow().is_movement_active()
                                && shape_object_2.borrow().is_movement_active())
                        {
                            // Trouver la liste des coordonnées occupées par l'objet
                            let occupied_coords = object_shape_1.borrow().get_occupied_coords(true);

                            // Trouver la liste des coordonnées occupées par l'objet suivant
                            let next_occupied_coords =
                                shape_object_2.borrow().get_occupied_coords(true);

                            // Trouver les coordonnées occupées par les deux objets
                            let overlapping_coords = occupied_coords
                                .iter()
                                .filter(|&&coord| next_occupied_coords.contains(&coord))
                                .cloned()
                                .collect::<Vec<Coords>>();

                            if !overlapping_coords.is_empty() {
                                if object_shape_1.borrow().is_movement_active() {
                                    object_shape_1.borrow_mut().deactivate_movement();
                                    let history = object_shape_1.borrow_mut().coords_history();
                                    object_shape_1
                                        .borrow_mut()
                                        .set_coords(history[history.len() - 1]);
                                }
                                if shape_object_2.borrow().is_movement_active() {
                                    shape_object_2.borrow_mut().deactivate_movement();
                                    let history = shape_object_2.borrow_mut().coords_history();
                                    shape_object_2
                                        .borrow_mut()
                                        .set_coords(history[history.len() - 1]);
                                }
                                // Start new shape
                                for object in engine.objects_mut() {
                                    if object.borrow().id() == next_shape_object_2.borrow().id() {
                                        object.borrow_mut().set_visible(true);
                                        object.borrow_mut().activate_movement();
                                        break;
                                    }
                                }
                            }
                        }
                    },
                );
                collisions.push(collision);
            }
        }

        objects.push(gb_object);
        objects.push(gb_bottom_object);
        //panic!("DDDD");
        (objects, collisions)
    }

    fn execute(&mut self) {
        let (objects, collisions) = self.select_objects(std::env::args());
        let mut engine = Engine::new(objects, collisions, 10000);
        engine.run();
    }
}
