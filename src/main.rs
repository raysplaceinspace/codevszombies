#![allow(dead_code)]

mod collections;
mod evaluation;
mod geometry;
mod model;
mod parser;
mod formatter;
mod simulator;
mod agent;

/**
 * Save humans, destroy zombies!
 **/
fn main() {
    let mut previous_strategy = model::Strategy::new(0);

    // game loop
    let mut tick: i32 = 0;
    loop {
        let world = parser::read_world(tick);
        let strategy = agent::choose(&world, &previous_strategy);
        let action = agent::strategy_to_action(&strategy, &world);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", formatter::format_action(&action)); // Your destination coordinates

        previous_strategy = strategy;
        tick += 1;
    }
}
