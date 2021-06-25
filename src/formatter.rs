use super::model::*;

pub fn format_action(action: &Action) -> String {
    format!("{} {}", action.target.x, action.target.y)
}