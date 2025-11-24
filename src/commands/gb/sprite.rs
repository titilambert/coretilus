use crate::animation::Animation;
use crate::frame::Frame;
use crate::sprite::{Sprite, SpriteRef};

const FRAME_GAMEBOY: &str = include_str!("frames/gb.adoc");
const FRAME_SHAPE1: &str = include_str!("frames/shape_1.adoc");
const FRAME_SHAPE2: &str = include_str!("frames/shape_2.adoc");
const FRAME_SHAPE3: &str = include_str!("frames/shape_3.adoc");
const FRAME_SHAPE4: &str = include_str!("frames/shape_4.adoc");
const FRAME_SHAPE5: &str = include_str!("frames/shape_5.adoc");
const FRAME_SHAPE6: &str = include_str!("frames/shape_6.adoc");
const FRAME_SHAPE7: &str = include_str!("frames/shape_7.adoc");

pub fn get_sprite_gameboy() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_GAMEBOY)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(50, String::from("Gameboy"), 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_shape1() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SHAPE1)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(61, String::from("Tetris square"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite.borrow_mut().collider().set_active(false);
    sprite
}

pub fn get_sprite_shape2() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SHAPE2)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(62, String::from("Tetris square"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite.borrow_mut().collider().set_active(false);
    sprite
}

pub fn get_sprite_shape3() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SHAPE3)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(63, String::from("Tetris square"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite.borrow_mut().collider().set_active(false);
    sprite
}

pub fn get_sprite_shape4() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SHAPE4)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(50, String::from("Tetris square"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_shape5() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SHAPE5)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(50, String::from("Tetris square"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_shape6() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SHAPE6)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(50, String::from("Tetris square"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_shape7() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SHAPE7)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(50, String::from("Tetris square"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_bottom() -> SpriteRef {
    let frame = Frame::new("f".repeat(29));

    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(9, String::from("GB bottom"), 1);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
