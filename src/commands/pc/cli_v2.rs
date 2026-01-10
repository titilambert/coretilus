use std::cell::RefCell;
use std::rc::Rc;

use crate::command::CommandV2;
use crate::commands::pc::objects::get_object_cachel2;
use crate::commands::pc::objects::get_object_chipset;
use crate::commands::pc::objects::get_object_cpu;
use crate::commands::pc::objects::get_object_data;
use crate::commands::pc::objects::get_object_motherboard;
use crate::commands::pc::objects::get_object_ram;
use crate::engine_v2::collision::Collision;
use crate::engine_v2::coords::Coords;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::movement::Movement;
use crate::engine_v2::entity::object::Object;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::position::Position;
use crate::engine_v2::position::XTermPosition;
use crate::engine_v2::position::YTermPosition;
use crate::tools::parse_args;

// Helper function to create a object with movement and visibility
fn create_object(
    mut object_list: Vec<Rc<RefCell<Object>>>,
    get_object_fn: fn() -> ObjectRef,
    start_position: Position,
    end_position: Option<Position>,
    speed: usize,
    visible: bool,
) -> ObjectRef {
    let object = get_object_fn();

    let mut movement = Movement::new_stationary(start_position, 0);
    if let Some(end_p) = end_position {
        movement = Movement::new_linear(start_position, end_p, speed as i32);
    }
    object.borrow_mut().set_movement(movement);
    object.borrow_mut().set_visible(visible);
    object_list.push(object.clone());
    object
}

pub struct Pc {}

impl CommandV2 for Pc {
    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef> {
        vec![get_object_motherboard]
    }

