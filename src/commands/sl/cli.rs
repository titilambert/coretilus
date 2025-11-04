use std::cmp;

use crate::collision::Collision;
use crate::commands::sl::sprite::{
    get_sprite_accident, get_sprite_c51, get_sprite_coal, get_sprite_d51, get_sprite_logo,
    get_sprite_logo_car, get_sprite_logo_coal, get_sprite_smoke,
};
use crate::coord::{Coord, Position, XTermPosition, YTermPosition};
use crate::engine::RenderEngine;
use crate::sprite::SpriteRef;
use crate::tools::parse_args;
use crate::trajectory::Trajectory;

use crate::command::Command;

pub struct Sl {}

impl Command for Sl {
    fn get_all_sprites(&self) -> Vec<fn() -> SpriteRef> {
        vec![get_sprite_d51]
    }

    fn select_sprites(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<SpriteRef>, Vec<Collision>) {
        let mut sprite_list: Vec<SpriteRef> = Vec::new();
        // No collision
        let collision_list: Vec<Collision> = Vec::new();
        // Handle CLI flags
        let short_flags = ['a', 'F', 'l', 'c'];
        let long_flags: &[&str] = &[];
        let params: &[&str] = &[];
        let (flags, _) = parse_args(args.collect(), &short_flags, long_flags, params);

        // get sprites
        let smoke_sprite = get_sprite_smoke();
        let mut coal_sprite = get_sprite_coal();
        let mut locomotive_sprite = get_sprite_d51();
        if flags.contains("l") {
            locomotive_sprite = get_sprite_logo();
            coal_sprite = get_sprite_logo_coal();
        }
        if flags.contains("c") {
            locomotive_sprite = get_sprite_c51();
        }

        let locomotive_height = locomotive_sprite.borrow().current_frame().get_height();
        let locomotive_width = locomotive_sprite.borrow().current_frame().get_width();

        // Handle Coal
        let coal_x_offset = locomotive_width;

        let mut coal_coord = Coord::new(coal_x_offset as i32, 0);
        if flags.contains("c") {
            coal_coord = Coord::new(coal_x_offset as i32, 0);
            if flags.contains("F") {
                coal_coord = Coord::new(coal_x_offset as i32, -1);
            }
        } else if flags.contains("l") {
            coal_coord = Coord::new(coal_x_offset as i32 + 1, 0);
            if flags.contains("F") {
                coal_coord = Coord::new(coal_x_offset as i32 + 1, -2);
            }
        } else if flags.contains("F") {
            coal_coord = Coord::new(coal_x_offset as i32 + 1, -1);
        }
        let coal_movement = Trajectory::new_relative(locomotive_sprite.clone(), coal_coord);
        coal_sprite.borrow_mut().set_movement(coal_movement);
        sprite_list.push(coal_sprite.clone());

        // Handle smoke
        let mut smoke_x_offset: u32 = 8;
        if flags.contains("l") {
            smoke_x_offset = 5;
        }
        let smoke_coord = Coord::new(smoke_x_offset as i32, locomotive_height as i32);
        let smoke_movement = Trajectory::new_relative(locomotive_sprite.clone(), smoke_coord);
        smoke_sprite.borrow_mut().set_movement(smoke_movement);
        sprite_list.push(smoke_sprite.clone());

        // Locomotive
        let coal_width = coal_sprite.borrow().current_frame().get_width();
        let smoke_width = smoke_sprite.borrow().current_frame().get_width();
        let end_position_x = cmp::max(smoke_x_offset + smoke_width, coal_x_offset + coal_width);
        let end_position = Position::new(
            XTermPosition::Coord(0 - end_position_x as i32),
            YTermPosition::Middle,
        );
        let mut start_position = Position::new(XTermPosition::RightOut, YTermPosition::Middle);
        if flags.contains("F") {
            start_position = Position::new(XTermPosition::RightOut, YTermPosition::Coord(0));
        }
        let movement = Trajectory::new_linear(start_position, end_position, 7);
        locomotive_sprite.borrow_mut().set_movement(movement);
        sprite_list.push(locomotive_sprite.clone());
        // Handle logo cars
        if flags.contains("l") {
            let car1_sprite = get_sprite_logo_car();
            let car1_width = car1_sprite.borrow().current_frame().get_width();
            let mut y_offset = 0;
            if flags.contains("F") {
                y_offset = -2
            }
            let car1_coord = Coord::new(coal_width as i32 + 1, y_offset);
            let car1_movement = Trajectory::new_relative(coal_sprite.clone(), car1_coord);
            car1_sprite.borrow_mut().set_movement(car1_movement);
            let car2_sprite = get_sprite_logo_car();
            let car2_coord = Coord::new(car1_width as i32 + 1, y_offset);
            let car2_movement = Trajectory::new_relative(car1_sprite.clone(), car2_coord);
            car2_sprite.borrow_mut().set_movement(car2_movement);
            if flags.contains("a") {
                // Handle accident on locomotive
                let accident_loco_coords = Coord::new(13, 3);
                let accident_sprite_loco = get_sprite_accident(0);
                let accident_movement_loco =
                    Trajectory::new_relative(locomotive_sprite.clone(), accident_loco_coords);
                accident_sprite_loco
                    .borrow_mut()
                    .set_movement(accident_movement_loco);
                sprite_list.push(accident_sprite_loco);

                // Handle accident on cars
                let parents = [car1_sprite.clone(), car2_sprite.clone()];
                for parent in parents.iter() {
                    let accident_coords = [Coord::new(10, 3), Coord::new(2, 3)];
                    for (index, accident_coord) in accident_coords.iter().enumerate() {
                        let accident_carx_1_coords = accident_coord;
                        let accident_sprite_car1_1 = get_sprite_accident(index);
                        let accident_movement_cart1_1 =
                            Trajectory::new_relative(parent.clone(), *accident_carx_1_coords);
                        accident_sprite_car1_1
                            .borrow_mut()
                            .set_movement(accident_movement_cart1_1);
                        sprite_list.push(accident_sprite_car1_1);
                    }
                }
            }
            sprite_list.push(car1_sprite);
            sprite_list.push(car2_sprite);
        }

        // Handle accident
        if flags.contains("a") && !flags.contains("l") {
            let mut accidents_coords = vec![Coord::new(46, 6), Coord::new(42, 6)];
            if flags.contains("c") {
                accidents_coords = vec![Coord::new(48, 6), Coord::new(44, 6)];
            }
            for (index, accident_coord) in accidents_coords.iter().enumerate() {
                let accident_sprite = get_sprite_accident(index);
                let accident_movement =
                    Trajectory::new_relative(locomotive_sprite.clone(), *accident_coord);
                accident_sprite.borrow_mut().set_movement(accident_movement);
                sprite_list.push(accident_sprite);
            }
        }

        (sprite_list, collision_list)
    }
    fn execute(&mut self) {
        let mut engine = RenderEngine::new(0);
        // Start rendering
        let (mut sprites, mut collisions) = self.select_sprites(std::env::args());
        engine.render(&mut sprites, &mut collisions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::sl::cli::Sl;

    #[test]
    fn test_select_base() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 3);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 1);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-a")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 5);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 1);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_little_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-al")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 10);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_fly_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-aF")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 5);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 1);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_little_fly_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-aFl")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 10);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_c_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-ac")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 5);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_c_fly_accident() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-aFc")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 5);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_little() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-l")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 5);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_c() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-c")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 3);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_little_fly() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-lF")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 5);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 2);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_c_fly() {
        let mut sl = Sl {};
        let args: Vec<String> = vec![String::from("sl"), String::from("-cF")];
        let (sprites, collisions) = sl.select_sprites(args.into_iter());

        assert_eq!(sprites.len(), 3);
        assert_eq!(sprites[2].borrow_mut().movement().speed(), 7);
        assert_eq!(sprites[2].borrow_mut().tdid(), 3);
        assert_eq!(collisions.len(), 0);
    }
}
