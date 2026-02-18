use std::collections::{VecDeque, HashMap};
use super::unit::{Unit, UnitId};


#[derive(Debug)]
pub struct UnitQueue {
    queue: VecDeque<UnitId>,
}

impl UnitQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Recompute queue order based on speed and unit id
    /// `units` is the game units hashmap
    /*
    pub fn refresh(&mut self, units: &HashMap<UnitId, Unit>, registry: &ClassRegistry) {
        let mut units_vec: Vec<&Unit> = units.values().collect();

        // sort by descending speed, then ascending unit id
        units_vec.sort_by(|a, b| {
            let speed_a = a.speed(registry);
            let speed_b = b.speed(registry);
            speed_b.cmp(&speed_a)           // descending speed
                .then(a.id.cmp(&b.id))      // tie-breaker
        });

        self.queue.clear();
        for unit in units_vec {
            self.queue.push_back(unit.id);
        }
    }
    */

    /// Returns the unit ID whose turn it is now
    pub fn current(&self) -> Option<UnitId> {
        self.queue.front().cloned()
    }

    /// Advances the queue by removing the front unit
    pub fn advance(&mut self) -> Option<UnitId> {
        self.queue.pop_front()
    }

    /// Returns a slice of remaining unit IDs
    pub fn remaining(&self) -> &[UnitId] {
        self.queue.as_slices().0
    }

    /// Removes a unit from the queue
    pub fn remove_unit(&mut self, unit_id: UnitId) {
        self.queue.retain(|&id| id != unit_id);
    }

    /// Clear the queue entirely
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
