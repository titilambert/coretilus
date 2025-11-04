use crate::animation::Animation;
use crate::frame::Frame;
use crate::sprite::{Sprite, SpriteRef};

const FRAME_STD1: &str = include_str!("frames/std1.adoc");
const FRAME_STD2: &str = include_str!("frames/std2.adoc");
const FRAME_PULL1: &str = include_str!("frames/pull1.adoc");
const FRAME_PULL2: &str = include_str!("frames/pull2.adoc");
const FRAME_PUSH1: &str = include_str!("frames/push1.adoc");
const FRAME_PUSH2: &str = include_str!("frames/push2.adoc");
const FRAME_TAG1: &str = include_str!("frames/tag1.adoc");
const FRAME_TAG2: &str = include_str!("frames/tag2.adoc");
const FRAME_TAG3: &str = include_str!("frames/tag3.adoc");
const FRAME_COMMIT1: &str = include_str!("frames/commit1.adoc");
const FRAME_COMMIT2: &str = include_str!("frames/commit2.adoc");
const FRAME_COMMIT3: &str = include_str!("frames/commit3.adoc");

pub fn get_sprite_std() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_STD1), Frame::new(FRAME_STD2)];
    let anim = Animation::new_movement_based(frames.clone(), 0, true);

    let sprite = Sprite::new(9, "Golf GTI", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_pull() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_PULL1), Frame::new(FRAME_PULL2)];
    let anim = Animation::new_movement_based(frames.clone(), 0, true);

    let sprite = Sprite::new(10, "Golf GTI pulled", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_push() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_PUSH1), Frame::new(FRAME_PUSH2)];
    let anim = Animation::new_movement_based(frames.clone(), 0, true);

    let sprite = Sprite::new(11, "Golf GTI pushed", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_tag() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_TAG1),
        Frame::new(FRAME_TAG2),
        Frame::new(FRAME_TAG3),
    ];
    let anim = Animation::new_tick_based(frames.clone(), 0, 20, 300, true);

    let sprite = Sprite::new(12, "Golf GTI tagged", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_commit() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_COMMIT1),
        Frame::new(FRAME_COMMIT2),
        Frame::new(FRAME_COMMIT3),
    ];
    let anim = Animation::new_tick_based(frames.clone(), 0, 20, 300, true);

    let sprite = Sprite::new(13, "Golf GTI commited", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
