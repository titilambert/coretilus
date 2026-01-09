use crate::engine_v2::collision::Collider;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::entity::sprite::Sprite;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;
use crate::engine_v2::size::Size;

const FRAME_MINI1: &str = include_str!("./frames/mini1.adoc");
const FRAME_MINI2: &str = include_str!("./frames/mini2.adoc");
const FRAME_MINI3: &str = include_str!("./frames/mini3.adoc");
const FRAME_STD1: &str = include_str!("frames/std1.adoc");
const FRAME_STD2: &str = include_str!("frames/std2.adoc");
const FRAME_STD3: &str = include_str!("frames/std3.adoc");
const FRAME_SPACEPORT: &str = include_str!("frames/spaceport.adoc");
const FRAME_SIGN_LAND: &str = include_str!("frames/sign_land.adoc");
const FRAME_SIGN_TRYAGAIN: &str = include_str!("frames/sign_tryagain.adoc");
const FRAME_SIGN_FAILED: &str = include_str!("frames/sign_failed.adoc");
const FRAME_SIGN_SUCCESS: &str = include_str!("frames/sign_success.adoc");
const FRAME_EXPLOSION1: &str = include_str!("frames/explosion1.adoc");
const FRAME_EXPLOSION2: &str = include_str!("frames/explosion2.adoc");
const FRAME_EXPLOSION3: &str = include_str!("frames/explosion3.adoc");
const FRAME_EXPLOSION4: &str = include_str!("frames/explosion4.adoc");
const FRAME_EXPLOSION5: &str = include_str!("frames/explosion5.adoc");
const FRAME_EXPLOSION6: &str = include_str!("frames/explosion6.adoc");
const FRAME_EXPLOSION7: &str = include_str!("frames/explosion7.adoc");
const FRAME_EXPLOSION8: &str = include_str!("frames/explosion8.adoc");
const FRAME_EXPLOSION9: &str = include_str!("frames/explosion9.adoc");
const FRAME_EXPLOSION10: &str = include_str!("frames/explosion10.adoc");

pub fn get_object_mini() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_MINI1),
        Frame::new(FRAME_MINI2),
        Frame::new(FRAME_MINI3),
    ];

    let collider = Collider::new(
        Coords::new(-1, 1, 0),
        Size::new(frames[0].get_width() + 1, frames[0].get_height() - 1),
        true,
    );
    let anim = SpriteAnimation::new_movement_based(frames, true);
    let sprite = Sprite::new(anim);
    Object::new(
        14,
        String::from("Rocket mini"),
        vec![sprite],
        Some(collider),
    )
}

pub fn get_object_std() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_STD1),
        Frame::new(FRAME_STD2),
        Frame::new(FRAME_STD3),
    ];
    let collider = Collider::new(
        Coords::new(-1, 1, 0),
        Size::new(frames[0].get_width() + 1, frames[0].get_height() - 1),
        true,
    );
    let anim = SpriteAnimation::new_movement_based(frames, true);
    let sprite = Sprite::new(anim);
    Object::new(16, String::from("Rocket"), vec![sprite], Some(collider))
}

pub fn get_object_spaceport() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SPACEPORT));
    let sprite = Sprite::new(anim);
    Object::new(18, String::from("Spaceport"), vec![sprite], None)
}

pub fn get_object_explosion() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_EXPLOSION1),
        Frame::new(FRAME_EXPLOSION2),
        Frame::new(FRAME_EXPLOSION3),
        Frame::new(FRAME_EXPLOSION4),
        Frame::new(FRAME_EXPLOSION5),
        Frame::new(FRAME_EXPLOSION6),
        Frame::new(FRAME_EXPLOSION7),
        Frame::new(FRAME_EXPLOSION8),
        Frame::new(FRAME_EXPLOSION9),
        Frame::new(FRAME_EXPLOSION10),
    ];
    let anim = SpriteAnimation::new_tick_based(frames.clone(), 40, false, None);
    let sprite = Sprite::new(anim);
    Object::new(19, String::from("Explosion"), vec![sprite], None)
}

pub fn get_object_sign_land() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SIGN_LAND));
    let sprite = Sprite::new(anim);
    Object::new(20, String::from("Land rocket sign"), vec![sprite], None)
}

pub fn get_object_sign_success() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SIGN_SUCCESS));
    let sprite = Sprite::new(anim);
    Object::new(21, String::from("Landed rocket sign"), vec![sprite], None)
}

pub fn get_object_sign_fail() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SIGN_FAILED));
    let sprite = Sprite::new(anim);
    Object::new(22, String::from("Rocket crashed sign"), vec![sprite], None)
}

pub fn get_object_sign_tryagain() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SIGN_TRYAGAIN));
    let sprite = Sprite::new(anim);

    Object::new(
        23,
        String::from("Rocket crashed try again sign"),
        vec![sprite],
        None,
    )
}
