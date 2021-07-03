use std::io;
use std::collections::HashMap;
use super::model::*;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn read_world(tick: i32) -> World {
    let input_line = read_line();
    let inputs = split_line(&input_line);
    let pos = parse_v2(inputs[0], inputs[1]);

    let input_line = read_line();
    let human_count = parse_input!(input_line, i32);
    let mut humans = HashMap::<i32, Human>::new();
    for _ in 0..human_count as usize {
        let input_line = read_line();
        let human = parse_human(&input_line);
        humans.insert(human.id, human);
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let zombie_count = parse_input!(input_line, i32);
    let mut zombies = HashMap::<i32, Zombie>::new();
    for _ in 0..zombie_count as usize {
        let input_line = read_line();
        let zombie = parse_zombie(&input_line);
        zombies.insert(zombie.id, zombie);
    }

    World {
        tick,
        ash: Ash { pos },
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