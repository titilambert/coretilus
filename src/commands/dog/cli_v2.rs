use crate::command::CommandV2;
use crate::commands::dog::objects::get_object_dog;
use crate::commands::dog::objects::get_object_domain;
use crate::engine_v2::collision::Collision;
use crate::engine_v2::collision::ScreenEdge;
use crate::engine_v2::engine::Engine;
use crate::engine_v2::entity::movement::Movement;
use crate::engine_v2::entity::object::ObjectRef;
use crate::engine_v2::position::Position;
use crate::engine_v2::position::XTermPosition;
use crate::engine_v2::position::YTermPosition;
use crate::tools::filter_params_regex;

pub struct Dog {}

impl CommandV2 for Dog {
    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef> {
        vec![get_object_dog]
    }
    fn select_objects(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>) {
        let mut objects: Vec<ObjectRef> = Vec::new();
        let mut collisions: Vec<Collision> = Vec::new();
        // Regex to match domains
        let params = [r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$"];
        let dparams = filter_params_regex(args.collect(), &params);

        // domain object
        let mut domain_object_txt = "()".to_string();
        if !dparams.is_empty() {
            domain_object_txt = dparams[0].to_string();
        }
        let domain_object = get_object_domain(domain_object_txt);
        let movement = Movement::new_circular(
            Position::new(XTermPosition::LeftOut, YTermPosition::Coord(9), 0),
            Position::new(XTermPosition::RightOut, YTermPosition::Coord(9), 0),
            8,
            15,
        );
        domain_object.borrow_mut().set_movement(movement);
        objects.push(domain_object);

        // Dog object
        let dog_object = get_object_dog();
        let movement = Movement::new_linear(
            Position::new(XTermPosition::LeftOut, YTermPosition::Coord(3), 0),
            Position::new(XTermPosition::RightOut, YTermPosition::Coord(3), 0),
            12,
        );

        let collision = Collision::new_edge(
            dog_object.clone(),
            ScreenEdge::RightWithObjectLeftSide,
            move |_, _, engine| {
                engine.stop();
            },
        );

        dog_object.borrow_mut().set_movement(movement);
        objects.push(dog_object);
        collisions.push(collision);

        (objects, collisions)
    }
    fn execute(&mut self) {
        let (objects, collisions) = self.select_objects(std::env::args());
        let mut engine = Engine::new(objects, collisions, 0);
        engine.run();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::dog::cli_v2::Dog;
    use crate::engine_v2::entity::movement::Direction;

    #[test]
    fn test_select_sprite_std() {
        let mut dog = Dog {};
        let args: Vec<String> = vec![String::from("dog")];
        let (objects, collisions) = dog.select_objects(args.into_iter());

        assert_eq!(objects[1].borrow_mut().movement().speed(), 12);
        assert_eq!(objects[0].borrow_mut().movement().speed(), 8);
        assert_eq!(
            objects[1].borrow_mut().movement().direction(),
            Direction::Linear
        );
        assert_eq!(
            objects[0].borrow_mut().movement().direction(),
            Direction::Circular
        );
        assert_eq!(objects[0].borrow_mut().current_frame().content(), "()");
        assert_eq!(collisions.len(), 1);
    }

    #[test]
    fn test_select_sprite_with_domain() {
        let mut dog = Dog {};
        let args: Vec<String> = vec![String::from("dog"), String::from("debian.org")];
        let (objects, collisions) = dog.select_objects(args.into_iter());

        assert_eq!(objects[1].borrow_mut().movement().speed(), 12);
        assert_eq!(objects[0].borrow_mut().movement().speed(), 8);
        assert_eq!(
            objects[1].borrow_mut().movement().direction(),
            Direction::Linear
        );
        assert_eq!(
            objects[0].borrow_mut().movement().direction(),
            Direction::Circular
        );
        assert_eq!(
            objects[0].borrow_mut().current_frame().content(),
            "debian.org"
        );
        assert_eq!(collisions.len(), 1);
    }
}
