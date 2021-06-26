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
    let humans = &world.humans;
    let zombies = &mut world.zombies;
    for zombie in zombies.iter_mut() {
        let mut target = world.pos;
        let mut target_distance = target.distance_to(zombie.pos);

        for human in humans.iter() {
            let distance = zombie.pos.distance_to(human.pos);
            if distance < target_distance {
                target_distance = distance;
                target = zombie.pos;
            }
        }

        zombie.next = target;
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