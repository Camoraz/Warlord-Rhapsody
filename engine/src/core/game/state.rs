#![allow(dead_code, warnings)]

use std::collections::HashMap;
use crate::core::unit::{Unit, UnitId};
use crate::core::grid::Grid;
use crate::core::turn::UnitQueue;
use crate::core::geom::{Direction, Path, Position, position};
use crate::core::player::{PlayerId, Player};
use crate::core::combat::AttackId;
use crate::core::unit::UnitClassId;


/// Game is divided into rounds and turns.
/// Each round, all units from the queue have one turn.
/// One turn can have multiple requests (move, attack...).
/// The first turn of each round, this is a special turn where no units move,
/// but instead the clients request to spawn n units and, if accepted,
/// the new queue is constructed and the round proceeds as usual.
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

struct Turn {
    turn_number: u32,
    phase: RoundPhase,  // spawn phase, unit phase
    changes: Vec<ResolvedChange>,  // what happened
}

impl Game {
    pub fn new(players: HashMap<PlayerId, Player>, grid: Grid) -> Self {
        Game {
            players,
            units: HashMap::new(),
            grid,
            queue: UnitQueue::new(),
            round_number: 1,
            turn_number: 1,
            curr_turn: Turn::new(1, ),
            history: Vec::new(),
            snapshots: Vec::new(),
        }
    }

    fn next_unit_id(&self) -> Option<UnitId> {
        self.units
            .keys()
            .copied()
            .max()
            .map(|max_id| max_id.next())
            .unwrap_or(Some(UnitId(0)))
    }

    fn apply_resolution(&mut self, change: ResolvedChange) {
        match change {
            ResolvedChange::Move { unit_id, path } => {
                self.move_unit(unit_id, path);
            },
            ResolvedChange::Spawn { unit, owner, position } => {
                self.spawn_unit(unit, position, owner);
            }
            _ => unimplemented!()
        }
    }

    pub fn units_iter(&self) -> impl Iterator<Item = &Unit> {
        self.units.values()
    }

    pub fn spawn_unit(&mut self, unit_class: UnitClassId, pos: Position, owner: PlayerId) {
        let new_unit_id: UnitId = self.next_unit_id().unwrap();
        
        let new_unit = Unit::new(
            unit_class,
            owner,
            pos,
            new_unit_id);
        
        self.units.insert(new_unit_id, new_unit);
        self.grid.set_occupancy(pos, Some(new_unit_id));
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
        // Move current turn into history
        let old_turn = std::mem::replace(
            &mut self.curr_turn,
            Turn::new(
                self.turn_number + 1,
                RoundPhase::SpawnPhase // dummy, will overwrite
            ),
        );

        let finished_phase = old_turn.phase.clone();

        self.history.push(old_turn);
        self.turn_number += 1;

        match &self.history.last().unwrap().phase {
            RoundPhase::SpawnPhase => {
                // Spawn phase just ended

                // Build queue using updated units (including spawns)
                self.queue.reset_from_game(&self.units);

                // First unit of round
                let first_unit = self.queue
                    .next_unit()
                    .expect("Queue must contain at least one unit");

                self.curr_turn.phase = RoundPhase::UnitTurn { unit: first_unit };
            }

            RoundPhase::UnitTurn { .. } => {
                // Normal unit turn ended

                match self.queue.next_unit() {
                    Some(next_unit) => {
                        // Continue same round
                        self.curr_turn.phase = RoundPhase::UnitTurn { unit: next_unit };
                    }
                    None => {
                        // End of round
                        self.snapshot_round();

                        self.round_number += 1;

                        // Start next round in spawn phase
                        self.curr_turn.phase = RoundPhase::SpawnPhase;
                    }
                }
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
        unit: UnitClassId,
        owner: PlayerId,
        position: Position,
    },
    EndTurn,
}

impl Turn {
    pub fn new(turn_number: u32, phase: RoundPhase) -> Self {
        Turn {
            turn_number,
            phase,
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

#[derive(Clone)]
pub enum RoundPhase {
    SpawnPhase,                 // special first "turn"
    UnitTurn { unit: UnitId },  // normal turn
}