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

    // game loop
    let mut tick: i32 = 0;
    loop {
        let world = parser::read_world(tick);
        let action = agent::choose(&world);
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        println!("{}", formatter::format_action(&action)); // Your destination coordinates

        tick += 1;
    }
}
