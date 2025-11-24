use crate::{collision::Collision, engine_v2::object::object::ObjectRef, sprite::SpriteRef};
use std::any::type_name;

#[cfg(unix)]
use crate::signal;

pub trait Command {
    // Default implementation
    fn name(&self) -> String {
        let full = type_name::<Self>();
        full.split("::").last().unwrap_or(full).to_lowercase()
    }
    fn run(&mut self) {
        #[cfg(unix)]
        signal::ignore_sigint();

        self.execute();
    }

    fn execute(&mut self);

    fn get_all_sprites(&self) -> Vec<fn() -> SpriteRef>;

    fn select_sprites(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<SpriteRef>, Vec<Collision>);
}

pub trait CommandV2 {
    // Default implementation
    fn name(&self) -> String {
        let full = type_name::<Self>();
        full.split("::").last().unwrap_or(full).to_lowercase()
    }
    fn run(&mut self) {
        #[cfg(unix)]
        signal::ignore_sigint();

        self.execute();
    }

    fn execute(&mut self);

    fn get_all_objects(&self) -> Vec<fn() -> ObjectRef>;

    fn select_objects(
        &mut self,
        args: impl Iterator<Item = String>,
    ) -> (Vec<ObjectRef>, Vec<Collision>);
}
