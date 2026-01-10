use crate::engine_v2::collision::Collision;
use crate::engine_v2::entity::object::ObjectRef;
use std::any::type_name;

#[cfg(unix)]
use crate::signal;

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
