pub use super::model::*;

use std::time::Instant;
use rand;
use rand::Rng;
use rand::prelude::ThreadRng;
use super::evaluation::ScoreParams;
use super::mutations;
use super::rollouts;
use super::rollouts::{Rollout, WorldState};

const MAX_STRATEGY_GENERATION_MILLISECONDS: u128 = 90;

const MUTATE_PROPORTION: f32 = 0.9;

const MAX_MOVES_FROM_SCRATCH: i32 = 1;

struct StrategyPool<'a> {
    strategy_id: i32,
    score_sheet: &'a Vec<ScoreParams>,
    best: Rollout,
    entries: Vec<StrategyPoolEntry>,
}

impl StrategyPool<'_> {
    fn new<'a>(world: &World, score_sheet: &'a Vec<ScoreParams>) -> StrategyPool<'a> {
        let mut strategy_id = 0;
        let rollout = rollouts::rollout(Strategy::new(strategy_id), world, &score_sheet);
        strategy_id += 1;

        StrategyPool {
            strategy_id,
            entries: (0..score_sheet.len()).map(|i| StrategyPoolEntry::from(&rollout, i)).collect::<Vec<_>>(),
            best: rollout,
            score_sheet,
        }
    }

    fn accept(&mut self, strategy: Strategy, world: &World) {
        let rollout = rollouts::rollout(strategy, world, &self.score_sheet);

        // Improve pool
        for i in 0..self.entries.len() {
            let score = rollout.scores[i];
            if score > self.entries[i].score {
                self.entries[i] = StrategyPoolEntry::from(&rollout, i);
            }
        }

        // Improve best
        if rollout.scores[0] > self.best.scores[0] {
            self.best = rollout;
        }
    }

    fn import(&mut self, strategies: Vec<Strategy>, world: &World) {
        for strategy in strategies {
            let candidate = strategy.seed(self.strategy_id);
            self.strategy_id += 1;
            self.accept(candidate, world);
        }
    }

    fn export(&mut self) -> Vec<Strategy> {
        self.entries.iter().map(|entry| entry.strategy.clone()).collect::<Vec<Strategy>>()
    }

    fn gen(&self, rng: &mut ThreadRng) -> &Strategy {
        &self.entries[rng.gen_range(0..self.entries.len())].strategy
    }
}

struct StrategyPoolEntry {
    strategy: Strategy,
    score: f32,
    actual: f32,
    ending: WorldState,
}

impl StrategyPoolEntry {
    fn new() -> StrategyPoolEntry {
        StrategyPoolEntry {
            strategy: Strategy::new(-1),
            score: f32::NEG_INFINITY,
            actual: f32::NEG_INFINITY,
            ending: WorldState::new(),
        }
    }

    fn from(rollout: &Rollout, score_sheet_index: usize) -> StrategyPoolEntry {
        StrategyPoolEntry {
            strategy: rollout.strategy.clone(),
            score: rollout.scores[score_sheet_index],
            actual: rollout.scores[0],
            ending: rollout.ending.clone(),
        }
    }
}

pub fn choose(world: &World, score_sheet: &Vec<ScoreParams>, previous_strategies: Vec<Strategy>, rng: &mut ThreadRng) -> Vec<Strategy> {
    let mut strategy_id = 0;

    let mut pool = StrategyPool::new(world, score_sheet);
    pool.import(previous_strategies, world);

    let initial_scores = pool.entries.iter().map(|entry| entry.score).collect::<Vec<_>>();

    let start = Instant::now();
    while start.elapsed().as_millis() < MAX_STRATEGY_GENERATION_MILLISECONDS {
        strategy_id += 1;

        let initial_strategy = pool.gen(rng);
        let strategy = generate_strategy(strategy_id, &initial_strategy, world, rng);
        pool.accept(strategy, world);
    }

    eprintln!("Chosen generation {} after {} total generations", pool.best.strategy.id, strategy_id);
    eprintln!("Chosen strategy: {}", &pool.best.strategy);

    eprintln!("Optimized score (after {} generations):", strategy_id);
    for (i, entry) in pool.entries.iter().enumerate() {
        eprintln!(" #[{}]: {} -> {} ({}) (h={}, z={})", entry.strategy.id, initial_scores[i], entry.score, entry.actual, entry.ending.num_humans, entry.ending.num_zombies);
    }

    eprintln!("Tick {}: chosen strategy rolled out to tick {}", world.tick, pool.best.ending.tick);
    for event in pool.best.events.iter() {
        eprintln!(" {}", event);
    }
    
    pool.export()
}

fn generate_strategy(id: i32, incumbent: &Strategy, world: &World, rng: &mut rand::prelude::ThreadRng) -> Strategy {
    let mut strategy: Option<Strategy> = None;

    if rng.gen::<f32>() < MUTATE_PROPORTION {
        let mut candidate = incumbent.seed(id);
        let mutated = mutations::mutate_strategy(&mut candidate, world, rng);

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