    fn select_objects(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>) {
        let short_flags: &[char] = &[];
        let long_flags: &[&str] = &[];
        let params: &[&str] = &[];
        let (_, _) = parse_args(args.collect(), short_flags, long_flags, params);
        let mut collisions: Vec<Collision> = Vec::new();
        let mut objects: Vec<ObjectRef> = Vec::new();
        let speed: usize = 17;

        // Motherboard
        let motherboard_object = create_object(
            objects.clone(),
            get_object_motherboard,
            Position::new(XTermPosition::Coord(2), YTermPosition::Coord(1), 100),
            None,
            0,
            true,
        );
        // Chipset
        let chipsed_object = create_object(
            objects.clone(),
            get_object_chipset,
            Position::new(XTermPosition::Coord(62), YTermPosition::Coord(16), 100),
            None,
            0,
            true,
        );
        // RAM
        let ram_object = create_object(
            objects.clone(),
            get_object_ram,
            Position::new(XTermPosition::Coord(49), YTermPosition::Coord(10), 100),
            None,
            0,
            true,
        );
        // Cachel2
        let cachel2_object = get_object_cachel2();
        let movement = Movement::new_stationary(
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(22), 100),
            0,
        );
        cachel2_object.borrow_mut().set_movement(movement);
        // CPU
        let cpu_object = get_object_cpu();
        let movement = Movement::new_stationary(
            Position::new(XTermPosition::Coord(58), YTermPosition::Coord(23), 100),
            0,
        );
        cpu_object.borrow_mut().set_movement(movement);
        // DATA1
        let data1_object = get_object_data(1);
        let movement = Movement::new_linear(
            Position::new(XTermPosition::Coord(36), YTermPosition::Coord(30), 11),
            Position::new(XTermPosition::Coord(36), YTermPosition::Coord(20), 11),
            speed as i32,
        );
        data1_object.borrow_mut().set_movement(movement);
        // DATA2
        let data2_object = get_object_data(2);
        data2_object.borrow_mut().set_visible(false);
        let mut movement2 = Movement::new_linear(
            Position::new(XTermPosition::Coord(34), YTermPosition::Coord(22), 12),
            Position::new(XTermPosition::Coord(66), YTermPosition::Coord(22), 12),
            speed as i32,
        );
        movement2.deactivate();
        data2_object.borrow_mut().set_movement(movement2);
        // DATA3
        let data3_object = get_object_data(3);
        data3_object.borrow_mut().set_visible(false);
        let mut movement3 = Movement::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(22), 13),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(19), 13),
            speed as i32,
        );
        movement3.deactivate();
        data3_object.borrow_mut().set_movement(movement3);
        // DATA4
        let data4_object = get_object_data(4);
        let data4_object_id = data4_object.borrow().id();
        data4_object.borrow_mut().set_visible(false);
        let mut movement4 = Movement::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(15), 14),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(10), 14),
            speed as i32,
        );
        movement4.deactivate();
        data4_object.borrow_mut().set_movement(movement4);
        // DATA5
        let data5_object = get_object_data(5);
        let data5_object_id = data5_object.borrow().id();
        data5_object.borrow_mut().set_visible(false);
        let mut movement5 = Movement::new_linear(
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(10), 15),
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(21), 15),
            speed as i32,
        );
        movement5.deactivate();
        data5_object.borrow_mut().set_movement(movement5);
        // DATA6
        let data6_object = get_object_data(6);
        let data6_object_id = data6_object.borrow().id();
        data6_object.borrow_mut().set_visible(false);
        let mut movement6 = Movement::new_linear(
            Position::new(XTermPosition::Coord(76), YTermPosition::Coord(32), 16),
            Position::new(XTermPosition::Coord(58), YTermPosition::Coord(32), 16),
            speed as i32,
        );
        movement6.deactivate();
        data6_object.borrow_mut().set_movement(movement6);
        // DATA7
        let data7_object = get_object_data(7);
        data7_object.borrow_mut().set_visible(false);
        let mut movement7 = Movement::new_linear(
            Position::new(XTermPosition::Coord(61), YTermPosition::Coord(32), 17),
            Position::new(XTermPosition::Coord(61), YTermPosition::Coord(28), 17),
            speed as i32,
        );
        movement7.deactivate();
        data7_object.borrow_mut().set_movement(movement7);
        // DATA8
        let data8_object = get_object_data(8);
        let data8_object_id = data8_object.borrow().id();
        data8_object.borrow_mut().set_visible(false);
        let mut movement8 = Movement::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(28), 18),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(32), 18),
            speed as i32,
        );
        movement8.deactivate();
        data8_object.borrow_mut().set_movement(movement8);
        // DATA9
        let data9_object = get_object_data(9);
        data9_object.borrow_mut().set_visible(false);
        let mut movement9 = Movement::new_linear(
            Position::new(XTermPosition::Coord(66), YTermPosition::Coord(32), 19),
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(32), 19),
            speed as i32,
        );
        movement9.deactivate();
        data9_object.borrow_mut().set_movement(movement9);
        // DATA10
        let data10_object = get_object_data(10);
        let data10_object_id = data10_object.borrow().id();
        data10_object.borrow_mut().set_visible(false);
        let mut movement10 = Movement::new_linear(
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(21), 20),
            Position::new(XTermPosition::Coord(78), YTermPosition::Coord(10), 20),
            speed as i32,
        );
        movement10.deactivate();
        data10_object.borrow_mut().set_movement(movement10);
        // DATA11
        let data11_object = get_object_data(11);
        let data11_object_id = data11_object.borrow().id();
        data11_object.borrow_mut().set_visible(false);
        let mut movement11 = Movement::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(10), 21),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(16), 21),
            speed as i32,
        );
        movement11.deactivate();
        data11_object.borrow_mut().set_movement(movement11);
        // DATA12
        let data12_object = get_object_data(12);
        let data12_object_id = data12_object.borrow().id();
        data12_object.borrow_mut().set_visible(false);
        let mut movement12 = Movement::new_linear(
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(19), 22),
            Position::new(XTermPosition::Coord(68), YTermPosition::Coord(22), 22),
            speed as i32,
        );
        movement12.deactivate();
        data12_object.borrow_mut().set_movement(movement12);
        // DATA13
        let data13_object = get_object_data(13);
        let data13_object_id = data13_object.borrow().id();
        data13_object.borrow_mut().set_visible(false);
        let mut movement13 = Movement::new_linear(
            Position::new(XTermPosition::Coord(65), YTermPosition::Coord(22), 23),
            Position::new(XTermPosition::Coord(20), YTermPosition::Coord(22), 23),
            speed as i32,
        );
        movement13.deactivate();
        data13_object.borrow_mut().set_movement(movement13);

        // DATA14
        let data14_object = get_object_data(14);
        data14_object.borrow_mut().set_visible(false);
        let mut movement14 = Movement::new_linear(
            Position::new(XTermPosition::Coord(23), YTermPosition::Coord(20), 24),
            Position::new(XTermPosition::Coord(23), YTermPosition::Coord(31), 24),
            speed as i32,
        );
        movement14.deactivate();
        data14_object.borrow_mut().set_movement(movement14);

        // COLLISIONS
        // Collision data1/data2
        let collision1 = Collision::new_object(
            data1_object.clone(),
            data2_object.clone(),
            move |data1_objectc, data2_objectc, counter, _| {
                data2_objectc.borrow_mut().set_visible(true);
                data2_objectc.borrow_mut().activate_movement();
                if counter == 2 * speed {
                    data1_objectc.borrow_mut().set_visible(false);
                }
            },
        );

        collisions.push(collision1);

        // Collision data2/data3
        let collision2 = Collision::new_object(
            data2_object.clone(),
            data3_object.clone(),
            move |data2_objectc, data3_objectc, counter, _| {
                if counter == 2 * speed {
                    data3_objectc.borrow_mut().activate_movement();
                    data3_objectc.borrow_mut().set_visible(true);
                }
                if counter == 4 * speed {
                    data2_objectc.borrow_mut().set_visible(false);
                }
            },
        );
        collisions.push(collision2);

        // Collision data3/chipset
        let collision3 = Collision::new_object(
            data3_object.clone(),
            chipsed_object.clone(),
            move |data3_objectc, _, counter, engine| {
                if counter == 3 * speed {
                    data3_objectc.borrow_mut().set_visible(false);
                }
                if counter == speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data4_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
            },
        );
        collisions.push(collision3);

        // Collision data4/ram
        let collision4 = Collision::new_object(
            data4_object.clone(),
            ram_object.clone(),
            move |data4_objectc, _, counter, engine| {
                if counter == 2 * speed {
                    data4_objectc.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data5_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
            },
        );
        collisions.push(collision4);

        // Collision data5/cachel2
        let collision5 = Collision::new_object(
            data5_object.clone(),
            cachel2_object.clone(),
            move |data5_objectc, _, counter, engine| {
                if counter == 3 * speed {
                    data5_objectc.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data6_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
            },
        );
        collisions.push(collision5);
        // Collision data6/data7
        let collision6 = Collision::new_object(
            data6_object.clone(),
            data7_object.clone(),
            move |data6_objectc, data7_objectc, counter, engine| {
                if counter == 2 * speed {
                    data7_objectc.borrow_mut().set_visible(true);
                    data7_objectc.borrow_mut().activate_movement();
                }
                if counter == 4 * speed {
                    data6_objectc.borrow_mut().set_visible(false);
                }
                if counter == 4 * speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data8_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
            },
        );
        collisions.push(collision6);
        // Collision data8/data9
        let collision8 = Collision::new_object(
            data8_object.clone(),
            data9_object.clone(),
            move |data8_objectc, data9_objectc, counter, _| {
                if counter == speed {
                    data9_objectc.borrow_mut().set_visible(true);
                    data9_objectc.borrow_mut().activate_movement();
                }
                if counter == 2 * speed {
                    data8_objectc.borrow_mut().set_visible(false);
                }
            },
        );
        collisions.push(collision8);
        // Collision data9/cachel2
        let collision9 = Collision::new_object(
            data9_object.clone(),
            cachel2_object.clone(),
            move |data9_objectc, _, counter, engine| {
                if counter == 3 * speed {
                    data9_objectc.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data10_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
            },
        );
        collisions.push(collision9);
        // Collision data10/ram
        let collision10 = Collision::new_object(
            data10_object.clone(),
            ram_object.clone(),
            move |data10_objectc, _, counter, engine| {
                if counter == 2 * speed {
                    data10_objectc.borrow_mut().set_visible(false);
                }
                if counter == 2 * speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data11_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
            },
        );
        collisions.push(collision10);
        // Collision data11/chipset
        let collision11 = Collision::new_object(
            data11_object.clone(),
            chipsed_object.clone(),
            move |data11_objectc, _, counter, engine| {
                if counter == 3 * speed {
                    data11_objectc.borrow_mut().set_visible(false);
                }
                if counter == speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data12_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
                if counter == 2 * speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data13_object_id {
                            object.borrow_mut().set_visible(true);
                            object.borrow_mut().activate_movement();
                        }
                    }
                }
                if counter == 4 * speed {
                    for object in engine.objects_mut() {
                        if object.borrow().id() == data12_object_id {
                            object.borrow_mut().set_visible(false);
                        }
                    }
                }
            },
        );
        collisions.push(collision11);
        // Collision data13/data14
        let collision13 = Collision::new_object(
            data13_object.clone(),
            data14_object.clone(),
            move |data13_objectc, data14_objectc, counter, _| {
                if counter == speed {
                    data14_objectc.borrow_mut().set_visible(true);
                    data14_objectc.borrow_mut().activate_movement();
                }
                if counter == 3 * speed {
                    data13_objectc.borrow_mut().set_visible(false);
                }
            },
        );
        collisions.push(collision13);
        // Collision end
        let end_coords = Coords::new(23, 31, 24);
        let collision14 = Collision::new_point(
            data14_object.clone(),
            end_coords,
            move |data14_objectc, _, counter, engine| {
                if counter == 6 * speed {
                    data14_objectc.borrow_mut().set_visible(false);
                }
                if counter == 12 * speed {
                    engine.stop();
                }
            },
        );
        collisions.push(collision14);

        objects.push(motherboard_object);
        objects.push(chipsed_object);
        objects.push(ram_object);
        objects.push(cachel2_object);
        objects.push(cpu_object);
        objects.push(data1_object);
        objects.push(data2_object);
        objects.push(data3_object);
        objects.push(data4_object);
        objects.push(data5_object);
        objects.push(data6_object);
        objects.push(data7_object);
        objects.push(data8_object);
        objects.push(data9_object);
        objects.push(data10_object);
        objects.push(data11_object);
        objects.push(data12_object);
        objects.push(data13_object);
        objects.push(data14_object);

        (objects, collisions)
    }
    fn execute(&mut self) {
        let (objects, collisions) = self.select_objects(std::env::args());
        let mut engine = Engine::new(objects, collisions, 0);
        engine.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::pc::cli_v2::Pc;

    #[test]
    fn test_select_data1() {
        let mut pc = Pc {};
        let args: Vec<String> = vec![String::from("pc")];
        let (objects, collisions) = pc.select_objects(args.into_iter());

        assert_eq!(objects.len(), 19);
        assert_eq!(objects[0].borrow_mut().movement().speed(), 0);
        assert_eq!(objects[5].borrow_mut().movement().speed(), 17);
        assert_eq!(collisions.len(), 12);
    }
}
