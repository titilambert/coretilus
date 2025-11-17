use crate::collision::Collision;
use crate::commands::dog::sprite::{get_sprite_dog, get_sprite_domain};
use crate::coord::{Position, XTermPosition, YTermPosition};
use crate::engine::RenderEngine;
use crate::sprite::SpriteRef;
use crate::tools::filter_params_regex;
use crate::trajectory::Trajectory;

use crate::command::Command;

pub struct Dog {}

impl Command for Dog {
    fn get_all_sprites(&self) -> Vec<fn() -> SpriteRef> {
        vec![get_sprite_dog]
    }
    fn select_sprites(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<SpriteRef>, Vec<Collision>) {
        let mut sprites: Vec<SpriteRef> = Vec::new();
        let collisions: Vec<Collision> = Vec::new();
        // Regex to match domains
        let params = [r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$"];
        let dparams = filter_params_regex(args.collect(), &params);

        // domain sprite
        let mut domain_sprite_txt = "()".to_string();
        if !dparams.is_empty() {
            domain_sprite_txt = dparams[0].to_string();
        }
        let domain_sprite = get_sprite_domain(domain_sprite_txt);
        let trajectory = Trajectory::new_circular(
            Position::new(XTermPosition::LeftOut, YTermPosition::Coord(9)),
            Position::new(XTermPosition::RightOut, YTermPosition::Coord(9)),
            8,
            15,
        );
        domain_sprite.borrow_mut().set_trajectory(trajectory);
        sprites.push(domain_sprite);

        // Dog sprite
        let dog_sprite = get_sprite_dog();
        let trajectory = Trajectory::new_linear(
            Position::new(XTermPosition::LeftOut, YTermPosition::Coord(3)),
            Position::new(XTermPosition::RightOut, YTermPosition::Coord(3)),
            12,
        );
        dog_sprite.borrow_mut().set_trajectory(trajectory);
        sprites.push(dog_sprite);

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
    use crate::{commands::dog::cli::Dog, trajectory::Direction};

    #[test]
    fn test_select_sprite_std() {
        let mut dog = Dog {};
        let args: Vec<String> = vec![String::from("dog")];
        let (sprites, collisions) = dog.select_sprites(args.into_iter());

        assert_eq!(sprites[1].borrow_mut().trajectory().speed(), 12);
        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 8);
        assert_eq!(
            sprites[1].borrow_mut().trajectory().direction(),
            Direction::Linear
        );
        assert_eq!(
            sprites[0].borrow_mut().trajectory().direction(),
            Direction::Circular
        );
        assert_eq!(sprites[0].borrow_mut().current_frame().content(), "()");
        assert_eq!(collisions.len(), 0);
    }

    #[test]
    fn test_select_sprite_with_domain() {
        let mut dog = Dog {};
        let args: Vec<String> = vec![String::from("dog"), String::from("debian.org")];
        let (sprites, collisions) = dog.select_sprites(args.into_iter());

        assert_eq!(sprites[1].borrow_mut().trajectory().speed(), 12);
        assert_eq!(sprites[0].borrow_mut().trajectory().speed(), 8);
        assert_eq!(
            sprites[1].borrow_mut().trajectory().direction(),
            Direction::Linear
        );
        assert_eq!(
            sprites[0].borrow_mut().trajectory().direction(),
            Direction::Circular
        );
        assert_eq!(
            sprites[0].borrow_mut().current_frame().content(),
            "debian.org"
        );
        assert_eq!(collisions.len(), 0);
    }
}
