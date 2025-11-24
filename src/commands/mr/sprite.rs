use crate::animation::Animation;
use crate::collision::Collider;
use crate::coord::Coord;
use crate::engine::Size;
use crate::frame::Frame;
use crate::sprite::Sprite;
use crate::sprite::SpriteRef;

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

pub fn get_sprite_mini() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_MINI1),
        Frame::new(FRAME_MINI2),
        Frame::new(FRAME_MINI3),
    ];

    let collider = Collider::new(
        Coord::new(-1, 2),
        Size::new(frames[0].get_width() + 1, frames[0].get_height() - 2),
        true,
    );
    let anim = Animation::new_movement_based(frames.clone(), 0, true);
    let sprite = Sprite::new(14, String::from("Rocket mini"), 10);
    sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_mini_landed() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_MINI1)];
    let anim = Animation::new_tick_based(frames.clone(), 0, 1, 50, true);
    let sprite = Sprite::new(15, String::from("Landed Rocket mini"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_std() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_STD1),
        Frame::new(FRAME_STD2),
        Frame::new(FRAME_STD3),
    ];
    let collider = Collider::new(
        Coord::new(-1, 2),
        Size::new(frames[0].get_width() + 1, frames[0].get_height() - 2),
        true,
    );
    let anim = Animation::new_movement_based(frames.clone(), 0, true);
    let sprite = Sprite::new(16, String::from("Rocket"), 10);
    sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_std_landed() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_STD1)];
    let anim = Animation::new_tick_based(frames.clone(), 0, 1, 50, true);
    let sprite = Sprite::new(17, String::from("Landed Rocket"), 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_spaceport() -> SpriteRef {
    let anim = Animation::new_static(Frame::new(FRAME_SPACEPORT));

    let sprite = Sprite::new(18, String::from("Spaceport"), 0);
    sprite.borrow_mut().collider().set_active(true);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_explosion() -> SpriteRef {
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
    let anim = Animation::new_tick_based(frames.clone(), 0, 20, 0, false);

    let sprite = Sprite::new(19, String::from("Explosion"), 1);
    sprite.borrow_mut().set_visible(false);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_sign_land() -> SpriteRef {
    let anim = Animation::new_static(Frame::new(FRAME_SIGN_LAND));

    let sprite = Sprite::new(20, String::from("Land rocket sign"), 0);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_sign_success() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_SIGN_SUCCESS)];
    let anim = Animation::new_tick_based(frames.clone(), 0, 1, 200, true);

    let sprite = Sprite::new(21, String::from("Landed rocket sign"), 0);

    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_sign_fail() -> SpriteRef {
    let anim = Animation::new_static(Frame::new(FRAME_SIGN_FAILED));

    let sprite = Sprite::new(22, String::from("Rocket crashed sign"), 0);

    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_sign_tryagain() -> SpriteRef {
    let anim = Animation::new_static(Frame::new(FRAME_SIGN_TRYAGAIN));

    let sprite = Sprite::new(23, String::from("Rocket crashed try again sign"), 0);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
