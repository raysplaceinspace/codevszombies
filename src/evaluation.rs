pub use super::model::*;

pub fn evaluate(world: &World) -> f32 {
    // TODO: improve evaluation function
    let num_humans = world.humans.len() as f32;
    let num_zombies = world.zombies.len() as f32;
    100.0 * num_humans - 1.0 * num_zombies
}