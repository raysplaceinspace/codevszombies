use super::simulator;

use super::model::*;

pub struct Verifier {
    previous: World,
    predicted: World,
}

impl Verifier {
    pub fn new() -> Verifier {
        Verifier {
            previous: World::new(),
            predicted: World::new(),
        }
    }

    pub fn next(&mut self, world: &World, action: &Action) {
        self.previous = world.clone();

        let mut predicted = world.clone();
        simulator::next(&mut predicted, action);
        self.predicted = predicted;
    }

    pub fn log_prediction_error(&self, world: &World) {
        if world.tick <= 0 { return; } // No predicting for first tick

        let mut error_found = false;

        for zombie_id in self.previous.zombies.keys() {
            let predicted = self.predicted.zombies.get(zombie_id);
            let current = world.zombies.get(zombie_id);
            if predicted.is_some() != current.is_some() {
                eprintln!("Mispredicted zombie {}: {} -> {}", zombie_id, format_alive(predicted), format_alive(current));
                error_found = true;
            }
        }

        for human_id in self.previous.humans.keys() {
            let predicted = self.predicted.zombies.get(human_id);
            let current = world.zombies.get(human_id);
            if predicted.is_some() != current.is_some() {
                eprintln!("Mispredicted human {}: {} -> {}", human_id, format_alive(predicted), format_alive(current));
                error_found = true;
            }
        }

        if !error_found {
            eprintln!("Verifier: no prediction errors found");
        }
    }
}

fn format_alive<T>(v: Option<T>) -> &'static str {
    match v.is_some() {
        true => "1",
        false => "0",
    }
}