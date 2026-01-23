use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::entity::sprite::Sprite;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;

const FRAME_PARROT_1: &str = include_str!("frames/parrot_1.adoc");
const FRAME_PARROT_2: &str = include_str!("frames/parrot_2.adoc");
const FRAME_PARROT_3: &str = include_str!("frames/parrot_3.adoc");

pub fn get_word_object(domain_name: String) -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(domain_name));
    let sprite = Sprite::new(anim, true);
    Object::new(9, String::from("Domain"), vec![sprite], None)
}

pub fn get_object_parrot() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_PARROT_1),
        Frame::new(FRAME_PARROT_2),
        Frame::new(FRAME_PARROT_3),
        Frame::new(FRAME_PARROT_2),
    ];
    let anim = SpriteAnimation::new_tick_based(frames.clone(), 20, true, Some(0), false, false);
    let sprite = Sprite::new(anim, true);
    Object::new(9, String::from("Speaking parrot"), vec![sprite], None)
}
