pub use super::model::*;

use std::time::Instant;
use rand;
use rand::Rng;
use super::mutations;
use super::rollouts;

const MAX_STRATEGY_GENERATION_MILLISECONDS: u128 = 90;

const MUTATE_PROPORTION: f32 = 0.75;
const MAX_MUTATIONS: i32 = 2;
const MUTATION_REPEAT_PROBABILITY: f32 = 0.1;

const MAX_MOVES_FROM_SCRATCH: i32 = 2;

pub fn choose(world: &World, previous_strategy: &Strategy) -> Strategy {
    let mut rng = rand::thread_rng();

    let mut strategy_id = 0;

    let mut best_strategy = previous_strategy.clone(strategy_id);
    let mut best_strategy_result = rollouts::rollout(&best_strategy, world, f32::NEG_INFINITY);

    let initial_strategy_score = best_strategy_result.score;

    let start = Instant::now();
    while start.elapsed().as_millis() < MAX_STRATEGY_GENERATION_MILLISECONDS {
        strategy_id += 1;

        let strategy = generate_strategy(strategy_id, &best_strategy, world, &mut rng);
        let rollout_result = rollouts::rollout(&strategy, world, best_strategy_result.score);
        if rollout_result.score > best_strategy_result.score {
            best_strategy_result = rollout_result;
            best_strategy = strategy;
        }
    }

    eprintln!("Chosen generation {} after {} total generations", best_strategy.id, strategy_id);
    eprintln!("Chosen strategy: {}", &best_strategy);
    eprintln!("Tick {}: chosen strategy rolled out to tick {}", world.tick, best_strategy_result.final_tick);
    eprintln!("Optimized score (after {} generations): {} -> {}", strategy_id, initial_strategy_score, best_strategy_result.score);

    for event in best_strategy_result.events.iter() {
        eprintln!(" {}", event);
    }
    
    best_strategy
}

fn generate_strategy(id: i32, incumbent: &Strategy, world: &World, rng: &mut rand::prelude::ThreadRng) -> Strategy {
    let mut strategy: Option<Strategy> = None;

    if rng.gen::<f32>() < MUTATE_PROPORTION {
        let mut candidate = incumbent.clone(id);
        let mut mutated = false;
        for _ in 0..MAX_MUTATIONS {
            mutated |= mutations::mutate_strategy(&mut candidate, world, rng);

            if rng.gen::<f32>() < MUTATION_REPEAT_PROBABILITY {
                continue;
            } else {
                break;
            }
        }

        if mutated {
            strategy = Some(candidate);
        }
    }

    if strategy.is_none() {
        strategy = Some(generate_strategy_from_scratch(id, world, rng));
    }
    strategy.unwrap()
}

fn generate_strategy_from_scratch(id: i32, world: &World, rng: &mut rand::prelude::ThreadRng) -> Strategy {
    let mut strategy = Strategy::new(id);

    let num_moves = rng.gen_range(0..MAX_MOVES_FROM_SCRATCH);
    for _ in 0..num_moves {
        let target = V2 {
            x: rng.gen_range(0..constants::MAP_WIDTH) as f32,
            y: rng.gen_range(0..constants::MAP_HEIGHT) as f32,
        };
        strategy.milestones.push(Milestone::MoveTo { target });
    }

    let mut remaining_zombie_ids = world.zombies.values().map(|zombie| zombie.id).collect::<Vec<i32>>();
    while remaining_zombie_ids.len() > 0 {
        let zombie_id = remaining_zombie_ids.remove(rng.gen_range(0..remaining_zombie_ids.len()));
        strategy.milestones.push(Milestone::KillZombie { zombie_id });
    }

    strategy
}
