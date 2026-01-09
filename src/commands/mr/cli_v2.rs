use std::cell::RefCell;
use std::rc::Rc;

use crate::command::CommandV2;
use crate::commands::mr::objects::get_object_explosion;
use crate::commands::mr::objects::get_object_mini;
use crate::commands::mr::objects::get_object_sign_fail;
use crate::commands::mr::objects::get_object_sign_land;
use crate::commands::mr::objects::get_object_sign_success;
use crate::commands::mr::objects::get_object_sign_tryagain;
use crate::commands::mr::objects::get_object_spaceport;
use crate::commands::mr::objects::get_object_std;
use crate::engine_v2::collision::Collision;
use crate::engine_v2::collision::ScreenEdge;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::movement::Movement;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::position::Position;
use crate::engine_v2::position::XTermPosition;
use crate::engine_v2::position::YTermPosition;
use crate::tools::get_terminal_size;
use crate::tools::parse_args;
use crossterm::event::KeyCode;
use rand::Rng;

pub struct Mr {
    pub landed: Rc<RefCell<bool>>,
    pub retry: Rc<RefCell<bool>>,
}

impl CommandV2 for Mr {
    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef> {
        vec![get_object_std, get_object_mini, get_object_spaceport]
    }
    fn select_objects(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>) {
        let mut rng = rand::rng();
        let term_size = get_terminal_size();
        let landed_clone = self.landed.clone();
        let retry_clone = self.retry.clone();
        let mut object_list: Vec<ObjectRef> = Vec::new();

        let short_flags = ['f', 'r'];
        let long_flags = ["force", "recursive"];
        let params: &[&str] = &[];
        let (flags, _) = parse_args(args.collect(), &short_flags, &long_flags, params);
        let mut is_recursive = false;
        if flags.contains("r") || flags.contains("recursive") {
            is_recursive = true;
            *retry_clone.borrow_mut() = true;
        }

        // Signs
        let end_sign_position = Movement::new_stationary(
            Position::new(XTermPosition::Middle, YTermPosition::Middle, 0),
            20,
        );
        // Land sign
        let land_sign_object = get_object_sign_land();
        let sign_position = Movement::new_stationary(
            Position::new(XTermPosition::LeftIn, YTermPosition::TopIn, 0),
            0,
        );
        land_sign_object
            .borrow_mut()
            .set_movement(sign_position.clone());
        object_list.push(land_sign_object.clone());
        // Success sign
        let success_sign_object = get_object_sign_success();
        success_sign_object
            .borrow_mut()
            .set_movement(end_sign_position.clone());
        success_sign_object.borrow_mut().set_visible(false);
        object_list.push(success_sign_object.clone());
        // Failed sign
        let failed_sign_object = get_object_sign_fail();
        failed_sign_object.borrow_mut().set_visible(false);
        failed_sign_object
            .borrow_mut()
            .set_movement(end_sign_position.clone());
        object_list.push(failed_sign_object.clone());
        // Try again sign
        let tryagain_sign_object = get_object_sign_tryagain();
        tryagain_sign_object.borrow_mut().set_visible(false);
        tryagain_sign_object
            .borrow_mut()
            .set_movement(end_sign_position.clone());
        object_list.push(tryagain_sign_object.clone());

        // Rocket
        let mut rocket_object: ObjectRef = get_object_mini();
        if flags.contains("f") || flags.contains("force") {
            rocket_object = get_object_std();
        }

        // Rocket control
        Object::on_key(&rocket_object, KeyCode::Char('d'), move |o| {
            o.borrow_mut().movement().add_offset(Coords::new(1, 0, 0))
        });
        Object::on_key(&rocket_object, KeyCode::Right, move |o| {
            o.borrow_mut().movement().add_offset(Coords::new(1, 0, 0))
        });
        Object::on_key(&rocket_object, KeyCode::Char('a'), move |o| {
            o.borrow_mut().movement().add_offset(Coords::new(-1, 0, 0));
        });
        Object::on_key(&rocket_object, KeyCode::Left, move |o| {
            o.borrow_mut().movement().add_offset(Coords::new(-1, 0, 0))
        });
        let movement = Movement::new_linear(
            Position::new(XTermPosition::Middle, YTermPosition::TopOut, 0),
            Position::new(XTermPosition::Middle, YTermPosition::Coord(-2), 0),
            20,
        );
        rocket_object.borrow_mut().set_movement(movement);
        object_list.push(rocket_object.clone());
        // Space port
        let spaceport_object = get_object_spaceport();
        let spaceport_x_position: i32 = rng.random_range(
            0..(term_size.width() as i32)
                - spaceport_object.borrow().current_frame().get_width() as i32,
        );
        let spaceport_position = Movement::new_stationary(
            Position::new(
                XTermPosition::Coord(spaceport_x_position),
                YTermPosition::Coord(0),
                0,
            ),
            300,
        );
        spaceport_object
            .borrow_mut()
            .set_movement(spaceport_position);
        object_list.push(spaceport_object.clone());

        // Explosion
        let explosion_object = get_object_explosion();
        explosion_object.borrow_mut().set_visible(false);
        let movement = Movement::new_stationary(
            Position::new(XTermPosition::LeftIn, YTermPosition::TopIn, 0),
            0,
        );
        explosion_object.borrow_mut().set_movement(movement);
        object_list.push(explosion_object.clone());

        // Collisions
        let mut collision_list: Vec<Collision> = Vec::new();

        // Rocket collision with spaceport
        let collision = Collision::new_sprite(
            rocket_object.clone(),
            spaceport_object.clone(),
            move |rocket_object, _, counter, engine| {
                let rocket_coord = rocket_object.borrow().coords();
                let rocket_landed_traj = Movement::new_stationary(
                    Position::new(
                        XTermPosition::Coord(rocket_coord.x()),
                        YTermPosition::Coord(rocket_coord.y()),
                        0,
                    ),
                    20,
                );
                rocket_object.borrow_mut().set_movement(rocket_landed_traj);
                rocket_object.borrow_mut().set_frame_id(0);
                rocket_object
                    .borrow_mut()
                    .compute_predefined_path(engine.terminal_size());
                success_sign_object.borrow_mut().set_visible(true);
                *landed_clone.borrow_mut() = true;
                if counter >= 200 {
                    engine.stop();
                }
            },
        );
        collision_list.push(collision);

        // Rocket Collision with bottom of the screen
        let collision_bottom = Collision::new_edge(
            rocket_object.clone(),
            ScreenEdge::BottomWithObjectBottomSide,
            move |rocket_object, counter, engine| {
                if rocket_object.borrow().visible() {
                    rocket_object.borrow_mut().set_visible(false);
                    let rocket_coord = rocket_object.borrow_mut().coords();
                    let explosion_position = Movement::new_stationary(
                        Position::new(
                            XTermPosition::Coord(
                                rocket_coord.x()
                                    - (explosion_object.borrow().current_frame().get_width() / 2)
                                        as i32
                                    + (rocket_object.borrow().current_frame().get_width() / 2)
                                        as i32,
                            ),
                            YTermPosition::Coord(0),
                            0,
                        ),
                        0,
                    );
                    explosion_object
                        .borrow_mut()
                        .set_movement(explosion_position);
                    explosion_object
                        .borrow_mut()
                        .compute_predefined_path(engine.terminal_size());
                    explosion_object.borrow_mut().set_visible(true);
                    explosion_object
                        .borrow_mut()
                        .reset_animation(engine.tick_id());

                    if is_recursive {
                        tryagain_sign_object.borrow_mut().set_visible(true);
                    } else {
                        failed_sign_object.borrow_mut().set_visible(true);
                    }
                } else if counter == 500 {
                    engine.stop();
                }
            },
        );
        collision_list.push(collision_bottom);
        (object_list, collision_list)
    }

