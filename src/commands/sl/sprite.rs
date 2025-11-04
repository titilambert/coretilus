use crate::animation::Animation;
use crate::frame::Frame;
use crate::sprite::{Sprite, SpriteRef};

const FRAME_D51_01: &str = include_str!("frames/D51_01.adoc");
const FRAME_D51_02: &str = include_str!("frames/D51_02.adoc");
const FRAME_D51_03: &str = include_str!("frames/D51_03.adoc");
const FRAME_D51_04: &str = include_str!("frames/D51_04.adoc");
const FRAME_D51_05: &str = include_str!("frames/D51_05.adoc");
const FRAME_D51_06: &str = include_str!("frames/D51_06.adoc");
const FRAME_LOGO_1: &str = include_str!("frames/logo_01.adoc");
const FRAME_LOGO_2: &str = include_str!("frames/logo_02.adoc");
const FRAME_LOGO_3: &str = include_str!("frames/logo_03.adoc");
const FRAME_LOGO_4: &str = include_str!("frames/logo_04.adoc");
const FRAME_LOGO_5: &str = include_str!("frames/logo_05.adoc");
const FRAME_LOGO_6: &str = include_str!("frames/logo_06.adoc");
const FRAME_C51_1: &str = include_str!("frames/C51_1.adoc");
const FRAME_C51_2: &str = include_str!("frames/C51_2.adoc");
const FRAME_C51_3: &str = include_str!("frames/C51_3.adoc");
const FRAME_C51_4: &str = include_str!("frames/C51_4.adoc");
const FRAME_C51_5: &str = include_str!("frames/C51_5.adoc");
const FRAME_C51_6: &str = include_str!("frames/C51_6.adoc");
const FRAME_SMOKE_1: &str = include_str!("frames/smoke_1.adoc");
const FRAME_SMOKE_2: &str = include_str!("frames/smoke_2.adoc");
const FRAME_SMOKE_3: &str = include_str!("frames/smoke_3.adoc");
const FRAME_SMOKE_4: &str = include_str!("frames/smoke_4.adoc");
const FRAME_ACCIDENT_1: &str = include_str!("frames/accident_01.adoc");
const FRAME_ACCIDENT_2: &str = include_str!("frames/accident_02.adoc");
const FRAME_COAL: &str = include_str!("frames/coal.adoc");
const FRAME_LOGO_COAL: &str = include_str!("frames/logo_coal.adoc");
const FRAME_LOGO_CAR: &str = include_str!("frames/logo_car.adoc");

pub fn get_sprite_d51() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_D51_01),
        Frame::new(FRAME_D51_02),
        Frame::new(FRAME_D51_03),
        Frame::new(FRAME_D51_04),
        Frame::new(FRAME_D51_05),
        Frame::new(FRAME_D51_06),
    ];
    let anim = Animation::new_movement_based(frames.clone(), 0, true);

    let sprite = Sprite::new(1, "Steam locomotive D51", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_logo() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_LOGO_1),
        Frame::new(FRAME_LOGO_2),
        Frame::new(FRAME_LOGO_3),
        Frame::new(FRAME_LOGO_4),
        Frame::new(FRAME_LOGO_5),
        Frame::new(FRAME_LOGO_6),
    ];

    let anim = Animation::new_movement_based(frames.clone(), 0, true);
    let sprite = Sprite::new(2, "Steam locomotive Logo", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_c51() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_C51_1),
        Frame::new(FRAME_C51_2),
        Frame::new(FRAME_C51_3),
        Frame::new(FRAME_C51_4),
        Frame::new(FRAME_C51_5),
        Frame::new(FRAME_C51_6),
    ];

    let anim = Animation::new_movement_based(frames.clone(), 0, true);
    let sprite = Sprite::new(3, "Steam locomotive C51", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_smoke() -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_SMOKE_1),
        Frame::new(FRAME_SMOKE_2),
        Frame::new(FRAME_SMOKE_3),
        Frame::new(FRAME_SMOKE_4),
    ];

    let anim = Animation::new_tick_based(frames.clone(), 0, 20, 0, true);
    let sprite = Sprite::new(4, "Smoke", 10);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_accident(start_frame_id: usize) -> SpriteRef {
    let frames = vec![
        Frame::new(FRAME_ACCIDENT_1),
        Frame::new(FRAME_ACCIDENT_1),
        Frame::new(FRAME_ACCIDENT_2),
        Frame::new(FRAME_ACCIDENT_2),
    ];

    let anim = Animation::new_tick_based(frames.clone(), start_frame_id, 50, 0, true);
    let sprite = Sprite::new(5, "Accident", 15);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_coal() -> SpriteRef {
    let anim = Animation::new_static(Frame::new(FRAME_COAL));
    let sprite = Sprite::new(6, "Coal", 12);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_logo_coal() -> SpriteRef {
    let anim = Animation::new_static(Frame::new(FRAME_LOGO_COAL));
    let sprite = Sprite::new(7, "Little coal", 12);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_logo_car() -> SpriteRef {
    let anim = Animation::new_static(Frame::new(FRAME_LOGO_CAR));
    let sprite = Sprite::new(8, "Litte car", 13);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
