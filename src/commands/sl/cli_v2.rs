use std::cmp;

use crate::commands::sl::objects::get_object_accident;
use crate::commands::sl::objects::get_object_c51;
use crate::commands::sl::objects::get_object_coal;
use crate::commands::sl::objects::get_object_d51;
use crate::commands::sl::objects::get_object_logo;
use crate::commands::sl::objects::get_object_logo_car;
use crate::commands::sl::objects::get_object_logo_coal;
use crate::commands::sl::objects::get_object_smoke;
use crate::engine_v2::collision::Collision;
use crate::engine_v2::collision::ScreenEdge;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::movement::Movement;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::position::Position;
use crate::engine_v2::position::XTermPosition;
use crate::engine_v2::position::YTermPosition;
use crate::tools::parse_args;

use crate::command::CommandV2;

pub struct Sl {}

impl CommandV2 for Sl {
    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef> {
        vec![get_object_d51]
    }

    fn select_objects(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>) {
        let mut object_list: Vec<ObjectRef> = Vec::new();
        // No collision
        let mut collision_list: Vec<Collision> = Vec::new();
        // Handle CLI flags
        let short_flags = ['a', 'F', 'l', 'c'];
        let long_flags: &[&str] = &[];
        let params: &[&str] = &[];
        let (flags, _) = parse_args(args.collect(), &short_flags, long_flags, params);
        let locomotive_speed = 7;

        // get objects
        let smoke_object = get_object_smoke();
        let mut coal_object = get_object_coal();
        let mut locomotive_object = get_object_d51();
        if flags.contains("l") {
            locomotive_object = get_object_logo();
            coal_object = get_object_logo_coal();
        }
        if flags.contains("c") {
            locomotive_object = get_object_c51();
        }

        let locomotive_height = locomotive_object.borrow().current_frame().get_height();
        let locomotive_width = locomotive_object.borrow().current_frame().get_width();

        // Handle Coal
        let coal_x_offset = locomotive_width;

        let mut coal_coord = Coords::new(coal_x_offset as i32, 0, 1);
        if flags.contains("c") {
            coal_coord = Coords::new(coal_x_offset as i32, 0, 1);
            if flags.contains("F") {
                coal_coord = Coords::new(coal_x_offset as i32, -1, 1);
            }
        } else if flags.contains("l") {
            coal_coord = Coords::new(coal_x_offset as i32 + 1, 0, 1);
            if flags.contains("F") {
                coal_coord = Coords::new(coal_x_offset as i32 + 1, -2, 1);
            }
        } else if flags.contains("F") {
            coal_coord = Coords::new(coal_x_offset as i32 + 1, -1, 1);
        }
        let coal_movement = Movement::new_relative(locomotive_object.clone(), coal_coord);
        coal_object.borrow_mut().set_movement(coal_movement);
        object_list.push(coal_object.clone());

        // Handle smoke
        let mut smoke_x_offset: u32 = 8;
        if flags.contains("l") {
            smoke_x_offset = 5;
        }
        let smoke_coord = Coords::new(smoke_x_offset as i32, locomotive_height as i32, 0);
        let smoke_movement = Movement::new_relative(locomotive_object.clone(), smoke_coord);
        smoke_object.borrow_mut().set_movement(smoke_movement);
        object_list.push(smoke_object.clone());

        // Locomotive
        let coal_width = coal_object.borrow().current_frame().get_width();
        let smoke_width = smoke_object.borrow().current_frame().get_width();
        let mut locomotive_end_position_x =
            cmp::max(smoke_x_offset + smoke_width, coal_x_offset + coal_width);

        // Handle logo cars
        let mut end_object = coal_object.clone();
        if flags.contains("l") {
            let car1_object = get_object_logo_car();
            let car1_width = car1_object.borrow().current_frame().get_width();
            let mut y_offset = 0;
            if flags.contains("F") {
                y_offset = -2
            }
            let car1_coord = Coords::new(coal_width as i32 + 1, y_offset, 0);
            let car1_movement = Movement::new_relative(coal_object.clone(), car1_coord);
            car1_object.borrow_mut().set_movement(car1_movement);
            let car2_object = get_object_logo_car();
            let car2_coord = Coords::new(car1_width as i32 + 1, y_offset, 0);
            let car2_movement = Movement::new_relative(car1_object.clone(), car2_coord);
            end_object = car2_object.clone();
            car2_object.borrow_mut().set_movement(car2_movement);

            locomotive_end_position_x = cmp::max(
                smoke_x_offset + smoke_width + car1_width * 2,
                coal_x_offset + coal_width,
            );

            if flags.contains("a") {
                // Handle accident on locomotive
                let accident_loco_coords = Coords::new(13, 3, 20);
                let accident_object_loco = get_object_accident(0);
                let accident_movement_loco =
                    Movement::new_relative(locomotive_object.clone(), accident_loco_coords);
                accident_object_loco
                    .borrow_mut()
                    .set_movement(accident_movement_loco);
                object_list.push(accident_object_loco);

                // Handle accident on cars
                let parents = [car1_object.clone(), car2_object.clone()];
                for parent in parents.iter() {
                    let accident_coords = [Coords::new(10, 3, 20), Coords::new(2, 3, 20)];
                    for (index, accident_coord) in accident_coords.iter().enumerate() {
                        let accident_carx_1_coords = accident_coord;
                        let accident_object_car1_1 = get_object_accident(index);
                        let accident_movement_cart1_1 =
                            Movement::new_relative(parent.clone(), *accident_carx_1_coords);
                        accident_object_car1_1
                            .borrow_mut()
                            .set_movement(accident_movement_cart1_1);
                        object_list.push(accident_object_car1_1);
                    }
                }
            }
            object_list.push(car1_object);
            object_list.push(car2_object);
        }
        // Complete locomotive
        let locomotive_end_position = Position::new(
            XTermPosition::Coord(0 - locomotive_end_position_x as i32),
            YTermPosition::Middle,
            0,
        );
        let mut locomotive_start_position =
            Position::new(XTermPosition::RightOut, YTermPosition::Middle, 0);
        if flags.contains("F") {
            locomotive_start_position =
                Position::new(XTermPosition::RightOut, YTermPosition::Coord(0), 0);
        }
        let locomotive_movement = Movement::new_linear(
            locomotive_start_position,
            locomotive_end_position,
            locomotive_speed,
        );
        locomotive_object
            .borrow_mut()
            .set_movement(locomotive_movement);
        object_list.push(locomotive_object.clone());

        // Handle accident
        if flags.contains("a") && !flags.contains("l") {
            let mut accidents_coords = vec![Coords::new(46, 6, 20), Coords::new(42, 6, 20)];
            if flags.contains("c") {
                accidents_coords = vec![Coords::new(48, 6, 20), Coords::new(44, 6, 20)];
            }
            for (index, accident_coord) in accidents_coords.iter().enumerate() {
                let accident_object = get_object_accident(index);
                let accident_movement =
                    Movement::new_relative(locomotive_object.clone(), *accident_coord);
                accident_object.borrow_mut().set_movement(accident_movement);
                object_list.push(accident_object);
            }
        }

        let collision = Collision::new_edge(
            end_object.clone(),
            ScreenEdge::LeftWithObjectRightSide,
            move |_, _, engine| {
                engine.stop();
            },
        );
        collision_list.push(collision);

        (object_list, collision_list)
    }
    fn execute(&mut self) {
        // Start rendering
        let (objects, collisions) = self.select_objects(std::env::args());
        let mut engine = Engine::new(objects, collisions, 0);
        engine.run();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::sl::cli_v2::Sl;

    #[test]
    fn test_select_base() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 3);
        assert_eq!(objects[2].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[2].borrow_mut().tdid(), 1);
        assert_eq!(collisions.len(), 1);
    }
    #[test]
    fn test_select_object_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-a")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 5);
        assert_eq!(objects[2].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[2].borrow_mut().tdid(), 1);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_object_little_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-al")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 10);
        assert_eq!(objects[9].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[9].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_object_fly_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-aF")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 5);
        assert_eq!(objects[2].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[2].borrow_mut().tdid(), 1);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_object_little_fly_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-aFl")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 10);
        assert_eq!(objects[9].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[9].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_object_c_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-ac")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 5);
        assert_eq!(objects[2].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_object_c_fly_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-aFc")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 5);
        assert_eq!(objects[2].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_object_little() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-l")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 5);
        assert_eq!(objects[4].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[4].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 1);
    }
    #[test]
    fn test_select_object_c() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-c")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 3);
        assert_eq!(objects[2].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_object_little_fly() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-lF")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 5);
        assert_eq!(objects[4].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[4].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 1);
    }
    #[test]
    fn test_select_object_c_fly() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-cF")];
        let (objects, collisions) = sl.select_objects(args.into_iter());

        assert_eq!(objects.len(), 3);
        assert_eq!(objects[2].borrow_mut().movement().speed(), 7);
        assert_eq!(objects[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 1);
    }
}