    fn execute(&mut self) {
        loop {
            let (objects, collisions) = self.select_objects(std::env::args());
            let mut engine = Engine::new(objects, collisions, 0);
            engine.run();

            if engine.is_killed() {
                // SIG KILL
                break;
            }
            if !*self.retry.borrow() {
                // recursive
                break;
            }
            if *self.landed.borrow() {
                // Landed
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::mr::cli_v2::Mr;

    #[test]
    fn test_select_sprite_std() {
        let mut mr = Mr {
            landed: Rc::new(RefCell::new(false)),
            retry: Rc::new(RefCell::new(false)),
        };
        let args: Vec<String> = vec![String::from("mr")];
        let (objects, collisions) = mr.select_objects(args.into_iter());

        assert_eq!(objects.len(), 7);
        assert_eq!(objects[0].borrow_mut().movement().speed(), 0);
        assert_eq!(objects[4].borrow_mut().tdid(), 14);
        assert_eq!(objects[5].borrow_mut().tdid(), 18);
        assert_eq!(collisions.len(), 2);
    }

    #[test]
    fn test_select_sprite_force() {
        let mut mr = Mr {
            landed: Rc::new(RefCell::new(false)),
            retry: Rc::new(RefCell::new(false)),
        };
        let args: Vec<String> = vec![String::from("mr"), String::from("-f")];
        let (objects, collisions) = mr.select_objects(args.into_iter());

        assert_eq!(objects.len(), 7);
        assert_eq!(objects[0].borrow_mut().movement().speed(), 0);
        assert_eq!(objects[4].borrow_mut().tdid(), 16);
        assert_eq!(objects[5].borrow_mut().tdid(), 18);
        assert_eq!(collisions.len(), 2);
    }

    #[test]
    fn test_select_sprite_recursive() {
        let mut mr = Mr {
            landed: Rc::new(RefCell::new(false)),
            retry: Rc::new(RefCell::new(true)),
        };
        let args: Vec<String> = vec![String::from("mr"), String::from("-r")];
        let (objects, collisions) = mr.select_objects(args.into_iter());

        assert_eq!(objects.len(), 7);
        assert_eq!(objects[0].borrow_mut().movement().speed(), 0);
        assert_eq!(objects[3].borrow_mut().tdid(), 23);
        assert_eq!(collisions.len(), 2);
    }
}
