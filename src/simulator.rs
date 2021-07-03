pub use super::model::*;
use std::collections::HashSet;

pub struct FibonacciSequence {
    previous: (i32, i32),
}

impl FibonacciSequence {
    pub fn new() -> FibonacciSequence {
        FibonacciSequence { previous: (0, 1) }
    }
    pub fn next(&mut self) -> i32 {
        let (a, b) = self.previous;
        let result = a + b;
        self.previous = (b, result);
        result
    }
}

pub fn next(world: &mut World, action: &Action) -> Vec<Event> {
    world.tick += 1;

    let mut events = Vec::<Event>::new();
    if !is_over(world) {
        move_zombies(world);
        move_ash(world, &action);
        destroy_zombies(world, &mut events);
        destroy_humans(world, &mut events);
        update_zombie_targets(world);

        emit_ending(world, &mut events);
    }
    events
}

fn move_zombies(world: &mut World) {
    for zombie in world.zombies.values_mut() {
        zombie.pos = zombie.next;
    }
}

fn update_zombie_targets(world: &mut World) {
    let humans = &world.humans;
    let zombies = &mut world.zombies;
    for zombie in zombies.values_mut() {
        let mut target = world.ash.pos;
        let mut target_distance = zombie.pos.distance_to(target);

        for human in humans.values() {
            let distance = zombie.pos.distance_to(human.pos);
            if distance < target_distance {
                target_distance = distance;
                target = human.pos;
            }
        }

        zombie.next = zombie.pos.towards(target, constants::MAX_ZOMBIE_STEP).floor();
    }
}

fn move_ash(world: &mut World, action: &Action) {
    world.ash.pos = world.ash.pos.towards(action.target, constants::MAX_ASH_STEP).floor();
}

fn destroy_zombies(world: &mut World, events: &mut Vec<Event>) {
    let max_distance_squared = constants::MAX_ASH_KILL_RANGE.powf(2.0);
    let mut zombie_ids_to_delete = HashSet::<i32>::new();

    let base_kill_score = calculate_zombie_kill_score(world.humans.len() as i32);
    let mut multiplier_sequence = FibonacciSequence::new();

    for zombie in world.zombies.values() {
        if zombie.pos.distance_to_squared(world.ash.pos) <= max_distance_squared {
            zombie_ids_to_delete.insert(zombie.id);

            let multiplier = multiplier_sequence.next() as f32;
            events.push(Event::ZombieKilled {
                tick: world.tick,
                zombie_id: zombie.id,
                score: multiplier * base_kill_score,
            });
        }
    }

    for zombie_id in zombie_ids_to_delete.iter() {
        world.zombies.remove(zombie_id);
    }
}

pub fn calculate_zombie_kill_score(num_humans: i32) -> f32 {
    10.0 * (num_humans as f32).powf(2.0)
}

fn destroy_humans(world: &mut World, events: &mut Vec<Event>) {
    if world.zombies.len() == 0 { return; } // Nothing to kill the humans
    if world.humans.len() == 0 { return; } // Game was already over before this turn started

    let max_distance_squared = constants::MAX_ZOMBIE_KILL_RANGE.powf(2.0);
    let mut human_ids_to_delete = HashSet::<i32>::new();
    for human in world.humans.values() {
        let close_zombie = world.zombies.values().any(|zombie| zombie.pos.distance_to_squared(human.pos) <= max_distance_squared);
        if close_zombie {
            human_ids_to_delete.insert(human.id);
            events.push(Event::HumanKilled { tick: world.tick, human_id: human.id });
        }
    }

    for human_id in human_ids_to_delete.iter() {
        world.humans.remove(human_id);
    }
}

fn is_over(world: &World) -> bool {
    world.humans.len() == 0 || world.zombies.len() == 0
}

fn emit_ending(world: &mut World, events: &mut Vec<Event>) {
    if world.humans.len() == 0 {
        events.push(Event::Lost { tick: world.tick, num_zombies: world.zombies.len() });
    } else if world.zombies.len() == 0 {
        events.push(Event::Won{ tick: world.tick, num_humans: world.humans.len() });
    }
}