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
    pub pos: V2,
    pub humans: Vec<Human>,
    pub zombies: Vec<Zombie>,
}

#[derive(Clone)]
pub struct Action {
    pub target: V2,
}