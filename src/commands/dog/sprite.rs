use crate::animation::Animation;
use crate::frame::Frame;
use crate::sprite::{Sprite, SpriteRef};

const FRAME_DOG1: &str = include_str!("frames/dog1.adoc");
const FRAME_DOG2: &str = include_str!("frames/dog2.adoc");
const FRAME_DOG3: &str = include_str!("frames/dog3.adoc");
const FRAME_DOG4: &str = include_str!("frames/dog4.adoc");
const FRAME_DOG5: &str = include_str!("frames/dog5.adoc");
const FRAME_DOG6: &str = include_str!("frames/dog6.adoc");

pub fn get_sprite_dog() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_DOG1),
        Frame::new(FRAME_DOG2),
        Frame::new(FRAME_DOG3),
        Frame::new(FRAME_DOG4),
        Frame::new(FRAME_DOG5),
        Frame::new(FRAME_DOG6),
    ];
    let anim = Animation::new_movement_based(frames.clone(), 0, true);

    let sprite = Sprite::new(9, String::from("Running dog"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_domain(domain_name: String) -> SpriteRef {
    let frame = Frame::new(domain_name);

    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(9, String::from("Domain"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
