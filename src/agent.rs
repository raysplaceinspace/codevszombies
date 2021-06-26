pub use super::model::*;

use rand;
use rand::Rng;
use super::evaluation;
use super::simulator;

const MAX_ROLLOUT_TICKS: i32 = 50;
const MAX_STRATEGY_GENERATIONS: i32 = 100;

struct Milestone {
    zombie_id: i32,
}

struct Strategy {
    milestones: Vec<Milestone>,
}

impl Strategy {
    pub fn new() -> Strategy {
        Strategy { milestones: Vec::new() }
    }
}

pub fn choose(world: &World) -> Action {
    let mut best_strategy = Strategy::new();
    let mut best_strategy_score = f32::NEG_INFINITY;

    for _ in 0..MAX_STRATEGY_GENERATIONS {
        let strategy = generate_strategy(world);
        let score = rollout(&strategy, world);
        if score > best_strategy_score {
            best_strategy_score = score;
            best_strategy = strategy;
        }
    }

    calculate_next_action(&best_strategy, world)
}

fn generate_strategy(world: &World) -> Strategy {
    let mut strategy = Strategy::new();
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
        simulator::next(&mut world, &action)
    }

    evaluation::evaluate(&world)
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