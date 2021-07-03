pub use super::model::*;

use std::cmp;
use std::collections::HashSet;
use std::ops;
use rand;
use rand::Rng;

const REPLACE_MOVE_PROPORTION: f32 = 0.5;
const BUMP_MOVE_PROPORTION: f32 = 0.25;

const ATTACK_ZOMBIE_PROPORTION: f32 = 0.1;
const PROTECT_HUMAN_PROPORTION: f32 = 0.1;
const DROP_PROPORTION: f32 = 0.05;

const BUBBLE_PROPORTION: f32 = 0.1;
const SWAP_PROPORTION: f32 = 0.05;
const DISPLACE_PROPORTION: f32 = 0.5;

pub fn mutate_strategy(strategy: &mut Strategy, world: &World, rng: &mut rand::prelude::ThreadRng) -> bool {
    let mut mutated = false;

    if !mutated && rng.gen::<f32>() < BUMP_MOVE_PROPORTION { mutated = bump_move(strategy, rng); }
    if !mutated && rng.gen::<f32>() < REPLACE_MOVE_PROPORTION { mutated = replace_move(strategy, rng); }

    if !mutated && rng.gen::<f32>() < DROP_PROPORTION { mutated = drop_element(strategy, rng); }

    if !mutated && rng.gen::<f32>() < ATTACK_ZOMBIE_PROPORTION { mutated = insert_attack(world, strategy, rng); }
    if !mutated && rng.gen::<f32>() < PROTECT_HUMAN_PROPORTION { mutated = insert_defend(world, strategy, rng); }

    if !mutated && rng.gen::<f32>() < BUBBLE_PROPORTION { mutated = bubble_elements(strategy, rng); }
    if !mutated && rng.gen::<f32>() < SWAP_PROPORTION { mutated = swap_elements(strategy, rng); }
    if !mutated && rng.gen::<f32>() < DISPLACE_PROPORTION { mutated = displace_section(world, strategy, rng); }

    mutated
}

fn bump_move(strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    const MUTATE_RADIUS: f32 = constants::MAX_ASH_STEP + constants::MAX_ASH_KILL_RANGE + 1.0; // Be able to step away from killing something

    match choose_move_index(strategy, rng) {
        Some(move_index) => {
            match strategy.milestones[move_index] {
                Milestone::MoveTo { target: previous } => {
                    strategy.milestones[move_index] = Milestone::MoveTo {
                        target: V2 {
                            x: clamp(previous.x + rng.gen_range(-MUTATE_RADIUS..MUTATE_RADIUS), 0.0, constants::MAP_WIDTH as f32),
                            y: clamp(previous.y + rng.gen_range(-MUTATE_RADIUS..MUTATE_RADIUS), 0.0, constants::MAP_HEIGHT as f32),
                        },
                    }
                },
                _ => {},
            };
            true
        },
        None => false,
    }
}

fn replace_move(strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    const KEEP_PROBABILITY: f32 = 0.9;

    // Drop random number of items
    strategy.milestones.retain(|_| rng.gen::<f32>() < KEEP_PROBABILITY);

    // Insert new move
    let target = V2 {
        x: rng.gen_range(0..constants::MAP_WIDTH) as f32,
        y: rng.gen_range(0..constants::MAP_HEIGHT) as f32,
    };
    let insert_index = rng.gen_range(0 .. (strategy.milestones.len() + 1)); // +1 because can add to end of list
    strategy.milestones.insert(insert_index, Milestone::MoveTo {
        target,
    });

    true
}
fn drop_element(strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    if strategy.milestones.len() == 0 { return false; }

    let drop_index = rng.gen_range(0..strategy.milestones.len());
    strategy.milestones.remove(drop_index);
    
    true
}

fn choose_move_index(strategy: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<usize> {
    let num_moves = strategy.milestones.iter().filter(|m| m.is_move()).count();
    if num_moves == 0 { return None }

    let target_move_ordinal = rng.gen_range(0 .. num_moves);
    let mut current_move_ordinal = 0;
    for (i, milestone) in strategy.milestones.iter().enumerate() {
        if milestone.is_move() {
            if current_move_ordinal == target_move_ordinal {
                return Some(i);
            }
            current_move_ordinal += 1;
        }
    }

    None
}

fn insert_attack(world: &World, strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    if world.zombies.len() == 0 { return false }

    let mut zombie_ids = world.zombies.keys().map(|k| *k).collect::<HashSet<i32>>();
    for milestone in strategy.milestones.iter() { // Remove zombie IDs that we're already attacking
        match milestone {
            Milestone::KillZombie { zombie_id } => { zombie_ids.remove(zombie_id); },
            _ => (),
        }
    }
    if zombie_ids.len() == 0 { return false }

    let zombie_id = *zombie_ids.iter().nth(rng.gen_range(0..zombie_ids.len())).unwrap();
    let insert_index = rng.gen_range(0 .. (strategy.milestones.len() + 1)); // +1 because can add at end of vec

    strategy.milestones.insert(insert_index, Milestone::KillZombie {
        zombie_id,
    });

    true
}

fn insert_defend(world: &World, strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    if world.humans.len() == 0 { return false; }

    let human_index = rng.gen_range(0..world.humans.len());
    let human = world.humans.values().nth(human_index).unwrap();

    let insert_index = rng.gen_range(0 .. (strategy.milestones.len() + 1)); // +1 because can add at end of vec

    strategy.milestones.insert(insert_index, Milestone::MoveTo{
        target: human.pos,
    });

    true
}

fn bubble_elements(strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    if strategy.milestones.len() < 2 { return false; }

    let bubble_index = rng.gen_range(0..(strategy.milestones.len() - 1));
    strategy.milestones.swap(bubble_index, bubble_index + 1);

    true
}

fn swap_elements(strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    if strategy.milestones.len() < 2 { return false; }

    let from_index = rng.gen_range(0..(strategy.milestones.len() - 1));
    let mut to_index = rng.gen_range(0..(strategy.milestones.len() - 1));
    if from_index == to_index {
        to_index += 1;
    }

    strategy.milestones.swap(from_index, to_index);

    true
}

fn displace_section(world: &World, strategy: &mut Strategy, rng: &mut rand::prelude::ThreadRng) -> bool {
    let range_random = RangeRandom { max_length: cmp::min(world.zombies.len(), 10), power: 2.0 };

    if strategy.milestones.len() < 2 { return false; }

    let from = rng.gen_range(0 .. strategy.milestones.len());
    let length = 1 + range_random.gen(0 .. (strategy.milestones.len() - from), rng);
    let to = from + length;
    let reverse = rng.gen::<f32>() < 0.5;

    let displaced = strategy.milestones.drain(from..to).collect::<Vec<Milestone>>();
    let displace_to_index = rng.gen_range(0 .. (strategy.milestones.len() + 1)); // +1 because can displace to after the end as well

    if reverse {
        strategy.milestones.splice(displace_to_index .. displace_to_index, displaced.into_iter().rev());
    } else {
        strategy.milestones.splice(displace_to_index .. displace_to_index, displaced.into_iter());
    }

    true
}

fn clamp(v: f32, min_value: f32, max_value: f32) -> f32 {
    if v < min_value { min_value }
    else if v > max_value { max_value }
    else { v }
}

struct RangeRandom {
    max_length: usize,
    power: f32,
}

impl RangeRandom {
    pub fn gen(&self, range: ops::Range<usize>, rng: &mut rand::prelude::ThreadRng) -> usize {
        let base = (rng.gen::<f32>().powf(self.power) * self.max_length as f32) as usize;
        cmp::min(range.start + base, range.end - 1)
    }
}