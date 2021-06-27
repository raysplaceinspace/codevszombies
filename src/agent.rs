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

    calculate_next_action(&best_strategy, world)
}

fn generate_strategy(id: i32, world: &World) -> Strategy {
    let mut strategy = Strategy::new(id);
    let mut rng = rand::thread_rng();

    let mut remaining_zombie_ids = world.zombies.iter().map(|zombie| zombie.id).collect::<Vec<i32>>();
    while remaining_zombie_ids.len() > 0 {
        let zombie_id = remaining_zombie_ids.remove(rng.gen_range(0..remaining_zombie_ids.len()));
        strategy.milestones.push(Milestone { zombie_id });
    }

    strategy
}

fn rollout(strategy: &Strategy, initial: &World) -> f32 {
    let mut world = initial.clone();
    for _ in 0..MAX_ROLLOUT_TICKS {
        let action = calculate_next_action(strategy, &world);
        simulator::next(&mut world, &action);

        if simulator::is_terminal(&world) {
            break;
        }
    }

    let score = evaluation::evaluate(&world);
    score
}

fn calculate_next_action(strategy: &Strategy, world: &World) -> Action {
    let mut target = world.pos;
    for milestone in strategy.milestones.iter() {
        // TODO: Find zombie in constant time
        match world.zombies.iter().find(|zombie| zombie.id == milestone.zombie_id) {
            Some(zombie) => {
                target = zombie.next;
                break;
            },
            None => (),
        }
    }

    Action { target }
}