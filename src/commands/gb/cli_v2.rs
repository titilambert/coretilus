use std::vec;

use crate::commands::gb::objects::get_object_gameboy;
use crate::commands::gb::objects::get_object_shape;
use crate::engine_v2::collision::Collision;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::movement::Movement;
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
        let mut collisions: Vec<Collision> = Vec::new();
        let speed = 20;
        // background
        let gb_object = get_object_gameboy();
        let movement = Movement::new_stationary(
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0), 0),
            0,
        );
        gb_object.borrow_mut().set_movement(movement);

        type ShapeSpec = (fn(usize, usize) -> ObjectRef, i32, i32, i32, i32, i32);
        let raw_shapes: [ShapeSpec; 8] = [
            (get_object_shape, 7, 17, 22, speed, speed / 2),
            (get_object_shape, 3, 16, 24, speed, speed / 2),
            (get_object_shape, 7, 20, 22, speed, speed / 2),
            (get_object_shape, 2, 22, 22, speed, speed / 2),
            (get_object_shape, 2, 25, 22, speed, speed / 2),
            (get_object_shape, 1, 27, 22, speed, speed / 2),
            (get_object_shape, 1, 30, 22, speed, speed / 2),
            (get_object_shape, 8, 30, 23, speed, speed / 2),
        ];
        for (index, (object_fun, id, x, y, speed, sprite_speed)) in raw_shapes.iter().enumerate() {
            let object_shape = object_fun(
                (*id).try_into().unwrap(),
                (*sprite_speed).try_into().unwrap(),
            )
            .clone();
            if index == 0 {
                object_shape.borrow_mut().activate_sprite(0);
            }
            let movement = Movement::new_linear(
                Position::new(XTermPosition::Coord(*x), YTermPosition::Coord(32), 0),
                Position::new(XTermPosition::Coord(*x), YTermPosition::Coord(*y), 0),
                *speed,
            );
            object_shape.borrow_mut().set_movement(movement);

            if !objects.is_empty() {
                object_shape.borrow_mut().set_visible(false);
                object_shape.borrow_mut().deactivate_movement();
            }
            objects.push(object_shape);
        }

        for (index, shape_object) in objects.iter().enumerate() {
            if index + 1 >= objects.len() {
                break;
            }
            let next_object_shape_id = objects[index + 1].borrow().id();

            let (_, _, x, y, speed, _) = raw_shapes[index];
            let collision = Collision::new_point(
                shape_object.clone(),
                Coords::new(x, y, 0),
                move |shape_object2, _, counter, engine| {
                    let tick_id = engine.tick_id();
                    if counter == speed as usize - 1 {
                        shape_object2.borrow_mut().deactivate_sprite();
                        shape_object2.borrow_mut().deactivate_movement();
                    }
                    if counter == speed as usize + 1 {
                        for object in engine.objects_mut() {
                            if object.borrow().id() == next_object_shape_id {
                                object.borrow_mut().set_visible(true);
                                object.borrow_mut().activate_movement();
                                object.borrow_mut().activate_sprite(tick_id);
                            }
                        }
                    }
                },
            );

            collisions.push(collision);
        }
        let last_shape_index = raw_shapes.len() - 1;
        let (_, _, x, y, _, _) = raw_shapes[last_shape_index];
        let last_shape_object = objects.get(last_shape_index).unwrap().clone();
        let collision = Collision::new_point(
            last_shape_object,
            Coords::new(x, y, 0),
            move |_, _, counter, engine| {
                if counter == 500 {
                    engine.stop();
                }
            },
        );
        collisions.push(collision);

        objects.push(gb_object);
        //panic!("DDDD");
        (objects, collisions)
    }

    fn execute(&mut self) {
        let (objects, collisions) = self.select_objects(std::env::args());
        let mut engine = Engine::new(objects, collisions, 10000);
        engine.run();
    }
}
