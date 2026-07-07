use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::entity::sprite::Sprite;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;

const FRAME_DOG1: &str = include_str!("frames/dog1.adoc");
const FRAME_DOG2: &str = include_str!("frames/dog2.adoc");
const FRAME_DOG3: &str = include_str!("frames/dog3.adoc");
const FRAME_DOG4: &str = include_str!("frames/dog4.adoc");
const FRAME_DOG5: &str = include_str!("frames/dog5.adoc");
const FRAME_DOG6: &str = include_str!("frames/dog6.adoc");

pub fn get_object_dog() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_DOG1),
        Frame::new(FRAME_DOG2),
        Frame::new(FRAME_DOG3),
        Frame::new(FRAME_DOG4),
        Frame::new(FRAME_DOG5),
        Frame::new(FRAME_DOG6),
    ];
    let anim = SpriteAnimation::new_movement_based(frames.clone(), true);
    let sprite = Sprite::new(anim, true);
    Object::new(9, String::from("Running dog"), vec![sprite], None)
}

pub fn get_object_domain(domain_name: String) -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(domain_name));
    let sprite = Sprite::new(anim, true);
    Object::new(9, String::from("Domain"), vec![sprite], None)
}
