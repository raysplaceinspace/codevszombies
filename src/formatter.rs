use super::model::*;

pub fn format_action(action: &Action) -> String {
    format!("{} {}", action.target.x, action.target.y)
}

pub fn format_strategy(strategy: &Strategy) -> String {
    let mut result = String::new();
    let id_str = format!("[{}]: ", strategy.id);
    result.push_str(&id_str);

    for milestone in strategy.milestones.iter() {
        let milestone_str = format_milestone(&milestone);
        result.push_str(&milestone_str);
        result.push(' ');
    }

    result
}

pub fn format_milestone(milestone: &Milestone) -> String {
    format!("{}", milestone.zombie_id)
}