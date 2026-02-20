#![allow(dead_code, warnings)]

use std::collections::HashMap;
use super::unit::{Unit, UnitId};
use super::grid::Grid;
use super::turn::UnitQueue;
use crate::core::geom::{Direction, Position, Path};
use crate::core::player::{PlayerId, Player};
use crate::core::combat::AttackId;
use crate::core::unit::UnitClassId;

const SNAPSHOT_FREQUENCY: Option<u16> = Some(5);  // magic constant

pub struct Game {
    players: HashMap<PlayerId, Player>,
    
    units: HashMap<UnitId, Unit>,
    grid: Grid,
    queue: UnitQueue,

    turn_number: u32,
    history: Vec<Turn>,
    snapshots: Vec<GameSnapshot>,
}

impl Game {
    pub fn new(
        players: HashMap<PlayerId, Player>,
        units: HashMap<UnitId, Unit>,
        grid: Grid,
        queue: UnitQueue,
    ) -> Self {
        Self {
            players,
            units,
            grid,
            queue,
            turn_number: 0,
            history: Vec::new(),
            snapshots: Vec::new(),
        }
    }

    /// Single mutation entry point
    pub fn apply_proposed(
        &mut self,
        player_id: PlayerId,
        action: ProposedAction,
    ) -> Result<ResolvedAction, GameError> {
        self.validate_turn(player_id)?;

        let resolved = self.resolve_action(player_id, action)?;

        self.record_action(resolved.clone());

        self.maybe_snapshot();

        Ok(resolved)
    }
}


impl Game {
    fn resolve_action(
        &mut self,
        player_id: PlayerId,
        action: ProposedAction,
    ) -> Result<ResolvedAction, GameError> {
        match action {
            ProposedAction::Move { path } => {
                self.resolve_move(player_id, path)
            }
            ProposedAction::Attack { target, attack } => {
                self.resolve_attack(player_id, target, attack)
            }
            ProposedAction::Spawn { unit, position } => {
                unimplemented!();
            }
            ProposedAction::EndTurn => {
                self.resolve_end_turn()
            }
            _ => unimplemented!()
        }
    }
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
struct GameSnapshot {
    units: HashMap<UnitId, Unit>,
    grid: Grid,
    queue: UnitQueue,

    turn_number: u32,
}

struct Turn {
    turn_number: u32,
    actions: Vec<ActionLog>,
}

struct ActionLog {
    action: ResolvedAction,
    changes: Vec<Change>,
}

// Broadcasted action
#[derive(Clone)]
pub enum ResolvedAction {
    Move {
        unit_id: UnitId,
        path: Path,
    },
    Attack {
        attacker: UnitId,
        target: UnitId,
        attack: AttackId,
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

enum Change {
    UnitModified {
        unit_id: UnitId,
        before: Unit,
    },
    UnitRemoved {
        unit: Unit, // full snapshot so we can resurrect
    },
    UnitInserted {
        unit_id: UnitId,
        position: Position,
        unit_type: UnitClassId,
    },
    QueueModified {
        previous: UnitQueue,
    },
}
