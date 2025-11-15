use std::cell::RefCell;
use std::rc::Rc;

use crate::collision::Collision;
use crate::commands::pc::sprite::{
    get_sprite_cachel2, get_sprite_chipset, get_sprite_cpu, get_sprite_data,
    get_sprite_motherboard, get_sprite_ram,
};
use crate::coord::{Position, XTermPosition, YTermPosition};
use crate::engine::RenderEngine;
use crate::sprite::{Sprite, SpriteRef};
use crate::tools::parse_args;
use crate::trajectory::Trajectory;

use crate::command::Command;

// Helper function to create a sprite with movement and visibility
fn create_sprite(
    mut sprite_list: Vec<Rc<RefCell<Sprite>>>,
    get_sprite_fn: fn() -> SpriteRef,
    start_position: Position,
    end_position: Option<Position>,
    speed: usize,
    visible: bool,
) -> SpriteRef {
    let sprite = get_sprite_fn();

    let mut trajectory = Trajectory::new_stationary(start_position, 0);
    if let Some(end_p) = end_position {
        trajectory = Trajectory::new_linear(start_position, end_p, speed as i32);
    }

    sprite.borrow_mut().set_trajectory(trajectory);
    sprite.borrow_mut().set_visible(visible);
    sprite_list.push(sprite.clone());
    sprite
}

pub struct Pc {}

impl Command for Pc {
    fn get_all_sprites(&self) -> Vec<fn() -> SpriteRef> {
        vec![get_sprite_motherboard]
    }

