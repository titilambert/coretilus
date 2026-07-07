use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::entity::sprite::Sprite;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;

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

pub fn get_object_d51() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_D51_01),
        Frame::new(FRAME_D51_02),
        Frame::new(FRAME_D51_03),
        Frame::new(FRAME_D51_04),
        Frame::new(FRAME_D51_05),
        Frame::new(FRAME_D51_06),
    ];
    let anim = SpriteAnimation::new_movement_based(frames.clone(), true);
    let sprite = Sprite::new(anim, true);

    Object::new(1, String::from("Steam locomotive D51"), vec![sprite], None)
}

pub fn get_object_logo() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_LOGO_1),
        Frame::new(FRAME_LOGO_2),
        Frame::new(FRAME_LOGO_3),
        Frame::new(FRAME_LOGO_4),
        Frame::new(FRAME_LOGO_5),
        Frame::new(FRAME_LOGO_6),
    ];

    let anim = SpriteAnimation::new_movement_based(frames.clone(), true);
    let sprite = Sprite::new(anim, true);

    Object::new(2, String::from("Steam locomotive Logo"), vec![sprite], None)
}

pub fn get_object_c51() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_C51_1),
        Frame::new(FRAME_C51_2),
        Frame::new(FRAME_C51_3),
        Frame::new(FRAME_C51_4),
        Frame::new(FRAME_C51_5),
        Frame::new(FRAME_C51_6),
    ];

    let anim = SpriteAnimation::new_movement_based(frames.clone(), true);
    let sprite = Sprite::new(anim, true);
    Object::new(3, String::from("Steam locomotive C51"), vec![sprite], None)
}

pub fn get_object_smoke() -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_SMOKE_1),
        Frame::new(FRAME_SMOKE_2),
        Frame::new(FRAME_SMOKE_3),
        Frame::new(FRAME_SMOKE_4),
    ];

    let anim = SpriteAnimation::new_tick_based(frames.clone(), 20, true, None, false, false);
    let sprite = Sprite::new(anim, true);
    Object::new(4, String::from("Smoke"), vec![sprite], None)
}

pub fn get_object_accident(start_frame_id: usize) -> ObjectRef {
    let frames = vec![
        Frame::new(FRAME_ACCIDENT_1),
        Frame::new(FRAME_ACCIDENT_1),
        Frame::new(FRAME_ACCIDENT_2),
        Frame::new(FRAME_ACCIDENT_2),
    ];

    let anim = SpriteAnimation::new_tick_based(
        frames.clone(),
        50,
        true,
        Some(start_frame_id),
        false,
        false,
    );
    let sprite = Sprite::new(anim, true);
    Object::new(5, String::from("Accident"), vec![sprite], None)
}

pub fn get_object_coal() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_COAL));
    let sprite = Sprite::new(anim, true);
    Object::new(6, String::from("Coal"), vec![sprite], None)
}

pub fn get_object_logo_coal() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_LOGO_COAL));
    let sprite = Sprite::new(anim, true);
    Object::new(7, String::from("Little coal"), vec![sprite], None)
}

pub fn get_object_logo_car() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_LOGO_CAR));
    let sprite = Sprite::new(anim, true);
    Object::new(8, String::from("Little car"), vec![sprite], None)
}
