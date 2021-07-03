#![allow(dead_code)]

mod agent;
mod collections;
mod evaluation;
mod formatter;
mod geometry;
mod model;
mod parser;
mod simulator;
mod verifier;

use model::*;
use verifier::Verifier;

/**
 * Save humans, destroy zombies!
 **/
fn main() {
    let mut previous_strategy = Strategy::new(0);

    // game loop
    let mut tick: i32 = 0;
    let mut verifier = Verifier::new();
    loop {
        let world = parser::read_world(tick);
        let strategy = agent::choose(&world, &previous_strategy);
        let action = agent::strategy_to_action(&strategy, &world);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", formatter::format_action(&action)); // Your destination coordinates

        verifier.log_prediction_error(&world);

        // Update for next tick
        verifier.next(&world, &action);
        previous_strategy = strategy;
        tick += 1;
    }
}
