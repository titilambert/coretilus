use crate::engine_v2::collision::Collider;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::entity::frame::Frame;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::entity::sprite::Sprite;
use crate::engine_v2::entity::sprite_animation::SpriteAnimation;
use crate::engine_v2::size::Size;

const FRAME_GAMEBOY: &str = include_str!("frames/gb.adoc");
const FRAME_SHAPE1: &str = include_str!("frames/shape_1.adoc");
const FRAME_SHAPE2: &str = include_str!("frames/shape_2.adoc");
const FRAME_SHAPE3: &str = include_str!("frames/shape_3.adoc");
const FRAME_SHAPE4: &str = include_str!("frames/shape_4.adoc");
const FRAME_SHAPE5: &str = include_str!("frames/shape_5.adoc");
const FRAME_SHAPE6: &str = include_str!("frames/shape_6.adoc");
const FRAME_SHAPE7: &str = include_str!("frames/shape_7.adoc");

pub fn get_object_gameboy() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_GAMEBOY));
    let sprite = Sprite::new(anim);
    Object::new(50, String::from("Gameboy"), vec![sprite], None)
    //let sprite = SpriteV1::new(50, String::from("Gameboy"), 100);
}

pub fn get_object_shape1() -> ObjectRef {
    //let anim = SpriteAnimation::new_tick_based(frames, 0, 1, 50, true);
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SHAPE1));
    let sprite = Sprite::new(anim);
    Object::new(61, String::from("Tetris square"), vec![sprite], None)
}

pub fn get_object_shape2() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SHAPE2));
    let sprite = Sprite::new(anim);
    Object::new(62, String::from("Tetris square"), vec![sprite], None)
    //let sprite = SpriteV1::new(62, String::from("Tetris square"), 10);
    //sprite.borrow_mut().collider().set_active(false);
}

pub fn get_object_shape3() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SHAPE3));
    let sprite = Sprite::new(anim);
    Object::new(63, String::from("Tetris square"), vec![sprite], None)
}

pub fn get_object_shape4() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SHAPE4));
    let sprite = Sprite::new(anim);
    Object::new(64, String::from("Tetris square"), vec![sprite], None)
}

pub fn get_object_shape5() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SHAPE5));
    let sprite = Sprite::new(anim);
    Object::new(65, String::from("Tetris square"), vec![sprite], None)
}

pub fn get_object_shape6() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SHAPE6));
    let sprite = Sprite::new(anim);
    Object::new(66, String::from("Tetris square"), vec![sprite], None)
}

pub fn get_object_shape7() -> ObjectRef {
    let anim = SpriteAnimation::new_static(Frame::new(FRAME_SHAPE7));
    let sprite = Sprite::new(anim);
    Object::new(67, String::from("Tetris square"), vec![sprite], None)
}

pub fn get_object_bottom() -> ObjectRef {
    let frame = Frame::new("f".repeat(29));
    let anim = SpriteAnimation::new_static(frame);
    let sprite = Sprite::new(anim);
    let collider = Collider::new(Coords::new(0, 1, 0), Size::new(40, 1), true);
    Object::new(68, String::from("BG bottom"), vec![sprite], Some(collider))
}
