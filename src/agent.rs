use super::collections;
pub use super::model::*;

pub fn choose(world: &World) -> Action {
    Action {
        target: choose_target(world),
    }
}

fn choose_target(world: &World) -> V2 {
    let closest_zombie = collections::min_by_fkey(&world.zombies, |zombie| zombie.next.distance_to_squared(world.pos));
    match closest_zombie {
        Some(zombie) => zombie.next,
        None => V2::zero(),
    }
}