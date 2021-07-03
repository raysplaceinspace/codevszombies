#![allow(dead_code)]

const SCORE_SHEET_SIZE: usize = 10;

mod agent;
mod collections;
mod evaluation;
mod formatter;
mod geometry;
mod milestones;
mod model;
mod mutations;
mod parser;
mod rollouts;
mod simulator;
mod verifier;

use rand;
use rand::Rng;
use rand::prelude::ThreadRng;

use model::*;
use evaluation::ScoreParams;
use verifier::Verifier;

/**
 * Save humans, destroy zombies!
 **/
fn main() {
    let mut rng = rand::thread_rng();

    let score_sheet = ScoreParams::gen_sheet(&mut rng);
    let mut previous_strategies = Vec::<Strategy>::new();

    // game loop
    let mut tick: i32 = 0;
    let mut verifier = Verifier::new();
    loop {
        let world = parser::read_world(tick);
        let strategies = agent::choose(&world, &score_sheet, previous_strategies, &mut rng);
        let action = rollouts::strategy_to_action(&strategies[0], &world);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", &action); // Your destination coordinates

        verifier.log_prediction_error(&world);

        // Update for next tick
        verifier.next(&world, &action);
        previous_strategies = strategies;
        tick += 1;
    }
}
