use std::cell::RefCell;
use std::rc::Rc;

use crate::collision::Collision;
use crate::collision::ScreenEdge;
use crate::commands::mr::sprite::{
    get_sprite_explosion, get_sprite_mini, get_sprite_mini_landed, get_sprite_sign_fail,
    get_sprite_sign_land, get_sprite_sign_success, get_sprite_sign_tryagain, get_sprite_spaceport,
    get_sprite_std, get_sprite_std_landed,
};
use crate::coord::{Coord, Position, XTermPosition, YTermPosition};
use crate::engine::{RenderEngine, get_terminal_size};
use crate::sprite::Sprite;
use crate::sprite::SpriteRef;
use crate::tools::parse_args;
use crate::trajectory::Trajectory;
use crossterm::event::KeyCode;
use rand::Rng;

use crate::command::Command;

pub struct Mr {
    pub landed: Rc<RefCell<bool>>,
}

impl Command for Mr {
    fn get_all_sprites(&self) -> Vec<fn() -> SpriteRef> {
        vec![get_sprite_std, get_sprite_mini, get_sprite_spaceport]
    }
    fn select_sprites(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<SpriteRef>, Vec<Collision>) {
        let mut rng = rand::rng();
        let (term_width, _) = get_terminal_size();
        let landed_clone = self.landed.clone();
        let do_not_retry_clone = self.landed.clone();
        let mut sprite_list: Vec<SpriteRef> = Vec::new();

        let short_flags = ['f', 'r'];
        let long_flags = ["force", "recursive"];
        let params: &[&str] = &[];
        let (flags, _) = parse_args(args.collect(), &short_flags, &long_flags, params);
        let mut is_recursive = false;
        if flags.contains("r") || flags.contains("recursive") {
            is_recursive = true;
        }

        // Signs
        let end_sign_position = Trajectory::new_stationary(
            Position::new(XTermPosition::Middle, YTermPosition::Middle),
            20,
        );
        // Land sign
        let land_sign_sprite = get_sprite_sign_land();
        let sign_position = Trajectory::new_stationary(
            Position::new(XTermPosition::LeftIn, YTermPosition::TopIn),
            0,
        );
        land_sign_sprite
            .borrow_mut()
            .set_movement(sign_position.clone());
        sprite_list.push(land_sign_sprite.clone());
        // Success sign
        let success_sign_sprite = get_sprite_sign_success();
        success_sign_sprite
            .borrow_mut()
            .set_movement(end_sign_position.clone());
        success_sign_sprite.borrow_mut().set_visible(false);
        sprite_list.push(success_sign_sprite.clone());
        // Failed sign
        let failed_sign_sprite = get_sprite_sign_fail();
        failed_sign_sprite.borrow_mut().set_visible(false);
        failed_sign_sprite
            .borrow_mut()
            .set_movement(end_sign_position.clone());
        sprite_list.push(failed_sign_sprite.clone());
        // Try again sign
        let tryagain_sign_sprite = get_sprite_sign_tryagain();
        tryagain_sign_sprite.borrow_mut().set_visible(false);
        tryagain_sign_sprite
            .borrow_mut()
            .set_movement(end_sign_position.clone());
        sprite_list.push(tryagain_sign_sprite.clone());

        // Landed Rocket
        let mut landed_rocket_sprite = get_sprite_mini_landed();
        if flags.contains("f") || flags.contains("force") {
            landed_rocket_sprite = get_sprite_std_landed();
        }
        landed_rocket_sprite.borrow_mut().set_visible(false);
        sprite_list.push(landed_rocket_sprite.clone());

        // Rocket
        let mut rocket_sprite: SpriteRef = get_sprite_mini();
        if flags.contains("f") || flags.contains("force") {
            rocket_sprite = get_sprite_std();
        }

        // Rocket control
        Sprite::on_key(&rocket_sprite, KeyCode::Char('d'), |s| {
            s.borrow_mut().movement().add_offset(Coord::new(1, 0))
        });
        Sprite::on_key(&rocket_sprite, KeyCode::Right, |s| {
            s.borrow_mut().movement().add_offset(Coord::new(1, 0))
        });
        Sprite::on_key(&rocket_sprite, KeyCode::Char('a'), |s| {
            s.borrow_mut().movement().add_offset(Coord::new(-1, 0));
        });
        Sprite::on_key(&rocket_sprite, KeyCode::Left, |s| {
            s.borrow_mut().movement().add_offset(Coord::new(-1, 0))
        });
        let movement = Trajectory::new_linear(
            Position::new(XTermPosition::Middle, YTermPosition::TopOut),
            Position::new(XTermPosition::Middle, YTermPosition::Coord(-2)),
            20,
        );
        rocket_sprite.borrow_mut().set_movement(movement);
        sprite_list.push(rocket_sprite.clone());
        // Space port
        let spaceport_sprite = get_sprite_spaceport();
        let spaceport_x_position: i32 = rng.random_range(
            0..(term_width as i32) - spaceport_sprite.borrow().current_frame().get_width() as i32,
        );
        let spaceport_position = Trajectory::new_stationary(
            Position::new(
                XTermPosition::Coord(spaceport_x_position),
                YTermPosition::Coord(0),
            ),
            300,
        );
        spaceport_sprite
            .borrow_mut()
            .set_movement(spaceport_position);
        sprite_list.push(spaceport_sprite.clone());

        // Explosion
        let sprite_explosion = get_sprite_explosion();
        sprite_explosion.borrow_mut().set_visible(false);
        sprite_list.push(sprite_explosion.clone());

        // Collisions
        let mut collision_list: Vec<Collision> = Vec::new();

        // Collision with spaceport
        let collision = Collision::new_sprite(
            rocket_sprite.clone(),
            spaceport_sprite.clone(),
            move |rocket_sprite, _, _, _| {
                rocket_sprite.borrow_mut().set_visible(false);
                let rocket_coord = rocket_sprite.borrow().current_coordinate();
                let rocket_landed_traj = Trajectory::new_stationary(
                    Position::new(
                        XTermPosition::Coord(rocket_coord.x()),
                        YTermPosition::Coord(rocket_coord.y() + 1),
                    ),
                    20,
                );
                landed_rocket_sprite
                    .borrow_mut()
                    .set_movement(rocket_landed_traj);
                landed_rocket_sprite.borrow_mut().set_visible(true);
                success_sign_sprite.borrow_mut().set_visible(true);
                *landed_clone.borrow_mut() = true;
            },
        );
        collision_list.push(collision);

        // Collision with bottom of the screen
        let collision_bottom = Collision::new_edge(
            rocket_sprite.clone(),
            ScreenEdge::Bottom,
            move |rocket_sprite, _, _| {
                rocket_sprite.borrow_mut().set_visible(false);
                let rocket_coord = rocket_sprite.borrow_mut().movement().current_coordinate();
                let explosion_position = Trajectory::new_stationary(
                    Position::new(
                        XTermPosition::Coord(
                            rocket_coord.x()
                                - (sprite_explosion.borrow().current_frame().get_width() / 2)
                                    as i32
                                + (rocket_sprite.borrow().current_frame().get_width() / 2) as i32,
                        ),
                        YTermPosition::Coord(0),
                    ),
                    0,
                );
                sprite_explosion
                    .borrow_mut()
                    .set_movement(explosion_position);
                sprite_explosion.borrow_mut().set_visible(true);
                if is_recursive {
                    tryagain_sign_sprite.borrow_mut().set_visible(true);
                } else {
                    failed_sign_sprite.borrow_mut().set_visible(true);
                    *do_not_retry_clone.borrow_mut() = true;
                }
            },
        );
        collision_list.push(collision_bottom);
        (sprite_list, collision_list)
    }

