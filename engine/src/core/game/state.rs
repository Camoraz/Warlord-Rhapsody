#![allow(dead_code, warnings)]

use std::collections::HashMap;
use crate::core::unit::{Unit, UnitId};
use crate::core::grid::Grid;
use crate::core::turn::UnitQueue;
use crate::core::geom::{Direction, Position, Path};
use crate::core::player::{PlayerId, Player};
use crate::core::combat::AttackId;
use crate::core::unit::UnitClassId;


pub struct Game {
    players: HashMap<PlayerId, Player>,
    
    units: HashMap<UnitId, Unit>,
    grid: Grid,
    queue: UnitQueue,

    round_number: u32,  // every queue reset
    turn_number: u32,

    curr_turn: Turn,
    history: Vec<Turn>,  // turn history
    snapshots: Vec<RoundSnapshot>,  // round snapshots
}

impl Game {
    fn apply_resolution(&mut self, change: ResolvedChange) {
        match change {
            ResolvedChange::Move { unit_id, path } => {
                self.move_unit(unit_id, path);
            }
            _ => unimplemented!()
        }
    }

    pub fn units_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values()
    }

    pub fn move_unit(&mut self, unit_id: UnitId, path: Path) {
        let unit = self.units.get_mut(&unit_id).expect("Invalid unit_id");

        unit.change_pos(path.end());  // Update unit
        self.grid.move_occupancy(path.start(), path.end());  // Update grid

        // Log the change in the current turn
        self.curr_turn.log_change(ResolvedChange::Move {
            unit_id,
            path,
        });
    }

    fn update_turn(&mut self, next_unit: UnitId) {
        let old_turn = std::mem::replace(
                &mut self.curr_turn,
                Turn::new(self.turn_number + 1, next_unit)
            );
            self.history.push(old_turn);

            self.turn_number += 1;
    }

    pub fn commit_turn(&mut self) {
        // Attempt to get the next unit
        let next_unit_opt = self.queue.next_unit();

        match next_unit_opt {
            Some(next_unit) => {
                // Normal turn commit
                self.update_turn(next_unit);
            }
            None => {
                // End of round
                self.snapshot_round();      // Save the full game state for this round
                self.queue.reset_from_game(&self.units);         // Reset the queue for the next round
                self.round_number += 1;     // Increment round counter

                // Start a new turn with the first unit in the new queue
                let first_unit = self.queue.next_unit()
                    .expect("Queue must have units after reset");

                self.update_turn(first_unit);
            }
        }
    }

    pub fn snapshot_round(&mut self) {
        let snap = RoundSnapshot {
            players: self.players.clone(),
            units: self.units.clone(),
            grid: self.grid.clone(),
            queue: self.queue.clone(),
            round_number: self.round_number,
        };
        self.snapshots.push(snap);
    }

    pub fn get_unit(&self, unit_id: UnitId) -> Option<&Unit> {
        self.units.get(&unit_id)
    }
}

#[derive(Clone)]
struct RoundSnapshot {
    players: HashMap<PlayerId, Player>,
    units: HashMap<UnitId, Unit>,
    grid: Grid,
    queue: UnitQueue,

    round_number: u32,
}

/// This is what is broadcasted for each client on server resolution.
/// It has enough information for the client to rollback when visualizing changes.
/// More than one ResolvedChange may be sent per turn.
#[derive(Clone)]
pub enum ResolvedChange {
    Move {
        unit_id: UnitId,
        path: Path,
    },
    Attack {
        attacker: UnitId,
        target: UnitId,
        attack: AttackId,
        // should include attack resolution
        // and previous health to restore
    },
    Ability {
        unit_id: UnitId,
    },
    Spawn {
        unit: Unit,
        position: Position,
    },
    EndTurn,
}

struct Turn {
    turn_number: u32,
    unit: UnitId,  // who moves
    changes: Vec<ResolvedChange>,  // what happened
}

impl Turn {
    fn new(turn_number: u32, unit: UnitId) -> Self {
        Turn {
            turn_number,
            unit,
            changes: Vec::new(),
        }
    }

    fn log_change(&mut self, change: ResolvedChange) {
        self.changes.push(change);
    }
}

/// Clients propose actions through this protocol
pub enum ProposedAction {
    Move {
        path: Path,
    },
    Attack {
        target: UnitId,
        attack: AttackId,
    },
    Ability {
        //ability: AbilityId,
        target: Option<Unit>,
    },
    Spawn {
        unit: UnitId,
        position: Position,
    },
    EndTurn,
}

#[derive(Debug)]
pub enum GameError {
    NotYourTurn,
    InvalidUnit,
    InvalidPath,
    OutOfRange,
    NotEnoughResources,
    IllegalAction,
}
