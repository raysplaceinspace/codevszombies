pub use super::geometry::*;
use std::collections::HashMap;

pub mod constants {
    pub const MAP_WIDTH: i32 = 16000;
    pub const MAP_HEIGHT: i32 = 9000;
    pub const MAX_ASH_STEP: f32 = 1000.0;
    pub const MAX_ASH_KILL_RANGE: f32 = 2000.0;
    pub const MAX_ZOMBIE_STEP: f32 = 400.0;
    pub const MAX_ZOMBIE_KILL_RANGE: f32 = 0.0001;
}

#[derive(Clone)]
pub struct Human {
    pub id: i32,
    pub pos: V2,
}

#[derive(Clone)]
pub struct Zombie {
    pub id: i32,
    pub pos: V2,
    pub next: V2,
}

#[derive(Clone)]
pub struct World {
    pub tick: i32,
    pub pos: V2,
    pub humans: HashMap<i32, Human>,
    pub zombies: HashMap<i32, Zombie>,
}

#[derive(Clone)]
pub struct Action {
    pub target: V2,
}

#[derive(Clone)]
pub enum Milestone {
    KillZombie { zombie_id: i32 },
    MoveTo { target: V2 },
}

impl Milestone {
    pub fn is_move(&self) -> bool {
        match self { Milestone::MoveTo{..} => true, _ => false }
    }
    pub fn is_kill_zombie(&self) -> bool {
        match self { Milestone::KillZombie{..} => true, _ => false }
    }
}

pub struct Strategy {
    pub id: i32,
    pub milestones: Vec<Milestone>,
}

impl Strategy {
    pub fn new(id: i32) -> Strategy {
        Strategy { id, milestones: Vec::new() }
    }

    pub fn clone(&self, id: i32) -> Strategy {
        Strategy {
            id,
            milestones: self.milestones.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.milestones.len() == 0
    }
}

pub enum Event {
    ZombieKilled { tick: i32, zombie_id: i32, score: f32 },
    HumanKilled { tick: i32, human_id: i32 },
    Ending { tick: i32, won: bool }
}