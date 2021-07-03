pub use super::model::*;

use rand;
use rand::Rng;

const WON_POINTS: f32 = 0.0;
const POINTS_PER_ZOMBIE: f32 = -1000.0;
const BONUS_POINTS_PER_HUMAN: f32 = -1000.0;
const POINTS_PER_TICK: f32 = -0.01;
const POINTS_PER_MILESTONE: f32 = -0.001;

pub struct ScoreParams {
    save_humans_multiplier: f32,
    kill_zombies_multiplier: f32,
    discount_rate: f32,
}

impl ScoreParams {
    pub fn official() -> ScoreParams {
        ScoreParams {
            save_humans_multiplier: 0.0,
            kill_zombies_multiplier: 1.0,
            discount_rate: 1.0,
        }
    }

    pub fn gen(rng: &mut rand::prelude::ThreadRng) -> ScoreParams {
        ScoreParams {
            kill_zombies_multiplier: rng.gen::<f32>(),
            save_humans_multiplier: rng.gen::<f32>(),
            discount_rate: 1.0 + rng.gen::<f32>(),
        }
    }
}

pub struct ScoreAccumulator<'a> {
    pub initial_tick: i32,
    pub total_score: f32,
    pub params: &'a ScoreParams,
}

impl ScoreAccumulator<'_> {
    pub fn new<'a>(world: &World, params: &'a ScoreParams) -> ScoreAccumulator<'a> {
        ScoreAccumulator { initial_tick: world.tick, total_score: 0.0, params }
    }

    pub fn evaluate_strategy(&mut self, strategy: &Strategy) {
        self.total_score += POINTS_PER_MILESTONE * (strategy.milestones.len() as f32);
    }

    pub fn accumulate(&mut self, events: &Vec<Event>) {
        for event in events.iter() {
            match event {
                Event::ZombieKilled { tick, score, .. } => {
                    let discount = self.discount(*tick);
                    self.total_score += discount * self.params.kill_zombies_multiplier * score;
                },
                Event::HumanKilled { tick, .. } => {
                    let discount = self.discount(*tick);
                    self.total_score += discount * self.params.save_humans_multiplier * BONUS_POINTS_PER_HUMAN;
                },
                Event::Won{ tick, .. } => {
                    let discount = self.discount(*tick);
                    self.total_score += discount * POINTS_PER_TICK * (*tick as f32);
                    self.total_score += discount * WON_POINTS;
                },
                Event::Lost{ tick, num_zombies, .. } => {
                    let discount = self.discount(*tick);
                    self.total_score += discount * POINTS_PER_TICK * (*tick as f32);
                    self.total_score += discount * POINTS_PER_ZOMBIE * (*num_zombies as f32);
                },
            }
        }
    }

    fn discount(&self, tick: i32) -> f32 {
        1.0 / self.params.discount_rate.powf((tick - self.initial_tick) as f32)
    }
}