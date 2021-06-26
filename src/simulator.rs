use super::collections;
pub use super::model::*;

pub fn next(world: &mut World, action: &Action) {
    move_zombies(world);
    move_ash(world, &action);
    destroy_zombies(world);
    destroy_humans(world);
    update_zombie_targets(world);
}

fn move_zombies(world: &mut World) {
    for zombie in world.zombies.iter_mut() {
        zombie.pos = zombie.next;
    }
}

fn update_zombie_targets(world: &mut World) {
    for zombie in world.zombies.iter_mut() {
        // TODO: Ash is a human too
        let closest_human = collections::min_by_fkey(&world.humans, |human| human.pos.distance_to_squared(zombie.pos));
        match closest_human {
            Some(human) => {
                zombie.next = zombie.pos.towards(human.pos, constants::MAX_ZOMBIE_STEP);
            },
            None => (),
        }
    }
}

fn move_ash(world: &mut World, action: &Action) {
    world.pos = world.pos.towards(action.target, constants::MAX_ASH_STEP);
}

fn destroy_zombies(world: &mut World) {
    let max_distance_squared = constants::MAX_ASH_KILL_RANGE.powf(2.0);
    let ash_pos = world.pos;
    world.zombies.retain(|zombie| zombie.pos.distance_to_squared(ash_pos) > max_distance_squared);
}

fn destroy_humans(world: &mut World) {
    let max_distance_squared = constants::MAX_ZOMBIE_KILL_RANGE.powf(2.0);
    let zombies = &world.zombies;
    let humans = &mut world.humans;
    humans.retain(|human| zombies.iter().any(|zombie| zombie.pos.distance_to_squared(human.pos) <= max_distance_squared));
}