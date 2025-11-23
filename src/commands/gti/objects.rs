use crate::engine_v2::object::{
    frame::Frame,
    object::{Object, ObjectRef},
    sprite::Sprite,
    sprite_animation::SpriteAnimation,
};

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

pub fn get_object_std() -> ObjectRef {
    let frames = vec![Frame::new(FRAME_STD1), Frame::new(FRAME_STD2)];
    let anim = SpriteAnimation::new_movement_based(frames, true);
    let sprite = Sprite::new(anim);

    let object = Object::new(9, String::from("Golf GTI"), vec![sprite]);
    object
}

pub fn get_object_pull() -> ObjectRef {
    let frames = vec![Frame::new(FRAME_PULL1), Frame::new(FRAME_PULL2)];
    let anim = SpriteAnimation::new_movement_based(frames, true);
    let sprite = Sprite::new(anim);
    let object = Object::new(10, String::from("Golf GTI pulled"), vec![sprite]);
    object
}

pub fn get_object_push() -> ObjectRef {
    let frames = vec![Frame::new(FRAME_PUSH1), Frame::new(FRAME_PUSH2)];
    let anim = SpriteAnimation::new_movement_based(frames, true);
    let sprite = Sprite::new(anim);

    let object = Object::new(11, String::from("Golf GTI pushed"), vec![sprite]);
    object
}

pub fn get_object_tag() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_TAG1),
        Frame::new(FRAME_TAG2),
        Frame::new(FRAME_TAG3),
    ];
    let anim = SpriteAnimation::new_tick_based(frames, 20, true);
    let sprite = Sprite::new(anim);

    let object = Object::new(12, String::from("Golf GTI tagged"), vec![sprite]);
    object
}

pub fn get_object_commit() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_COMMIT1),
        Frame::new(FRAME_COMMIT2),
        Frame::new(FRAME_COMMIT3),
    ];
    let anim = SpriteAnimation::new_tick_based(frames, 20, true);
    let sprite = Sprite::new(anim);

    let object = Object::new(13, String::from("Golf GTI commited"), vec![sprite]);
    object
}
