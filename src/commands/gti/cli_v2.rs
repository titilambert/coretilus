use crate::collision::Collision;
use crate::commands::gti::objects::{
    get_object_commit, get_object_pull, get_object_push, get_object_std, get_object_tag,
};
use crate::engine_v2::engine::Engine;
use crate::engine_v2::object::movement::Movement;
use crate::engine_v2::object::object::ObjectRef;
use crate::engine_v2::position::{Position, XTermPosition, YTermPosition};
use crate::tools::parse_args;

use crate::command::{Command, CommandV2};

pub struct Gti {}

impl CommandV2 for Gti {
    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef> {
        vec![
            get_object_std,
            get_object_commit,
            get_object_pull,
            get_object_push,
            get_object_tag,
        ]
    }

    fn select_objects(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>) {
        let short_flags: &[char] = &[];
        let long_flags: &[&str] = &[];
        let params = ["tag", "push", "pull", "commit"];
        let (_, dparams) = parse_args(args.collect(), short_flags, long_flags, &params);

        let mut is_loop = false;
        let mut object = get_object_std();
        let mut speed = 2;
        if dparams.iter().any(|s| s == "pull") {
            object = get_object_pull();
            speed = 5;
        } else if dparams.iter().any(|s| s == "push") {
            object = get_object_push();
            speed = 10;
        } else if dparams.iter().any(|s| s == "commit") {
            object = get_object_commit();
            is_loop = true;
        } else if dparams.iter().any(|s| s == "tag") {
            object = get_object_tag();
            is_loop = true;
        }

        let mut objects: Vec<ObjectRef> = Vec::new();
        if is_loop {
            let trajectory = Movement::new_stationary(
                Position::new(XTermPosition::Coord(2), YTermPosition::Coord(10), 0),
                200,
            );
            object.borrow_mut().set_movement(trajectory);
            objects.push(object);
        } else {
            let movement = Movement::new_linear(
                Position::new(XTermPosition::LeftOut, YTermPosition::Coord(10), 0),
                Position::new(XTermPosition::RightOut, YTermPosition::Coord(10), 0),
                speed,
            );
            object.borrow_mut().set_movement(movement);
            objects.push(object);
        }
        let collisions: Vec<Collision> = Vec::new();
        (objects, collisions)
    }
    fn execute(&mut self) {
        let (mut objects, _) = self.select_objects(std::env::args());
        let mut engine = Engine::new(&mut objects, 300);
        engine.run();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::gti::cli::Gti;

    #[test]
    fn test_select_sprite_std() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 2);
        assert_eq!(sprites[0].borrow_mut().tdid(), 9);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_push() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("push")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 8);
        assert_eq!(sprites[0].borrow_mut().tdid(), 11);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_pull() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("pull")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 5);
        assert_eq!(sprites[0].borrow_mut().tdid(), 10);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_commit() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("commit")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 0);
        assert_eq!(sprites[0].borrow_mut().tdid(), 13);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_tag() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("tag")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 0);
        assert_eq!(sprites[0].borrow_mut().tdid(), 12);
        assert_eq!(collisions.len(), 0);
    }
}
