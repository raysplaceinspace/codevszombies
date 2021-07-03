use std::fmt;
use super::model::*;

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} {:.0}", self.target.x, self.target.y)
    }
}

impl fmt::Display for Milestone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Milestone::KillZombie { zombie_id } => { write!(f, "z{} ", zombie_id) },
            Milestone::ProtectHuman { human_id } => { write!(f, "h{} ", human_id) },
            Milestone::MoveTo { target } => { write!(f, "({:.0},{:.0}) ", target.x, target.y) },
        }
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] ", self.id).unwrap();

        for milestone in self.milestones.iter() {
            match milestone {
                Milestone::KillZombie { zombie_id } => { write!(f, "z{} ", zombie_id).unwrap(); },
                Milestone::ProtectHuman { human_id } => { write!(f, "h{} ", human_id).unwrap(); },
                Milestone::MoveTo { target } => { write!(f, "({:.0},{:.0}) ", target.x, target.y).unwrap(); },
            }
        }

        write!(f, "") // TODO: Return the right value without doing this
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Event::ZombieKilled { tick, zombie_id, score, .. } => write!(f, "{}> zombie {} killed, +{}", tick, zombie_id, score),
            Event::HumanKilled { tick, human_id, .. } => write!(f, "{}> human {} killed", tick, human_id),
            Event::Won { tick, num_humans, .. } => write!(f, "{}> won - {} humans remain", tick, num_humans),
            Event::Lost { tick, num_zombies, .. } => write!(f, "{}> lost {} zombies remain", tick, num_zombies),
        }
    }
}

impl fmt::Display for Zombie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "z{}", self.id)
    }
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "h{}", self.id)
    }
}

impl fmt::Display for Ash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a0")
    }
}