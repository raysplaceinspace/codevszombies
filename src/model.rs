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

pub trait Positioned {
    fn pos(&self) -> V2;
}

#[derive(Clone)]
pub struct Human {
    pub id: i32,
    pub pos: V2,
}

impl Positioned for Human {
    fn pos(&self) -> V2 {
        self.pos
    }
}

#[derive(Clone)]
pub struct Zombie {
    pub id: i32,
    pub pos: V2,
    pub next: V2,
}

impl Positioned for Zombie {
    fn pos(&self) -> V2 {
        self.pos
    }
}

#[derive(Clone)]
pub struct Ash {
    pub pos: V2,
}

impl Positioned for Ash {
    fn pos(&self) -> V2 {
        self.pos
    }
}

#[derive(Clone)]
pub struct World {
    pub tick: i32,
    pub ash: Ash,
    pub humans: HashMap<i32, Human>,
    pub zombies: HashMap<i32, Zombie>,
}

impl World {
    pub fn new() -> World {
        World {
            tick: 0,
            ash: Ash { pos: V2::zero() },
            humans: HashMap::new(),
            zombies: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct Action {
    pub target: V2,
}

#[derive(Clone)]
pub enum Milestone {
    KillZombie { zombie_id: i32 },
    ProtectHuman { human_id: i32 },
    MoveTo { target: V2 },
}

impl Milestone {
    pub fn is_move(&self) -> bool {
        match self { Milestone::MoveTo{..} => true, _ => false }
    }
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

    pub fn seed(&self, id: i32) -> Strategy {
        Strategy {
            id,
            milestones: self.milestones.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.milestones.len() == 0
    }
}

#[derive(Clone)]
pub enum Event {
    ZombieKilled { tick: i32, zombie_id: i32, score: f32 },
    HumanKilled { tick: i32, human_id: i32 },
    Won { tick: i32, num_humans: usize },
    Lost { tick: i32, num_zombies: usize },
}

impl Event {
    pub fn is_ending(&self) -> bool {
        match &self {
            Event::Won{..} => true,
            Event::Lost{..} => true,
            _ => false,
        }
    }
}