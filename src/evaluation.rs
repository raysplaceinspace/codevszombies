pub use super::model::*;
use super::simulator;

const LOSS_POINTS: f32 = -10000.0;
const POINTS_PER_TICK: f32 = -0.01;
const POINTS_PER_MILESTONE: f32 = -0.001;

pub struct ScoreAccumulator {
    pub total_score: f32,
}

impl ScoreAccumulator {
    pub fn new() -> ScoreAccumulator {
        ScoreAccumulator { total_score: 0.0 }
    }

    pub fn evaluate_strategy(&mut self, strategy: &Strategy) {
        self.total_score += POINTS_PER_MILESTONE * (strategy.milestones.len() as f32);
    }

    pub fn accumulate(&mut self, events: &Vec<Event>) {
        for event in events.iter() {
            match event {
                Event::ZombieKilled { score, .. } => {
                    self.total_score += score;
                },
                Event::Won{ tick, .. } => {
                    self.total_score += POINTS_PER_TICK * (*tick as f32);
                },
                Event::Lost{ tick, .. } => {
                    self.total_score += POINTS_PER_TICK * (*tick as f32);
                    self.total_score += LOSS_POINTS;
                },
                _ => (),
            }
        }
    }

    pub fn upper_bound(&self, world: &World) -> f32 {
        let mut maximum_gain: f32 = 0.0;
        let mut multiplier = simulator::FibonacciSequence::new();
        let score_per_zombie = simulator::calculate_zombie_kill_score(world.humans.len() as i32);
        for _ in 0..world.zombies.len() {
            maximum_gain += (multiplier.next() as f32) * score_per_zombie
        }
        self.total_score + maximum_gain
    }
}