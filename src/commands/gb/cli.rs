use std::vec;

use crossterm::event::KeyCode;
use rand::Rng;

use crate::collision::Collision;
use crate::commands::gb::sprite::{
    get_sprite_bottom, get_sprite_gameboy, get_sprite_shape1, get_sprite_shape2, get_sprite_shape3,
    get_sprite_shape4, get_sprite_shape5, get_sprite_shape6, get_sprite_shape7,
};
use crate::coord::{Coord, Position, XTermPosition, YTermPosition};
use crate::engine::RenderEngine;
use crate::sprite::{Sprite, SpriteRef};
use crate::tools::parse_args;
use crate::trajectory::Trajectory;

use crate::command::Command;

pub struct Gb {}

impl Command for Gb {
    fn get_all_sprites(&self) -> Vec<fn() -> SpriteRef> {
        vec![get_sprite_gameboy]
    }

    fn select_sprites(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<SpriteRef>, Vec<Collision>) {
        let mut sprites: Vec<SpriteRef> = Vec::new();
        let mut shape_sprites: Vec<SpriteRef> = Vec::new();
        let mut collisions: Vec<Collision> = Vec::new();
        // background
        let gb_sprite = get_sprite_gameboy();
        let trajectory = Trajectory::new_stationary(
            Position::new(XTermPosition::Coord(0), YTermPosition::Coord(0)),
            0,
        );
        gb_sprite.borrow_mut().set_trajectory(trajectory);

        // bottom
        let gb_bottom_sprite = get_sprite_bottom();
        let trajectory = Trajectory::new_stationary(
            Position::new(XTermPosition::Coord(10), YTermPosition::Coord(22)),
            0,
        );
        gb_bottom_sprite.borrow_mut().set_trajectory(trajectory);

        let speed = 100;
        let mut rng = rand::rng();
        for shape_type in 0..3 {
            let mut sprite_shape = get_sprite_shape1();
            //let shape_type: u8 = rng.random_range(1..=7);
            if shape_type == 0 {
                sprite_shape = get_sprite_shape1();
            } else if shape_type == 1 {
                sprite_shape = get_sprite_shape2();
            } else if shape_type == 2 {
                sprite_shape = get_sprite_shape3();
            } else if shape_type == 4 {
                sprite_shape = get_sprite_shape4();
            } else if shape_type == 5 {
                sprite_shape = get_sprite_shape5();
            } else if shape_type == 6 {
                sprite_shape = get_sprite_shape6();
            } else if shape_type == 7 {
                sprite_shape = get_sprite_shape7();
            }
            let trajectory = Trajectory::new_linear(
                Position::new(XTermPosition::Coord(22), YTermPosition::Coord(32)),
                Position::new(XTermPosition::Coord(22), YTermPosition::Coord(22)),
                speed,
            );
            sprite_shape.borrow_mut().set_trajectory(trajectory);
            if sprites.len() > 0 {
                sprite_shape.borrow_mut().set_visible(false);
            }
            // shape control
            Sprite::on_key(&sprite_shape, KeyCode::Char('d'), |s| {
                let x_offset = s.borrow_mut().trajectory().offset().x();
                if !s.borrow_mut().is_done() && x_offset < 11 {
                    s.borrow_mut().trajectory().add_offset(Coord::new(2, 0))
                }
            });
            Sprite::on_key(&sprite_shape, KeyCode::Right, |s| {
                let x_offset = s.borrow_mut().trajectory().offset().x();
                if !s.borrow_mut().is_done() && x_offset < 11 {
                    s.borrow_mut().trajectory().add_offset(Coord::new(2, 0))
                }
            });
            Sprite::on_key(&sprite_shape, KeyCode::Char('a'), |s| {
                let x_offset = s.borrow_mut().trajectory().offset().x();
                if !s.borrow_mut().is_done() && x_offset > -12 {
                    s.borrow_mut().trajectory().add_offset(Coord::new(-2, 0));
                }
            });
            Sprite::on_key(&sprite_shape, KeyCode::Left, |s| {
                let x_offset = s.borrow_mut().trajectory().offset().x();
                if !s.borrow_mut().is_done() && x_offset > -12 {
                    s.borrow_mut().trajectory().add_offset(Coord::new(-2, 0))
                }
            });
            sprites.push(sprite_shape.clone());
            shape_sprites.push(sprite_shape.clone());
        }

        for (index, shape_sprite) in shape_sprites.iter().enumerate() {
            if index + 1 >= shape_sprites.len() {
                break;
            }
            let next_sprite_shape_id = shape_sprites[index + 1].borrow().id();

            let collision1 = Collision::new_sprite(
                shape_sprite.clone(),
                gb_bottom_sprite.clone(),
                move |sprite_shape, _, _, engine| {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == next_sprite_shape_id {
                            sprite.borrow_mut().set_visible(true);
                            sprite.borrow_mut().trajectory().reset_offset();
                            // TODO set offset sprite.borrow_mut().trajectory()
                            break;
                        }
                    }
                    // TODO check if a line is complete
                    //sprite_shape1.borrow_mut();
                },
            );
            collisions.push(collision1);

            for n_index in (index + 1)..shape_sprites.len() {
                if n_index + 1 >= shape_sprites.len() {
                    break;
                }
                let l_next_sprite_shape = shape_sprites[n_index].clone();
                let n_next_sprite_shape_id = shape_sprites[n_index + 1].borrow().id();
                let collision = Collision::new_sprite(
                    shape_sprite.clone(),
                    l_next_sprite_shape.clone(),
                    move |sprite_shape, next_sprite_shape, _, engine| {
                        if sprite_shape.borrow_mut().trajectory().is_done() {
                            //l_next_sprite_shape.borrow_mut().trajectory().stop();
                            for sprite in engine.sprites() {
                                if sprite.borrow().id() == n_next_sprite_shape_id {
                                    //panic!("BBBB");
                                    sprite.borrow_mut().set_visible(true);
                                    sprite.borrow_mut().collider().set_active(true);
                                    sprite.borrow_mut().trajectory().reset_offset();
                                    break;
                                }
                            }
                            //return false;
                        }
                        //true
                        // TODO check if a line is complete
                        //sprite_shape1.borrow_mut();
                    },
                );
                collisions.push(collision);
            }
        }

        sprites.push(gb_sprite);
        sprites.push(gb_bottom_sprite);
        (sprites, collisions)
    }

    fn execute(&mut self) {
        let mut engine = RenderEngine::new(0);
        let (mut sprites, mut collisions) = self.select_sprites(std::env::args());

        engine.render(&mut sprites, &mut collisions);
    }
}
