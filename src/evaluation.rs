pub use super::model::*;
use super::simulator;

const WON_POINTS: f32 = 0.0;
const POINTS_PER_ZOMBIE: f32 = -1000.0;
const POINTS_PER_HUMAN: f32 = 0.0;
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
                Event::HumanKilled { .. } => {
                    self.total_score += POINTS_PER_HUMAN;
                },
                Event::Won{ tick, .. } => {
                    self.total_score += POINTS_PER_TICK * (*tick as f32);
                    self.total_score += WON_POINTS;
                },
                Event::Lost{ tick, num_zombies, .. } => {
                    self.total_score += POINTS_PER_TICK * (*tick as f32);
                    self.total_score += POINTS_PER_ZOMBIE * (*num_zombies as f32);
                },
            }
        }
    }
}