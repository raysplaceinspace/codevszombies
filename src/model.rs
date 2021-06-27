pub use super::geometry::*;

pub mod constants {
    pub const MAX_ASH_STEP: f32 = 1000.0;
    pub const MAX_ASH_KILL_RANGE: f32 = 2000.0;
    pub const MAX_ZOMBIE_STEP: f32 = 400.0;
    pub const MAX_ZOMBIE_KILL_RANGE: f32 = 0.5;
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
    pub humans: Vec<Human>,
    pub zombies: Vec<Zombie>,
}

#[derive(Clone)]
pub struct Action {
    pub target: V2,
}

#[derive(Clone)]
pub struct Milestone {
    pub zombie_id: i32,
}

#[derive(Clone)]
pub struct Strategy {
    pub id: i32,
    pub milestones: Vec<Milestone>,
}

impl Strategy {
    pub fn new(id: i32) -> Strategy {
        Strategy { id, milestones: Vec::new() }
    }
}

pub enum Event {
    ZombieKilled { tick: i32, zombie_id: i32, score: f32 },
    HumanKilled { tick: i32, human_id: i32 },
    Ending { tick: i32, won: bool }
}