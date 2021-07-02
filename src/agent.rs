pub use super::model::*;

use std::time::Instant;
use rand;
use rand::Rng;
use super::evaluation;
use super::simulator;
use super::formatter;

const MAX_ROLLOUT_TICKS: i32 = 50;
const MAX_STRATEGY_GENERATION_MILLISECONDS: u128 = 80;

struct Rollout {
    strategy_id: i32,
    events: Vec<Event>,
    final_tick: i32,
    score: f32,
}

pub fn choose(world: &World, previous_strategy: &Strategy) -> Strategy {
    let mut strategy_id = 0;
    let mut best_strategy = previous_strategy.clone();
    best_strategy.id = strategy_id;

    let mut best_strategy_result = rollout(&best_strategy, world, f32::INFINITY);

    let start = Instant::now();
    while start.elapsed().as_millis() < MAX_STRATEGY_GENERATION_MILLISECONDS {
        strategy_id += 1;

        let strategy = generate_strategy(strategy_id, world);
        let rollout_result = rollout(&strategy, world, best_strategy_result.score);
        if rollout_result.score > best_strategy_result.score {
            best_strategy_result = rollout_result;
            best_strategy = strategy;
        }
    }

    eprintln!("Chosen strategy (after {} generations): {} -> {}", strategy_id, formatter::format_strategy(&best_strategy), best_strategy_result.score);
    for event in best_strategy_result.events.iter() {
        eprintln!(" {}", formatter::format_event(event));
    }
    
    best_strategy
}

fn generate_strategy(id: i32, world: &World) -> Strategy {
    let mut strategy = Strategy::new(id);
    let mut rng = rand::thread_rng();

    if rng.gen::<f32>() < 0.5 {
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

fn rollout(strategy: &Strategy, initial: &World, best_score: f32) -> Rollout {
    let mut world = initial.clone();
    let mut events = Vec::<Event>::new();
    let mut score_accumulator = evaluation::ScoreAccumulator::new();
    for _ in 0..MAX_ROLLOUT_TICKS {
        let action = strategy_to_action(strategy, &world);
        let tick_events = simulator::next(&mut world, &action);
        score_accumulator.accumulate(&tick_events);

        let is_finished = tick_events.iter().any(
            |event| match event {
                Event::Ending { .. } => true,
                _ => false
            }
        );
        events.extend(tick_events.into_iter());

        if is_finished { break; }

        if score_accumulator.upper_bound(&world) < best_score { break; }
    }

    Rollout {
        strategy_id: strategy.id,
        events,
        final_tick: world.tick,
        score: score_accumulator.total_score,
    }
}

pub fn strategy_to_action(strategy: &Strategy, world: &World) -> Action {
    for milestone in strategy.milestones.iter() {
        match milestone_to_action(milestone, world) {
            Some(action) => {
                return action;
            },
            None => (),
        }
    }

    // Fallback to non-action
    Action { target: world.pos }
}

fn milestone_to_action(milestone: &Milestone, world: &World) -> Option<Action> {
    match milestone {
        Milestone::KillZombie { zombie_id } => kill_zombie_to_action(*zombie_id, world),
        Milestone::MoveTo { target } => move_to_action(*target, world),
    }
}

fn kill_zombie_to_action(zombie_id: i32, world: &World) -> Option<Action> {
    match world.zombies.get(&zombie_id) {
        Some(zombie) => Some(Action { target: zombie.next }),
        None => None,
    }
}

fn move_to_action(target: V2, world: &World) -> Option<Action> {
    const PRECISION: f32 = 1.0;
    let distance = world.pos.distance_to(target);
    if distance < PRECISION {
        None
    } else {
        Some(Action { target })
    }
}