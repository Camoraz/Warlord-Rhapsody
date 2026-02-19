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
    turns: Vec<Turn>,

    snapshots: Vec<GameSnapshot>,
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
    active_unit: UnitId,
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
        final_position: Position,
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
enum ProposedAction {
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
