pub use super::model::*;

use std::cmp;
use std::ops;
use std::time::Instant;
use rand;
use rand::Rng;
use super::evaluation;
use super::simulator;
use super::formatter;

const MAX_ROLLOUT_TICKS: i32 = 50;
const MAX_STRATEGY_GENERATION_MILLISECONDS: u128 = 90;

const REPLACE_MOVE_PROPORTION: f32 = 0.5;
const BUMP_MOVE_PROPORTION: f32 = 0.25;

const ATTACK_ZOMBIE_PROPORTION: f32 = 0.05;
const DROP_PROPORTION: f32 = 0.05;

const BUBBLE_PROPORTION: f32 = 0.1;
const SWAP_PROPORTION: f32 = 0.05;
const DISPLACE_PROPORTION: f32 = 0.5;

struct Rollout {
    strategy_id: i32,
    events: Vec<Event>,
    final_tick: i32,
    score: f32,
}

struct ActionEmitter<'a> {
    strategy: &'a Strategy,
    current_index: usize,
}

impl ActionEmitter<'_> {
    pub fn new<'a>(strategy: &'a Strategy) -> ActionEmitter<'a> {
        ActionEmitter { strategy, current_index: 0 }
    }

    pub fn next(&mut self, world: &World) -> Action {
        let mut action: Option<Action> = None;

        while self.current_index < self.strategy.milestones.len() {
            let milestone = &self.strategy.milestones[self.current_index];
            action = actions::from_milestone(milestone, world);

            match action {
                Some(_) => break, // Found a valid action, return it and don't advance to next milestone
                None => { self.current_index += 1 }, // Milestone complete, consume this milestone
            }
        }

        action.unwrap_or(Action { target: world.pos })
    }
}

pub fn choose(world: &World, previous_strategy: &Strategy) -> Strategy {
    let mut rng = rand::thread_rng();

    let mut strategy_id = 0;
    let mut best_strategy = previous_strategy.clone(strategy_id);
    let mut best_strategy_result = rollout(&best_strategy, world, f32::NEG_INFINITY);

    let initial_strategy_score = best_strategy_result.score;

    let start = Instant::now();
    while start.elapsed().as_millis() < MAX_STRATEGY_GENERATION_MILLISECONDS {
        strategy_id += 1;

        let strategy = generate_strategy(strategy_id, &best_strategy, world, &mut rng);
        let rollout_result = rollout(&strategy, world, best_strategy_result.score);
        if rollout_result.score > best_strategy_result.score {
            best_strategy_result = rollout_result;
            best_strategy = strategy;
        }
    }

    eprintln!("Chosen strategy: {}", formatter::format_strategy(&best_strategy));
    eprintln!("Chosen generation {} after {} total generations", best_strategy.id, strategy_id);
    eprintln!("Optimized score (after {} generations): {} -> {}", strategy_id, initial_strategy_score, best_strategy_result.score);

    for event in best_strategy_result.events.iter() {
        eprintln!(" {}", formatter::format_event(event));
    }
    
    best_strategy
}

fn generate_strategy(id: i32, best_strategy: &Strategy, world: &World, rng: &mut rand::prelude::ThreadRng) -> Strategy {
    let mut strategy: Option<Strategy> = None;

    if strategy.is_none() && rng.gen::<f32>() < BUMP_MOVE_PROPORTION { strategy = bump_move(id, best_strategy, rng); }
    if strategy.is_none() && rng.gen::<f32>() < REPLACE_MOVE_PROPORTION { strategy = replace_move(id, best_strategy, rng); }

    if strategy.is_none() && rng.gen::<f32>() < DROP_PROPORTION { strategy = drop_element(id, best_strategy, rng); }

    if strategy.is_none() && rng.gen::<f32>() < ATTACK_ZOMBIE_PROPORTION { strategy = insert_attack(id, world, best_strategy, rng); }

    if strategy.is_none() && rng.gen::<f32>() < BUBBLE_PROPORTION { strategy = bubble_elements(id, best_strategy, rng); }
    if strategy.is_none() && rng.gen::<f32>() < SWAP_PROPORTION { strategy = swap_elements(id, best_strategy, rng); }
    if strategy.is_none() && rng.gen::<f32>() < DISPLACE_PROPORTION { strategy = displace_section(id, world, best_strategy, rng); }

    if strategy.is_none() {
        strategy = Some(Strategy::new(id)); // Some(generate_strategy_from_scratch(id, world, rng));
    }
    strategy.unwrap()
}

