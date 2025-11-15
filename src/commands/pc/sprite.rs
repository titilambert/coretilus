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
    let sprite = Sprite::new(50, String::from("Motherboard"), 1);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_chipset() -> SpriteRef {
    let frame = Frame::new(FRAME_CHIPSET);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, String::from("Chipset"), 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
pub fn get_sprite_ram() -> SpriteRef {
    let frame = Frame::new(FRAME_RAM);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, String::from("RAM"), 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

pub fn get_sprite_cachel2() -> SpriteRef {
    let frame = Frame::new(FRAME_CACHEL2);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, String::from("cachel2"), 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
pub fn get_sprite_cpu() -> SpriteRef {
    let frame = Frame::new(FRAME_CPU);
    let anim = Animation::new_static(frame);
    let sprite = Sprite::new(20, String::from("CPU"), 100);
    sprite.borrow_mut().set_animation(anim);
    sprite
}

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

fn build_frame_list(orientation: Orientation, repeat_count: usize) -> Vec<Frame> {
    match orientation {
        Orientation::Down => vec![Frame::new(FRAME_DATAD1), Frame::new(FRAME_DATAD2)]
            .into_iter()
            .chain(std::iter::repeat_n(Frame::new(FRAME_DATAD3), repeat_count))
            .chain(vec![Frame::new(FRAME_DATAD4), Frame::new(FRAME_DATAD5)])
            .collect::<Vec<Frame>>(),
        Orientation::Up => vec![Frame::new(FRAME_DATAU1), Frame::new(FRAME_DATAU2)]
            .into_iter()
            .chain(std::iter::repeat_n(Frame::new(FRAME_DATAU3), repeat_count))
            .chain(vec![Frame::new(FRAME_DATAU4), Frame::new(FRAME_DATAU5)])
            .collect::<Vec<Frame>>(),
        Orientation::Right => vec![Frame::new(FRAME_DATAR1), Frame::new(FRAME_DATAR2)]
            .into_iter()
            .chain(std::iter::repeat_n(Frame::new(FRAME_DATAR3), repeat_count))
            .chain(vec![Frame::new(FRAME_DATAR4), Frame::new(FRAME_DATAR5)])
            .collect::<Vec<Frame>>(),
        Orientation::Left => vec![Frame::new(FRAME_DATAL1), Frame::new(FRAME_DATAL2)]
            .into_iter()
            .chain(std::iter::repeat_n(Frame::new(FRAME_DATAL3), repeat_count))
            .chain(vec![Frame::new(FRAME_DATAL4), Frame::new(FRAME_DATAL5)])
            .collect::<Vec<Frame>>(),
    }
}

pub fn get_sprite_data(index: usize) -> SpriteRef {
    let frame_list: Vec<Vec<Frame>> = vec![
        // DATA1
        build_frame_list(Orientation::Down, 7),
        // DATA2
        build_frame_list(Orientation::Right, 29),
        // DATA3
        build_frame_list(Orientation::Down, 0),
        // DATA4
        build_frame_list(Orientation::Down, 7),
        // DATA5
        build_frame_list(Orientation::Up, 9),
        // DATA6
        build_frame_list(Orientation::Left, 15),
        // DATA7
        build_frame_list(Orientation::Down, 0),
        // DATA8
        vec![Frame::new(FRAME_DATAU2), Frame::new(FRAME_DATAU2)],
        // DATA9
        build_frame_list(Orientation::Right, 8),
        // DATA10
        build_frame_list(Orientation::Down, 9),
        // DATA11
        build_frame_list(Orientation::Up, 3),
        // DATA12
        build_frame_list(Orientation::Up, 0),
        // DATA13
        build_frame_list(Orientation::Left, 40),
        // DATA14
        build_frame_list(Orientation::Up, 8),
    ];
    if !frame_list.len() < index {
        panic!("No Data sprite with index {}", index);
    }

    let frames = frame_list[index - 1].clone();
    let anim = Animation::new_movement_based(frames, 0, false);
    let sprite = Sprite::new(
        14 + index as u64,
        format!("data{}", index),
        10 + index as i32,
    );
    sprite.borrow_mut().collider().set_active(true);
    sprite.borrow_mut().set_animation(anim);
    sprite
}
