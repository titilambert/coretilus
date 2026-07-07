use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::entity::sprite::Sprite;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;

const FRAME_GAMEBOY: &str = include_str!("frames/gb.adoc");
const FRAME_SHAPE1_1: &str = include_str!("frames/shape_1_1.adoc");
const FRAME_SHAPE1_2: &str = include_str!("frames/shape_1_2.adoc");
const FRAME_SHAPE2_1: &str = include_str!("frames/shape_2_1.adoc");
const FRAME_SHAPE2_2: &str = include_str!("frames/shape_2_2.adoc");
const FRAME_SHAPE3_1: &str = include_str!("frames/shape_3_1.adoc");
const FRAME_SHAPE3_2: &str = include_str!("frames/shape_3_2.adoc");
const FRAME_SHAPE7_1: &str = include_str!("frames/shape_7_1.adoc");
const FRAME_SHAPE7_2: &str = include_str!("frames/shape_7_2.adoc");
const FRAME_SHAPE8_1: &str = include_str!("frames/shape_8_1.adoc");
const FRAME_SHAPE8_2: &str = include_str!("frames/shape_8_2.adoc");

pub fn get_object_gameboy() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_GAMEBOY));
    let sprite = Sprite::new(anim, false);
    Object::new(50, String::from("Gameboy"), vec![sprite], None)
}

pub fn get_object_shape(id: usize, speed: usize) -> ObjectRef {
    let frames = match id {
        1 => vec![Frame::new(FRAME_SHAPE1_1), Frame::new(FRAME_SHAPE1_2)],
        2 => vec![Frame::new(FRAME_SHAPE2_1), Frame::new(FRAME_SHAPE2_2)],
        3 => vec![Frame::new(FRAME_SHAPE3_1), Frame::new(FRAME_SHAPE3_2)],
        //4 => vec![Frame::new(FRAME_SHAPE4_1), Frame::new(FRAME_SHAPE4_2)],
        //5 => vec![Frame::new(FRAME_SHAPE5_1), Frame::new(FRAME_SHAPE5_2)],
        //6 => vec![Frame::new(FRAME_SHAPE6_1), Frame::new(FRAME_SHAPE6_2)],
        7 => vec![Frame::new(FRAME_SHAPE7_1), Frame::new(FRAME_SHAPE7_2)],
        8 => vec![Frame::new(FRAME_SHAPE8_1), Frame::new(FRAME_SHAPE8_2)],
        _ => panic!("Invalid shape id: {}", id),
    };
    let anim = SpriteAnimation::new_tick_based(frames, speed, true, None, true, true);
    let sprite = Sprite::new(anim, false);
    Object::new(67, String::from("Tetris bar"), vec![sprite], None)
}
