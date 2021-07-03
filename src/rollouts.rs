pub use super::model::*;

use super::evaluation::{ScoreAccumulator, ScoreParams};
use super::simulator;

const MAX_ROLLOUT_TICKS: i32 = 50;

#[derive(Clone)]
pub struct Rollout {
    pub strategy: Strategy,
    pub events: Vec<Event>,
    pub ending: WorldState,
    pub scores: Vec<f32>,
}

#[derive(Clone)]
pub struct WorldState {
    pub tick: i32,
    pub num_zombies: usize,
    pub num_humans: usize,
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState { tick: -1, num_zombies: 0, num_humans: 0 }
    }

    pub fn from(world: &World) -> WorldState {
        WorldState { tick: world.tick, num_zombies: world.zombies.len(), num_humans: world.humans.len() }
    }
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
            action = milestone.to_action(world);

            match action {
                Some(_) => break, // Found a valid action, return it and don't advance to next milestone
                None => { self.current_index += 1 }, // Milestone complete, consume this milestone
            }
        }

        action.unwrap_or(Action { target: world.ash.pos })
    }
}


pub fn rollout(strategy: Strategy, initial: &World, score_params: &Vec<ScoreParams>) -> Rollout {
    let mut world = initial.clone();
    let mut events = Vec::<Event>::new();

    let mut score_accumulators = score_params.iter().map(|params| ScoreAccumulator::new(&world, params)).collect::<Vec<_>>();
    let mut action_emitter = ActionEmitter::new(&strategy);

    for score_accumulator in score_accumulators.iter_mut() {
        score_accumulator.evaluate_strategy(&strategy);
    }

    for _ in 0..MAX_ROLLOUT_TICKS {
        let action = action_emitter.next(&world);
        let tick_events = simulator::next(&mut world, &action);

        for score_accumulator in score_accumulators.iter_mut() {
            score_accumulator.accumulate(&tick_events);
        }

        let is_finished = tick_events.iter().any(|event| event.is_ending());
        events.extend(tick_events.into_iter());

        if is_finished { break; }
    }

    Rollout {
        strategy,
        events,
        ending: WorldState::from(&world),
        scores: score_accumulators.iter().map(|x| x.total_score).collect::<Vec<f32>>(),
    }
}

pub fn strategy_to_action(strategy: &Strategy, world: &World) -> Action {
    let mut action_emitter = ActionEmitter::new(strategy);
    action_emitter.next(world)
}