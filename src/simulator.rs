pub use super::model::*;
use std::collections::HashSet;

pub fn next(world: &mut World, action: &Action) -> Vec<Event> {
    world.tick += 1;

    let mut events = Vec::<Event>::new();
    move_zombies(world);
    move_ash(world, &action);
    destroy_zombies(world, &mut events);
    destroy_humans(world, &mut events);
    update_zombie_targets(world);
    events
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
        let mut target_distance = zombie.pos.distance_to(target);

        for human in humans.iter() {
            let distance = zombie.pos.distance_to(human.pos);
            if distance < target_distance {
                target_distance = distance;
                target = human.pos;
            }
        }

        zombie.next = zombie.pos.towards(target, constants::MAX_ZOMBIE_STEP);
    }
}

fn move_ash(world: &mut World, action: &Action) {
    world.pos = world.pos.towards(action.target, constants::MAX_ASH_STEP);
}

fn destroy_zombies(world: &mut World, events: &mut Vec<Event>) {
    let max_distance_squared = constants::MAX_ASH_KILL_RANGE.powf(2.0);
    let mut zombie_ids_to_delete = HashSet::<i32>::new();
    for zombie in world.zombies.iter() {
        if zombie.pos.distance_to_squared(world.pos) <= max_distance_squared {
            zombie_ids_to_delete.insert(zombie.id);

            events.push(Event::ZombieKilled {
                tick: world.tick,
                zombie_id: zombie.id,
                score: calculate_zombie_kill_score(world.humans.len() as i32, zombie_ids_to_delete.len() as i32),
            });
        }
    }

    world.zombies.retain(|zombie| !zombie_ids_to_delete.contains(&zombie.id));
}

fn calculate_zombie_kill_score(num_humans: i32, num_zombie_kills_already: i32) -> f32 {
    10.0 * (num_humans as f32).powf(2.0) * (1.0 + num_zombie_kills_already as f32) // TODO: Fibonacci sequence multiplier
}

fn destroy_humans(world: &mut World, events: &mut Vec<Event>) {
    if world.zombies.len() == 0 { return; } // Nothing to kill the humans
    if world.humans.len() == 0 { return; } // Game was already over before this turn started

    let max_distance_squared = constants::MAX_ZOMBIE_KILL_RANGE.powf(2.0);
    let mut human_ids_to_delete = HashSet::<i32>::new();
    for human in world.humans.iter() {
        let close_zombie = world.zombies.iter().any(|zombie| zombie.pos.distance_to_squared(human.pos) <= max_distance_squared);
        if close_zombie {
            human_ids_to_delete.insert(human.id);
            events.push(Event::HumanKilled { tick: world.tick, human_id: human.id });
        }
    }

    world.humans.retain(|human| !human_ids_to_delete.contains(&human.id));

    if world.humans.len() == 0 {
        events.push(Event::Ending { tick: world.tick, won: false })
    } else if world.zombies.len() == 0 {
        events.push(Event::Ending { tick: world.tick, won: true })
    }
}