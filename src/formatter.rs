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

pub fn format_event(event: &Event) -> String {
    match event {
        Event::ZombieKilled { tick, zombie_id, score, .. } => format!("{}> zombie {} killed, +{}", tick, zombie_id, score),
        Event::HumanKilled { tick, human_id, .. } => format!("{}> human {} killed", tick, human_id),
        Event::Ending { tick, won, .. } if *won => format!("{}> won", tick),
        Event::Ending { tick, .. } => format!("{}> lost", tick),
    }
}