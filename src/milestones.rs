use super::*;

impl Milestone {
    pub fn to_action(&self, world: &World) -> Option<Action> {
        match &self {
            Milestone::KillZombie { zombie_id } => kill_zombie_to_action(*zombie_id, world),
            Milestone::ProtectHuman { human_id } => protect_human_to_action(*human_id, world),
            Milestone::MoveTo { target } => move_to_action(*target, world),
        }
    }
}

fn kill_zombie_to_action(zombie_id: i32, world: &World) -> Option<Action> {
    match world.zombies.get(&zombie_id) {
        Some(zombie) => Some(Action { target: zombie.next }),
        None => None,
    }
}

fn protect_human_to_action(human_id: i32, world: &World) -> Option<Action> {
    const PRECISION: f32 = 1.0;

    match world.humans.get(&human_id) {
        Some(human) => {
            let distance = world.ash.pos.distance_to(human.pos);
            if distance < PRECISION {
                None // Already at human, stop and move to next milestone
            } else {
                Some(Action { target: human.pos })
            }
        },
        None => None,
    }
}

fn move_to_action(target: V2, world: &World) -> Option<Action> {
    const PRECISION: f32 = 1.0;
    let distance = world.ash.pos.distance_to(target);
    if distance < PRECISION {
        None
    } else {
        Some(Action { target })
    }
}