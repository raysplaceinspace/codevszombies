use std::fmt;
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

        Verifier::log_position_equivalent(&self.previous.ash, &self.predicted.ash, &world.ash);

        for initial in self.previous.zombies.values() {
            let predicted = self.predicted.zombies.get(&initial.id);
            let current = world.zombies.get(&initial.id);
            Verifier::log_alive_equivalent(initial, predicted, current);
            if predicted.is_some() && current.is_some() {
                Verifier::log_position_equivalent(initial, predicted.unwrap(), current.unwrap());
            }
        }

        for initial in self.previous.humans.values() {
            let predicted = self.predicted.humans.get(&initial.id);
            let current = world.humans.get(&initial.id);
            Verifier::log_alive_equivalent(initial, predicted, current);
            if predicted.is_some() && current.is_some() {
                Verifier::log_position_equivalent(initial, predicted.unwrap(), current.unwrap());
            }
        }
    }

    pub fn log_alive_equivalent<T>(initial: &T, predicted: Option<&T>, current: Option<&T>)
        where T : fmt::Display {

        if predicted.is_some() != current.is_some() {
            eprintln!("Mispredicted {}: {} -> {}", initial, Verifier::format_alive(predicted), Verifier::format_alive(current));
        }
    }

    pub fn log_position_equivalent<T>(initial: &T, predicted: &T, current: &T)
        where T : fmt::Display, T : Positioned {
        
        const PRECISION: f32 = 1.0;

        let distance = predicted.pos().distance_to(current.pos());
        if distance > PRECISION {
            eprintln!("Mispredicted {}: {} -> {}", initial, predicted.pos(), current.pos());
        }
    }

    fn format_alive<T>(v: Option<T>) -> &'static str {
        match v.is_some() {
            true => "alive",
            false => "dead",
        }
    }
}