fn bump_move(id: i32, incumbent: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<Strategy> {
    const MUTATE_RADIUS: f32 = constants::MAX_ASH_STEP + constants::MAX_ASH_KILL_RANGE + 1.0; // Be able to step away from killing something

    match choose_move_index(incumbent, rng) {
        Some(move_index) => {
            let mut strategy = incumbent.clone(id);
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

            Some(strategy)
        },
        None => None,
    }
}

fn replace_move(id: i32, incumbent: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<Strategy> {
    const KEEP_PROBABILITY: f32 = 0.9;

    let mut strategy = incumbent.clone(id);

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

    Some(strategy)
}
fn drop_element(id: i32, incumbent: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<Strategy> {
    if incumbent.milestones.len() == 0 { return None; }
    let drop_index = rng.gen_range(0..incumbent.milestones.len());
    let mut strategy = incumbent.clone(id);
    strategy.milestones.remove(drop_index);
    Some(strategy)
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

fn insert_attack(id: i32, world: &World, incumbent: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<Strategy> {
    let num_zombies = world.zombies.len();
    if num_zombies == 0 { return None }

    let zombie_id = world.zombies.values().nth(rng.gen_range(0..num_zombies)).unwrap().id;
    let insert_index = rng.gen_range(0 .. (incumbent.milestones.len() + 1)); // +1 because can add at end of vec

    let mut strategy = incumbent.clone(id);
    strategy.milestones.insert(insert_index, Milestone::KillZombie {
        zombie_id,
    });
    Some(strategy)
}

fn bubble_elements(id: i32, incumbent: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<Strategy> {
    if incumbent.milestones.len() < 2 { return None }

    let mut strategy = incumbent.clone(id);
    let bubble_index = rng.gen_range(0..(incumbent.milestones.len() - 1));
    strategy.milestones.swap(bubble_index, bubble_index + 1);
    Some(strategy)
}

fn swap_elements(id: i32, incumbent: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<Strategy> {
    if incumbent.milestones.len() < 2 { return None }

    let mut strategy = incumbent.clone(id);

    let from_index = rng.gen_range(0..(incumbent.milestones.len() - 1));
    let mut to_index = rng.gen_range(0..(incumbent.milestones.len() - 1));
    if from_index == to_index {
        to_index += 1;
    }

    strategy.milestones.swap(from_index, to_index);

    Some(strategy)
}

fn displace_section(id: i32, world: &World, incumbent: &Strategy, rng: &mut rand::prelude::ThreadRng) -> Option<Strategy> {
    let range_random = RangeRandom { max_length: cmp::min(world.zombies.len(), 10), power: 2.0 };

    if incumbent.milestones.len() < 2 { return None }

    let from = rng.gen_range(0 .. incumbent.milestones.len());
    let length = 1 + range_random.gen(0 .. (incumbent.milestones.len() - from), rng);
    let to = from + length;
    let reverse = rng.gen::<f32>() < 0.5;

    let mut strategy = incumbent.clone(id);
    let displaced = strategy.milestones.drain(from..to).collect::<Vec<Milestone>>();
    let displace_to_index = rng.gen_range(0 .. (strategy.milestones.len() + 1)); // +1 because can displace to after the end as well

    if reverse {
        strategy.milestones.splice(displace_to_index .. displace_to_index, displaced.into_iter().rev());
    } else {
        strategy.milestones.splice(displace_to_index .. displace_to_index, displaced.into_iter());
    }

    Some(strategy)
}

fn rollout(strategy: &Strategy, initial: &World, best_score: f32) -> Rollout {
    let mut world = initial.clone();
    let mut events = Vec::<Event>::new();

    let mut score_accumulator = evaluation::ScoreAccumulator::new();
    let mut action_emitter = ActionEmitter::new(strategy);

    score_accumulator.evaluate_strategy(&strategy);

    for _ in 0..MAX_ROLLOUT_TICKS {
        let action = action_emitter.next(&world);
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
    let mut action_emitter = ActionEmitter::new(strategy);
    action_emitter.next(world)
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

mod actions {
    use super::*;

    pub fn from_milestone(milestone: &Milestone, world: &World) -> Option<Action> {
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
}