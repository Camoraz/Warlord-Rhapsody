use core::alloc;
use std::collections::{HashMap, VecDeque};
use crate::core::unit::{Unit, UnitId};
use crate::core::game::Game; // to access units for speed

#[derive(Clone)]
pub struct UnitQueue {
    queue: VecDeque<UnitId>,
}

impl UnitQueue {
    /// Create a new queue from a slice of units
    pub fn new(units: &[Unit]) -> Self {
        let mut sorted_units: Vec<UnitId> = units.iter()
            .map(|u| u.id)
            .collect();

        // Sort by speed descending, then UnitId ascending
        sorted_units.sort_unstable_by(|a, b| {
            let a_speed = units.iter().find(|u| u.id == *a).unwrap().speed();
            let b_speed = units.iter().find(|u| u.id == *b).unwrap().speed();
            b_speed.cmp(&a_speed).then(a.cmp(b))
        });

        Self {
            queue: VecDeque::from(sorted_units),
        }
    }

    /// Pop the next unit from the front
    pub fn next_unit(&mut self) -> Option<UnitId> {
        self.queue.pop_front()
    }

    /// Peek at the next unit without removing it
    pub fn peek(&self) -> Option<UnitId> {
        self.queue.front().copied()
    }

    /// Push a unit to the back (e.g., for delayed effects or spawns mid-round)
    pub fn push_unit(&mut self, unit_id: UnitId) {
        self.queue.push_back(unit_id);
    }

    /// Recompute the queue ordering from the remaining units using their current speed
    pub fn recompute_from_units(&mut self, units: &HashMap<UnitId, Unit>) {
        let mut remaining: Vec<UnitId> = self.queue.iter().copied().collect();
        remaining.sort_unstable_by(|a, b| {
            let a_speed = units.get(a).unwrap().speed();
            let b_speed = units.get(b).unwrap().speed();
            b_speed.cmp(&a_speed).then(a.cmp(b))
        });
        self.queue = VecDeque::from(remaining);
    }

    /// Reset the queue for a new round using all units in the game
    pub fn reset_from_game(&mut self, units: &HashMap<UnitId, Unit>) {
        let mut all_units: Vec<UnitId> = units.keys().copied().collect();

        all_units.sort_unstable_by(|a, b| {
            let a_speed = units.get(a).unwrap().speed();
            let b_speed = units.get(b).unwrap().speed();
            b_speed.cmp(&a_speed).then(a.cmp(b))
        });
        self.queue = VecDeque::from(all_units);
    }
}
