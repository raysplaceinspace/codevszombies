pub use super::model::*;

use super::evaluation;
use super::simulator;

const MAX_ROLLOUT_TICKS: i32 = 50;

#[derive(Clone)]
pub struct Rollout {
    pub strategy_id: i32,
    pub events: Vec<Event>,
    pub final_tick: i32,
    pub score: f32,
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


pub fn rollout(strategy: &Strategy, initial: &World, best_score: f32) -> Rollout {
    let mut world = initial.clone();
    let mut events = Vec::<Event>::new();

    let mut score_accumulator = evaluation::ScoreAccumulator::new();
    let mut action_emitter = ActionEmitter::new(strategy);

    score_accumulator.evaluate_strategy(&strategy);

    for _ in 0..MAX_ROLLOUT_TICKS {
        let action = action_emitter.next(&world);
        let tick_events = simulator::next(&mut world, &action);
        score_accumulator.accumulate(&tick_events);

        let is_finished = tick_events.iter().any(|event| event.is_ending());
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