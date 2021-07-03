pub use super::model::*;

use std::time::Instant;
use rand;
use rand::Rng;
use super::evaluation;
use super::mutations;
use super::rollouts;

const MAX_STRATEGY_GENERATION_MILLISECONDS: u128 = 90;

const MUTATE_PROPORTION: f32 = 0.75;
const MAX_MUTATIONS: i32 = 1;
const MUTATION_REPEAT_PROBABILITY: f32 = 0.1;

const MAX_MOVES_FROM_SCRATCH: i32 = 1;

struct StrategyPoolEntry {
    strategy: Strategy,
    score: f32,
}

impl StrategyPoolEntry {
    pub fn new(strategy: Strategy, score: f32) -> StrategyPoolEntry {
        StrategyPoolEntry { strategy, score }
    }
}

pub fn choose(world: &World, previous_strategy: &Strategy) -> Strategy {
    let mut rng = rand::thread_rng();

    let mut strategy_id = 0;

    let score_sheet = vec![
        evaluation::ScoreParams::official(),
        evaluation::ScoreParams::gen(&mut rng),
        evaluation::ScoreParams::gen(&mut rng),
        evaluation::ScoreParams::gen(&mut rng),
        evaluation::ScoreParams::gen(&mut rng),
    ];

    let mut best_rollout = rollouts::rollout(previous_strategy.seed(strategy_id), world, &score_sheet);
    let mut pool = best_rollout.scores.iter().map(|score| StrategyPoolEntry::new(best_rollout.strategy.clone(), *score)).collect::<Vec<_>>();

    let initial_scores = best_rollout.scores.clone();

    let start = Instant::now();
    while start.elapsed().as_millis() < MAX_STRATEGY_GENERATION_MILLISECONDS {
        strategy_id += 1;

        let initial_strategy = &pool[rng.gen_range(0..pool.len())].strategy;
        let strategy = generate_strategy(strategy_id, &initial_strategy, world, &mut rng);
        let rollout = rollouts::rollout(strategy, world, &score_sheet);

        // Improve pool
        for i in 0..pool.len() {
            let score = rollout.scores[i];
            if score > pool[i].score {
                pool[i] = StrategyPoolEntry::new(rollout.strategy.clone(), score);
            }
        }

        // Improve overall best
        if rollout.scores[0] > best_rollout.scores[0] {
            best_rollout = rollout;
        }
    }

    eprintln!("Chosen generation {} after {} total generations", best_rollout.strategy.id, strategy_id);
    eprintln!("Chosen strategy: {}", &best_rollout.strategy);

    eprintln!("Optimized score (after {} generations): {} -> {}", strategy_id, initial_scores[0], best_rollout.scores[0]);
    for i in 0..score_sheet.len() {
        eprintln!(" #{}: {} -> {} [{}]", i, initial_scores[i], pool[i].score, pool[i].strategy.id);
    }

    eprintln!("Tick {}: chosen strategy rolled out to tick {}", world.tick, best_rollout.final_tick);
    for event in best_rollout.events.iter() {
        eprintln!(" {}", event);
    }
    
    best_rollout.strategy
}

fn generate_strategy(id: i32, incumbent: &Strategy, world: &World, rng: &mut rand::prelude::ThreadRng) -> Strategy {
    let mut strategy: Option<Strategy> = None;

    if rng.gen::<f32>() < MUTATE_PROPORTION {
        let mut candidate = incumbent.seed(id);
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

    let num_moves = rng.gen_range(0..(MAX_MOVES_FROM_SCRATCH+1)); // MAX_MOVES_FROM_SCRATCH is inclusive
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
