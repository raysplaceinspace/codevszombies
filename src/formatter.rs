use std::fmt::Write;
use super::model::*;

pub fn format_action(action: &Action) -> String {
    format!("{} {}", action.target.x, action.target.y)
}

pub fn format_strategy(strategy: &Strategy) -> String {
    let mut result = String::new();

    write!(result, "[{}]: ", strategy.id).unwrap();

    for milestone in strategy.milestones.iter() {
        match milestone {
            Milestone::KillZombie { zombie_id } => { write!(result, "z{} ", zombie_id).unwrap(); },
        }
    }

    result
}