pub use super::model::*;

const LOSS_POINTS: f32 = -10000.0;

pub fn evaluate(_world: &World, events: &Vec<Event>) -> f32 {
    let mut total_score: f32 = 0.0;
    for event in events.iter() {
        match event {
            Event::ZombieKilled { score, .. } => {
                total_score += score;
            },
            Event::Ending { won, .. } => {
                if !won {
                    total_score += LOSS_POINTS;
                }
            },
            _ => (),
        }
    }
    total_score
}