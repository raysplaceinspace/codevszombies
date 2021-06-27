pub use super::model::*;

use std::time::Instant;
use rand;
use rand::Rng;
use super::evaluation;
use super::simulator;
use super::formatter;

const MAX_ROLLOUT_TICKS: i32 = 50;
const MAX_STRATEGY_GENERATION_MILLISECONDS: u128 = 80;

pub fn choose(world: &World) -> Action {
    let mut strategy_id = 0;
    let mut best_strategy = Strategy::new(strategy_id);
    let mut best_strategy_score = f32::NEG_INFINITY;

    let start = Instant::now();
    while start.elapsed().as_millis() < MAX_STRATEGY_GENERATION_MILLISECONDS {
        strategy_id += 1;

        let strategy = generate_strategy(strategy_id, world);
        let score = rollout(&strategy, world);
        if score > best_strategy_score {
            best_strategy_score = score;
            best_strategy = strategy;
        }
    }

    eprintln!("Chosen strategy (after {} generations): {} -> {}", strategy_id, formatter::format_strategy(&best_strategy), best_strategy_score);

    strategy_to_action(&best_strategy, world)
}

fn generate_strategy(id: i32, world: &World) -> Strategy {
    let mut strategy = Strategy::new(id);
    let mut rng = rand::thread_rng();

    let mut remaining_zombie_ids = world.zombies.iter().map(|zombie| zombie.id).collect::<Vec<i32>>();
    while remaining_zombie_ids.len() > 0 {
        let zombie_id = remaining_zombie_ids.remove(rng.gen_range(0..remaining_zombie_ids.len()));
        strategy.milestones.push(Milestone::KillZombie { zombie_id });
    }

    strategy
}

fn rollout(strategy: &Strategy, initial: &World) -> f32 {
    let mut world = initial.clone();
    let mut all_events = Vec::<Event>::new();
    for _ in 0..MAX_ROLLOUT_TICKS {
        let action = strategy_to_action(strategy, &world);
        let tick_events = simulator::next(&mut world, &action);

        let is_finished = tick_events.iter().any(
            |event| match event {
                Event::Ending { .. } => true,
                _ => false
            }
        );
        all_events.extend(tick_events.into_iter());

        if is_finished { break; }
    }

    let score = evaluation::evaluate(&world, &all_events);
    score
}

fn strategy_to_action(strategy: &Strategy, world: &World) -> Action {
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
    }
}

fn kill_zombie_to_action(zombie_id: i32, world: &World) -> Option<Action> {
    // TODO: Find zombie in constant time
    match world.zombies.iter().find(|zombie| zombie.id == zombie_id) {
        Some(zombie) => Some(Action { target: zombie.next }),
        None => None,
    }
}