    fn execute(&mut self) {
        loop {
            let mut engine = RenderEngine::new(0);
            let (mut sprites, mut collisions) = self.select_sprites(std::env::args());
            engine.render(&mut sprites, &mut collisions);

            if engine.killed() {
                break;
            }
            if *self.landed.borrow_mut() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::mr::cli::Mr;

    #[test]
    fn test_select_sprite_std() {
        let mut mr = Mr {
            landed: Rc::new(RefCell::new(false)),
        };
        let args: Vec<String> = vec![String::from("mr")];
        let (sprites, collisions) = mr.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 8);
        assert_eq!(sprites[0].borrow_mut().movement().speed(), 0);
        assert_eq!(sprites[4].borrow_mut().tdid(), 15);
        assert_eq!(sprites[5].borrow_mut().tdid(), 14);
        assert_eq!(collisions.len(), 2);
    }

    #[test]
    fn test_select_sprite_force() {
        let mut mr = Mr {
            landed: Rc::new(RefCell::new(false)),
        };
        let args: Vec<String> = vec![String::from("mr"), String::from("-f")];
        let (sprites, collisions) = mr.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 8);
        assert_eq!(sprites[0].borrow_mut().movement().speed(), 0);
        assert_eq!(sprites[4].borrow_mut().tdid(), 17);
        assert_eq!(sprites[5].borrow_mut().tdid(), 16);
        assert_eq!(collisions.len(), 2);
    }

    #[test]
    fn test_select_sprite_recursive() {
        let mut mr = Mr {
            landed: Rc::new(RefCell::new(false)),
        };
        let args: Vec<String> = vec![String::from("mr"), String::from("-r")];
        let (sprites, collisions) = mr.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 8);
        assert_eq!(sprites[0].borrow_mut().movement().speed(), 0);
        assert_eq!(sprites[3].borrow_mut().tdid(), 23);
        assert_eq!(collisions.len(), 2);
    }
}
