pub use super::model::*;

use rand;
use rand::Rng;
use rand::prelude::ThreadRng;

const SCORE_SHEET_SIZE: i32 = 10;

const WON_POINTS: f32 = 0.0;
const LOSS_POINTS: f32 = -10000.0;
const POINTS_PER_ZOMBIE: f32 = -1000.0;
const BONUS_POINTS_PER_HUMAN: f32 = -1000.0;
const POINTS_PER_TICK: f32 = -0.01;
const POINTS_PER_MILESTONE: f32 = -0.001;

pub struct ScoreParams {
    save_humans_multiplier: f32,
    kill_zombies_multiplier: f32,
}

impl ScoreParams {
    pub fn official() -> ScoreParams {
        ScoreParams {
            save_humans_multiplier: 0.0,
            kill_zombies_multiplier: 1.0,
        }
    }

    pub fn gen(rng: &mut ThreadRng) -> ScoreParams {
        ScoreParams {
            kill_zombies_multiplier: rng.gen::<f32>(),
            save_humans_multiplier: rng.gen::<f32>(),
        }
    }

    pub fn gen_sheet(rng: &mut ThreadRng) -> Vec<ScoreParams> {
        let mut score_sheet = vec![ScoreParams::official()];

        for _ in 0..SCORE_SHEET_SIZE {
            score_sheet.push(ScoreParams::gen(rng));
        }

        score_sheet
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
                Event::ZombieKilled { score, .. } => {
                    self.total_score += self.params.kill_zombies_multiplier * score;
                },
                Event::HumanKilled { .. } => {
                    self.total_score += self.params.save_humans_multiplier * BONUS_POINTS_PER_HUMAN;
                },
                Event::Won{ tick, .. } => {
                    self.total_score += POINTS_PER_TICK * (*tick as f32);
                    self.total_score += WON_POINTS;
                },
                Event::Lost{ tick, num_zombies, .. } => {
                    self.total_score += POINTS_PER_TICK * (*tick as f32);
                    self.total_score += POINTS_PER_ZOMBIE * (*num_zombies as f32);
                    self.total_score += LOSS_POINTS;
                },
            }
        }
    }
}