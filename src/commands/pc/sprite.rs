use crate::animation::Animation;
use crate::frame::Frame;
use crate::sprite::Sprite;
use crate::sprite::SpriteRef;

const FRAME_MOTHERBOARD: &str = include_str!("./frames/motherboard.adoc");
const FRAME_CHIPSET: &str = include_str!("./frames/chipset.adoc");
const FRAME_RAM: &str = include_str!("./frames/ram.adoc");
const FRAME_CACHEL2: &str = include_str!("./frames/cachel2.adoc");
const FRAME_CPU: &str = include_str!("./frames/cpu.adoc");
const FRAME_DATAD1: &str = include_str!("./frames/datad1.adoc");
const FRAME_DATAD2: &str = include_str!("./frames/datad2.adoc");
const FRAME_DATAD3: &str = include_str!("./frames/datad3.adoc");
const FRAME_DATAD4: &str = include_str!("./frames/datad4.adoc");
const FRAME_DATAD5: &str = include_str!("./frames/datad5.adoc");
const FRAME_DATAR1: &str = include_str!("./frames/datar1.adoc");
const FRAME_DATAR2: &str = include_str!("./frames/datar2.adoc");
const FRAME_DATAR3: &str = include_str!("./frames/datar3.adoc");
const FRAME_DATAR4: &str = include_str!("./frames/datar4.adoc");
const FRAME_DATAR5: &str = include_str!("./frames/datar5.adoc");
const FRAME_DATAU1: &str = include_str!("./frames/datau1.adoc");
const FRAME_DATAU2: &str = include_str!("./frames/datau2.adoc");
const FRAME_DATAU3: &str = include_str!("./frames/datau3.adoc");
const FRAME_DATAU4: &str = include_str!("./frames/datau4.adoc");
const FRAME_DATAU5: &str = include_str!("./frames/datau5.adoc");
const FRAME_DATAL1: &str = include_str!("./frames/datal1.adoc");
const FRAME_DATAL2: &str = include_str!("./frames/datal2.adoc");
const FRAME_DATAL3: &str = include_str!("./frames/datal3.adoc");
const FRAME_DATAL4: &str = include_str!("./frames/datal4.adoc");
const FRAME_DATAL5: &str = include_str!("./frames/datal5.adoc");

pub fn get_sprite_motherboard() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_MOTHERBOARD)];
    let anim = Animation::new_tick_based(frames, 0, 1, 50, true);
    let sprite = Sprite::new(50, "Motherboard", 1);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_chipset() -> SpriteRef {
    let frame = Frame::new(FRAME_CHIPSET);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, "Chipset", 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
pub fn get_sprite_ram() -> SpriteRef {
    let frame = Frame::new(FRAME_RAM);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, "Ram", 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_cachel2() -> SpriteRef {
    let frame = Frame::new(FRAME_CACHEL2);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, "Ram", 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
pub fn get_sprite_cpu() -> SpriteRef {
    let frame = Frame::new(FRAME_CPU);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, "Ram", 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
pub fn get_sprite_data1() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAD1), Frame::new(FRAME_DATAD2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAD3)).take(7));
    frames.extend([Frame::new(FRAME_DATAD4), Frame::new(FRAME_DATAD5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data1", 11);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data2() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAR1), Frame::new(FRAME_DATAR2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAR3)).take(29));
    frames.extend([Frame::new(FRAME_DATAR4), Frame::new(FRAME_DATAR5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data2", 12);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data3() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAD1), Frame::new(FRAME_DATAD2)];
    frames.extend([Frame::new(FRAME_DATAD4), Frame::new(FRAME_DATAD5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data3", 13);

    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data4() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAD1), Frame::new(FRAME_DATAD2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAD3)).take(7));
    frames.extend([Frame::new(FRAME_DATAD4), Frame::new(FRAME_DATAD5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data4", 14);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data5() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAU1), Frame::new(FRAME_DATAU2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAU3)).take(9));
    frames.extend([Frame::new(FRAME_DATAU4), Frame::new(FRAME_DATAU5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data5", 15);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data6() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAL1), Frame::new(FRAME_DATAL2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAL3)).take(15));
    frames.extend([Frame::new(FRAME_DATAL4), Frame::new(FRAME_DATAL5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(16, "data6", 16);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data7() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAD1), Frame::new(FRAME_DATAD2)];
    frames.extend([Frame::new(FRAME_DATAD4), Frame::new(FRAME_DATAD5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data3", 17);

    sprite.borrow_mut().collider().set_active(true);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data8() -> SpriteRef {
    let frames = vec![Frame::new(FRAME_DATAU2), Frame::new(FRAME_DATAU2)];

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data3", 18);

    sprite.borrow_mut().collider().set_active(true);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data9() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAR1), Frame::new(FRAME_DATAR2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAR3)).take(8));
    frames.extend([Frame::new(FRAME_DATAR4), Frame::new(FRAME_DATAR5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data9", 19);
    sprite.borrow_mut().collider().set_active(true);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data10() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAD1), Frame::new(FRAME_DATAD2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAD3)).take(9));
    frames.extend([Frame::new(FRAME_DATAD4), Frame::new(FRAME_DATAD5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data10", 20);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data11() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAU1), Frame::new(FRAME_DATAU2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAU3)).take(3));
    frames.extend([Frame::new(FRAME_DATAU4), Frame::new(FRAME_DATAU5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data11", 21);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data12() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAU1), Frame::new(FRAME_DATAU2)];
    frames.extend([Frame::new(FRAME_DATAU4), Frame::new(FRAME_DATAU5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data12", 22);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data13() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAL1), Frame::new(FRAME_DATAL2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAL3)).take(40));
    frames.extend([Frame::new(FRAME_DATAL4), Frame::new(FRAME_DATAL5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(16, "data13", 23);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_data14() -> SpriteRef {
    let mut frames = vec![Frame::new(FRAME_DATAU1), Frame::new(FRAME_DATAU2)];
    frames.extend(std::iter::repeat(Frame::new(FRAME_DATAU3)).take(8));
    frames.extend([Frame::new(FRAME_DATAU4), Frame::new(FRAME_DATAU5)]);

    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(14, "data14", 24);
    sprite.borrow_mut().collider().set_active(true);
    //sprite.borrow_mut().set_collider(collider);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
