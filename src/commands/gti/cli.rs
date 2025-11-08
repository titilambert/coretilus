use crate::collision::Collision;
use crate::commands::gti::sprite::{
    get_sprite_commit, get_sprite_pull, get_sprite_push, get_sprite_std, get_sprite_tag,
};
use crate::coord::{Position, XTermPosition, YTermPosition};
use crate::engine::RenderEngine;
use crate::sprite::SpriteRef;
use crate::tools::parse_args;
use crate::trajectory::Trajectory;

use crate::command::Command;

pub struct Gti {}

impl Command for Gti {
    fn get_all_sprites(&self) -> Vec<fn() -> SpriteRef> {
        vec![
            get_sprite_std,
            get_sprite_commit,
            get_sprite_pull,
            get_sprite_push,
            get_sprite_tag,
        ]
    }

    fn select_sprites(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<SpriteRef>, Vec<Collision>) {
        let short_flags: &[char] = &[];
        let long_flags: &[&str] = &[];
        let params = ["tag", "push", "pull", "commit"];
        let (_, dparams) = parse_args(args.collect(), short_flags, long_flags, &params);

        let mut is_loop = false;
        let mut sprite = get_sprite_std();
        let mut speed = 2;
        if dparams.iter().any(|s| s == "pull") {
            sprite = get_sprite_pull();
            speed = 5;
        } else if dparams.iter().any(|s| s == "push") {
            sprite = get_sprite_push();
            speed = 8;
        } else if dparams.iter().any(|s| s == "commit") {
            sprite = get_sprite_commit();
            is_loop = true;
        } else if dparams.iter().any(|s| s == "tag") {
            sprite = get_sprite_tag();
            is_loop = true;
        }

        let mut sprites: Vec<SpriteRef> = Vec::new();
        if is_loop {
            let movement = Trajectory::new_stationary(
                Position::new(XTermPosition::Coord(2), YTermPosition::Coord(10)),
                200,
            );
            sprite.borrow_mut().set_movement(movement);
            sprites.push(sprite);
        } else {
            let movement = Trajectory::new_linear(
                Position::new(XTermPosition::LeftOut, YTermPosition::Coord(10)),
                Position::new(XTermPosition::RightOut, YTermPosition::Coord(10)),
                speed,
            );
            sprite.borrow_mut().set_movement(movement);
            sprites.push(sprite);
        }
        let collisions: Vec<Collision> = Vec::new();
        (sprites, collisions)
    }
    fn execute(&mut self) {
        let mut engine = RenderEngine::new(0);
        let (mut sprites, mut collisions) = self.select_sprites(std::env::args());

        engine.render(&mut sprites, &mut collisions);
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

        assert_eq!(sprites[0].borrow_mut().movement().speed(), 2);
        assert_eq!(sprites[0].borrow_mut().tdid(), 9);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_push() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("push")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().movement().speed(), 8);
        assert_eq!(sprites[0].borrow_mut().tdid(), 11);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_pull() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("pull")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().movement().speed(), 5);
        assert_eq!(sprites[0].borrow_mut().tdid(), 10);
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_commit() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("commit")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().movement().speed(), 0);
        assert_eq!(sprites[0].borrow_mut().tdid(), 13);
        assert_eq!(collisions.len(), 0);
    }
    #[test]
    fn test_select_sprite_tag() {
        let mut gti = Gti {};
        let args: Vec<String> = vec![String::from("gti"), String::from("tag")];
        let (sprites, collisions) = gti.select_sprites(args.into_iter());

        assert_eq!(sprites[0].borrow_mut().movement().speed(), 0);
        assert_eq!(sprites[0].borrow_mut().tdid(), 12);
        assert_eq!(collisions.len(), 0);
    }
}