    fn select_sprites(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<SpriteRef>, Vec<Collision>) {
        let short_flags: &[char] = &[];
        let long_flags: &[&str] = &[];
        let params: &[&str] = &[];
        let (_, _) = parse_args(args.collect(), short_flags, long_flags, params);
        let mut sprites: Vec<SpriteRef> = Vec::new();
        let speed: usize = 17;

        // Motherboard
        let motherboard_sprite = create_sprite(
            sprites.clone(),
            get_sprite_motherboard,
            Position::new(XTermPosition::Coord(2), YTermPosition::Coord(1)),
            None,
            0,
            true,
        );
        // Chipset
        let chipsed_sprite = create_sprite(
            sprites.clone(),
            get_sprite_chipset,
            Position::new(XTermPosition::Coord(62), YTermPosition::Coord(16)),
            None,
            0,
            true,
        );
        // RAM
        let ram_sprite = create_sprite(
            sprites.clone(),
            get_sprite_ram,
            Position::new(XTermPosition::Coord(49), YTermPosition::Coord(10)),
            None,
            0,
            true,
        );
        // Cachel2
        let cachel2_sprite = get_sprite_cachel2();
        let movement = Trajectory::new_stationary(
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(22)),
            0,
        );
        cachel2_sprite.borrow_mut().set_trajectory(movement);
        // CPU
        let cpu_sprite = get_sprite_cpu();
        let movement = Trajectory::new_stationary(
            Position::new(XTermPosition::Coord(58), YTermPosition::Coord(23)),
            0,
        );
        cpu_sprite.borrow_mut().set_trajectory(movement);
        // DATA1
        let data1_sprite = get_sprite_data(1);
        //
        //let data1_sprite = get_sprite_data1();
        let movement = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(36), YTermPosition::Coord(30)),
            Position::new(XTermPosition::Coord(36), YTermPosition::Coord(20)),
            speed as i32,
        );
        data1_sprite.borrow_mut().set_trajectory(movement);
        // DATA2
        let data2_sprite = get_sprite_data(2);
        data2_sprite.borrow_mut().set_visible(false);
        let movement2 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(34), YTermPosition::Coord(22)),
            Position::new(XTermPosition::Coord(66), YTermPosition::Coord(22)),
            speed as i32,
        );
        data2_sprite.borrow_mut().set_trajectory(movement2);
        // DATA3
        let data3_sprite = get_sprite_data(3);
        data3_sprite.borrow_mut().set_visible(false);
        let movement3 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(22)),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(19)),
            speed as i32,
        );
        data3_sprite.borrow_mut().set_trajectory(movement3);
        // DATA4
        let data4_sprite = get_sprite_data(4);
        let data4_sprite_id = data4_sprite.borrow().id();
        data4_sprite.borrow_mut().set_visible(false);
        let movement4 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(15)),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(10)),
            speed as i32,
        );
        data4_sprite.borrow_mut().set_trajectory(movement4);
        // DATA5
        let data5_sprite = get_sprite_data(5);
        let data5_sprite_id = data5_sprite.borrow().id();
        data5_sprite.borrow_mut().set_visible(false);
        let movement5 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(10)),
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(21)),
            speed as i32,
        );
        data5_sprite.borrow_mut().set_trajectory(movement5);
        // DATA6
        let data6_sprite = get_sprite_data(6);
        let data6_sprite_id = data6_sprite.borrow().id();
        data6_sprite.borrow_mut().set_visible(false);
        let movement6 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(76), YTermPosition::Coord(32)),
            Position::new(XTermPosition::Coord(58), YTermPosition::Coord(32)),
            speed as i32,
        );
        data6_sprite.borrow_mut().set_trajectory(movement6);
        // DATA7
        let data7_sprite = get_sprite_data(7);
        data7_sprite.borrow_mut().set_visible(false);
        let movement7 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(61), YTermPosition::Coord(32)),
            Position::new(XTermPosition::Coord(61), YTermPosition::Coord(28)),
            speed as i32,
        );
        data7_sprite.borrow_mut().set_trajectory(movement7);
        // DATA8
        let data8_sprite = get_sprite_data(8);
        let data8_sprite_id = data8_sprite.borrow().id();
        data8_sprite.borrow_mut().set_visible(false);
        let movement8 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(28)),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(32)),
            speed as i32,
        );
        data8_sprite.borrow_mut().set_trajectory(movement8);
        // DATA9
        let data9_sprite = get_sprite_data(9);
        data9_sprite.borrow_mut().set_visible(false);
        let movement9 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(66), YTermPosition::Coord(32)),
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(32)),
            speed as i32,
        );
        data9_sprite.borrow_mut().set_trajectory(movement9);
        // DATA10
        let data10_sprite = get_sprite_data(10);
        let data10_sprite_id = data10_sprite.borrow().id();
        data10_sprite.borrow_mut().set_visible(false);
        let movement10 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(21)),
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(10)),
            speed as i32,
        );
        data10_sprite.borrow_mut().set_trajectory(movement10);
        // DATA11
        let data11_sprite = get_sprite_data(11);
        let data11_sprite_id = data11_sprite.borrow().id();
        data11_sprite.borrow_mut().set_visible(false);
        let movement11 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(10)),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(16)),
            speed as i32,
        );
        data11_sprite.borrow_mut().set_trajectory(movement11);
        // DATA12
        let data12_sprite = get_sprite_data(12);
        let data12_sprite_id = data12_sprite.borrow().id();
        data12_sprite.borrow_mut().set_visible(false);
        let movement12 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(19)),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(22)),
            speed as i32,
        );
        data12_sprite.borrow_mut().set_trajectory(movement12);
        // DATA13
        let data13_sprite = get_sprite_data(13);
        let data13_sprite_id = data13_sprite.borrow().id();
        data13_sprite.borrow_mut().set_visible(false);
        let movement13 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(65), YTermPosition::Coord(22)),
            Position::new(XTermPosition::Coord(20), YTermPosition::Coord(22)),
            speed as i32,
        );
        data13_sprite.borrow_mut().set_trajectory(movement13);

        // DATA14
        let data14_sprite = get_sprite_data(14);
        data14_sprite.borrow_mut().set_visible(false);
        let movement14 = Trajectory::new_linear(
            Position::new(XTermPosition::Coord(23), YTermPosition::Coord(20)),
            Position::new(XTermPosition::Coord(23), YTermPosition::Coord(31)),
            speed as i32,
        );
        data14_sprite.borrow_mut().set_trajectory(movement14);

        // COLLISIONS
        // Collision data1/data2
        let collision1 = Collision::new_sprite(
            data1_sprite.clone(),
            data2_sprite.clone(),
            move |data1_spritec, data2_spritec, counter, _| {
                data2_spritec.borrow_mut().set_visible(true);
                if counter == 2 * speed {
                    data1_spritec.borrow_mut().set_visible(false);
                }
            },
        );

        let mut collisions: Vec<Collision> = Vec::new();
        collisions.push(collision1);

        // Collision data2/data3
        let collision2 = Collision::new_sprite(
            data2_sprite.clone(),
            data3_sprite.clone(),
            move |data2_spritec, data3_spritec, counter, _| {
                if counter == 2 * speed {
                    data3_spritec.borrow_mut().set_visible(true);
                }
                if counter == 4 * speed {
                    data2_spritec.borrow_mut().set_visible(false);
                }
            },
        );
        collisions.push(collision2);

        // Collision data3/chipset
        let collision3 = Collision::new_sprite(
            data3_sprite.clone(),
            chipsed_sprite.clone(),
            move |data3_spritec, _, counter, engine| {
                if counter == 3 * speed {
                    data3_spritec.borrow_mut().set_visible(false);
                }
                if counter == speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data4_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
            },
        );
        collisions.push(collision3);

        // Collision data4/ram
        let collision4 = Collision::new_sprite(
            data4_sprite.clone(),
            ram_sprite.clone(),
            move |data4_spritec, _, counter, engine| {
                if counter == 2 * speed {
                    data4_spritec.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data5_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
            },
        );
        collisions.push(collision4);

        // Collision data5/cachel2
        let collision5 = Collision::new_sprite(
            data5_sprite.clone(),
            cachel2_sprite.clone(),
            move |data5_spritec, _, counter, engine| {
                if counter == 3 * speed {
                    data5_spritec.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data6_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
            },
        );
        collisions.push(collision5);
        // Collision data6/data7
        let collision6 = Collision::new_sprite(
            data6_sprite.clone(),
            data7_sprite.clone(),
            move |data6_spritec, data7_spritec, counter, engine| {
                if counter == 2 * speed {
                    data7_spritec.borrow_mut().set_visible(true);
                }
                if counter == 4 * speed {
                    data6_spritec.borrow_mut().set_visible(false);
                }
                if counter == 4 * speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data8_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
            },
        );
        collisions.push(collision6);
        // Collision data8/data9
        let collision8 = Collision::new_sprite(
            data8_sprite.clone(),
            data9_sprite.clone(),
            move |data8_spritec, data9_spritec, counter, _| {
                if counter == speed {
                    data9_spritec.borrow_mut().set_visible(true);
                }
                if counter == 2 * speed {
                    data8_spritec.borrow_mut().set_visible(false);
                }
            },
        );
        collisions.push(collision8);
        // Collision data9/cachel2
        let collision9 = Collision::new_sprite(
            data9_sprite.clone(),
            cachel2_sprite.clone(),
            move |data9_spritec, _, counter, engine| {
                if counter == 3 * speed {
                    data9_spritec.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data10_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
            },
        );
        collisions.push(collision9);
        // Collision data10/ram
        let collision10 = Collision::new_sprite(
            data10_sprite.clone(),
            ram_sprite.clone(),
            move |data10_spritec, _, counter, engine| {
                if counter == 2 * speed {
                    data10_spritec.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data11_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
            },
        );
        collisions.push(collision10);
        // Collision data11/chipset
        let collision11 = Collision::new_sprite(
            data11_sprite.clone(),
            chipsed_sprite.clone(),
            move |data11_spritec, _, counter, engine| {
                if counter == 3 * speed {
                    data11_spritec.borrow_mut().set_visible(false);
                }
                if counter == speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data12_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
                if counter == 2 * speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data13_sprite_id {
                            sprite.borrow_mut().set_visible(true);
                        }
                    }
                }
                if counter == 4 * speed {
                    for sprite in engine.sprites() {
                        if sprite.borrow().id() == data12_sprite_id {
                            sprite.borrow_mut().set_visible(false);
                        }
                    }
                }
            },
        );
        collisions.push(collision11);
        // Collision data13/data14
        let collision13 = Collision::new_sprite(
            data13_sprite.clone(),
            data14_sprite.clone(),
            move |data13_spritec, data14_spritec, counter, _| {
                if counter == speed {
                    data14_spritec.borrow_mut().set_visible(true);
                }
                if counter == 3 * speed {
                    data13_spritec.borrow_mut().set_visible(false);
                }
            },
        );
        collisions.push(collision13);

        sprites.push(motherboard_sprite);
        sprites.push(chipsed_sprite);
        sprites.push(ram_sprite);
        sprites.push(cachel2_sprite);
        sprites.push(cpu_sprite);
        sprites.push(data1_sprite);
        sprites.push(data2_sprite);
        sprites.push(data3_sprite);
        sprites.push(data4_sprite);
        sprites.push(data5_sprite);
        sprites.push(data6_sprite);
        sprites.push(data7_sprite);
        sprites.push(data8_sprite);
        sprites.push(data9_sprite);
        sprites.push(data10_sprite);
        sprites.push(data11_sprite);
        sprites.push(data12_sprite);
        sprites.push(data13_sprite);
        sprites.push(data14_sprite);

        (sprites, collisions)
    }
    fn execute(&mut self) {
        let mut engine = RenderEngine::new(0);
        let (mut sprites, mut collisions) = self.select_sprites(std::env::args());

        engine.render(&mut sprites, &mut collisions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::gti::cli::Gti;

    #[test]
    fn test_select_data1() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("pc")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 2);
        assert_eq!(collisions.len(), 0);
    }
}
