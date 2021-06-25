#![allow(dead_code)]

mod collections {
    pub fn min_by_fkey<T, F>(vec: &Vec<T>, selector: F) -> Option<&T>
    where F: Fn(&T) -> f32 {

        let mut result: Option<&T> = None;
        let mut result_value = f32::INFINITY;
        for item in vec.iter() {
            let value = selector(&item);
            if value < result_value {
                result = Some(&item);
                result_value = value;
            }
        }
        result
    }
}

mod geometry {
    #[derive(Clone, Copy)]
    pub struct V2 {
        pub x: f32,
        pub y: f32,
    }

    impl V2 {
        pub fn zero() -> V2 {
            V2 { x: 0.0, y: 0.0 }
        }

        pub fn add(self, other: V2) -> V2 {
            V2 {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }

        pub fn sub(self, other: V2) -> V2 {
            V2 {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }

        pub fn mul(self, multiplier: f32) -> V2 {
            V2 {
                x: self.x * multiplier,
                y: self.y * multiplier,
            }
        }

        pub fn div(self, divisor: f32) -> V2 {
            V2 {
                x: self.x / divisor,
                y: self.y / divisor,
            }
        }

        pub fn diff(b: V2, a: V2) -> V2 {
            V2 {
                x: b.x - a.x,
                y: b.y - a.y,
            }
        }

        pub fn length(self) -> f32 {
            self.length_squared().sqrt()
        }

        pub fn length_squared(self) -> f32 {
            self.x.powf(2.0) + self.y.powf(2.0)
        }

        pub fn distance(a: V2, b: V2) -> f32 {
            V2::diff(b, a).length()
        }

        pub fn distance_squared(a: V2, b: V2) -> f32 {
            V2::diff(b, a).length_squared()
        }

        pub fn distance_to(self, target: V2) -> f32 {
            V2::distance(self, target)
        }

        pub fn distance_to_squared(self, target: V2) -> f32 {
            V2::distance_squared(self, target)
        }

        pub fn towards(self, target: V2, max_step: f32) -> V2 {
            let diff = V2::diff(target, self);
            let distance = diff.length();
            if distance < max_step {
                return target;
            } else if distance > 0.0 {
                return self.add(diff.mul(max_step / distance));
            } else {
                return self;
            }
        }
    }
}

mod model {
    pub use crate::geometry::*;

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
}

mod parser {
    use std::io;
    use crate::model::*;

    macro_rules! parse_input {
        ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
    }

    pub fn read_world() -> World {
        let input_line = read_line();
        let inputs = split_line(&input_line);
        let pos = parse_v2(inputs[0], inputs[1]);

        let input_line = read_line();
        let human_count = parse_input!(input_line, i32);
        let mut humans = Vec::<Human>::new();
        for _ in 0..human_count as usize {
            let input_line = read_line();
            humans.push(parse_human(&input_line));
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        let mut zombies = Vec::<Zombie>::new();
        for _ in 0..zombie_count as usize {
            let input_line = read_line();
            zombies.push(parse_zombie(&input_line));
        }

        World {
            pos,
            humans,
            zombies,
        }
    }

    fn read_line() -> String {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line
    }
    
    fn split_line(input_line: &str) -> Vec<&str> {
        input_line.split(" ").collect::<Vec<&str>>()
    }

    fn parse_v2(x: &str, y: &str) -> V2 {
        V2 {
            x: parse_input!(x, f32),
            y: parse_input!(y, f32),
        }
    }

    fn parse_human(input_line: &str) -> Human {
        let inputs = split_line(input_line);
        Human {
            id: parse_input!(inputs[0], i32),
            pos: parse_v2(inputs[1], inputs[2]),
        }
    }

    fn parse_zombie(input_line: &str) -> Zombie {
        let inputs = split_line(input_line);
        Zombie {
            id: parse_input!(inputs[0], i32),
            pos: parse_v2(inputs[1], inputs[2]),
            next: parse_v2(inputs[3], inputs[4]),
        }
    }
}

mod formatter {
    use crate::model::*;

    pub fn format_action(action: &Action) -> String {
        format!("{} {}", action.target.x, action.target.y)
    }
}

mod simulator {
    use crate::collections;
    pub use crate::model::*;

    pub struct SimulationResult {
        world: World,
    }

    pub fn next(initial: &World, action: &Action) -> SimulationResult {
        let mut world = initial.clone();
        move_zombies(&mut world);
        move_ash(&mut world, &action);
        destroy_zombies(&mut world);
        destroy_humans(&mut world);
        update_zombie_targets(&mut world);
        SimulationResult { world }
    }

    fn move_zombies(world: &mut World) {
        for zombie in world.zombies.iter_mut() {
            zombie.pos = zombie.next;
        }
    }

    fn update_zombie_targets(world: &mut World) {
        for zombie in world.zombies.iter_mut() {
            let closest_human = collections::min_by_fkey(&world.humans, |human| human.pos.distance_to_squared(zombie.pos));
            match closest_human {
                Some(human) => {
                    zombie.next = zombie.pos.towards(human.pos, constants::MAX_ZOMBIE_STEP);
                },
                None => (),
            }
        }
    }

    fn move_ash(world: &mut World, action: &Action) {
        world.pos = world.pos.towards(action.target, constants::MAX_ASH_STEP);
    }

    fn destroy_zombies(world: &mut World) {
        let max_distance_squared = constants::MAX_ASH_KILL_RANGE.powf(2.0);
        let ash_pos = world.pos;
        world.zombies.retain(|zombie| zombie.pos.distance_to_squared(ash_pos) > max_distance_squared);
    }

    fn destroy_humans(world: &mut World) {
        let max_distance_squared = constants::MAX_ZOMBIE_KILL_RANGE.powf(2.0);
        let zombies = &world.zombies;
        let humans = &mut world.humans;
        humans.retain(|human| zombies.iter().any(|zombie| zombie.pos.distance_to_squared(human.pos) <= max_distance_squared));
    }
}

mod agent {
    use crate::collections;
    pub use crate::model::*;

    pub fn choose(world: &World) -> Action {
        Action {
            target: choose_target(world),
        }
    }

    fn choose_target(world: &World) -> V2 {
        let closest_zombie = collections::min_by_fkey(&world.zombies, |zombie| zombie.next.distance_to_squared(world.pos));
        match closest_zombie {
            Some(zombie) => zombie.next,
            None => V2::zero(),
        }
    }
}


/**
 * Save humans, destroy zombies!
 **/
fn main() {

    // game loop
    loop {
        let world = parser::read_world();
        let action = agent::choose(&world);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", formatter::format_action(&action)); // Your destination coordinates
    }
}
