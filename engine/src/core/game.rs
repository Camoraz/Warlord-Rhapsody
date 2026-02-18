use std::collections::HashMap;
use super::unit::{Unit, UnitId};
use super::grid::Grid;
use super::turn::UnitQueue;
use crate::core::geom::{Direction, Position};

struct PlayerId(u32);
struct AttackId(u16);

struct Player {
    name: String,
}

pub struct Game {
    players: HashMap<PlayerId, Player>,
    units: HashMap<UnitId, Unit>,
    state: Grid,
    history: Vec<Turn>,
    queue: UnitQueue,
    turn_number: u32,
}


struct Turn {
    turn_number: u32,
    unit_id: UnitId,
    action: ResolvedAction,
}

// This is broadcasted and saved in history
enum ResolvedAction {
    Move {
        unit_id: UnitId,
        path: Vec<Direction>,
        final_position: Position,
    },
    Attack {
        attacker: UnitId,
        target: UnitId,
        attack: AttackId,
    },
    Ability {
        unit_id: UnitId,
        //ability: AbilityId,
    },
    Spawn {
        unit_id: UnitId,
        //unit_type: unit::UnitType,
        position: Position,
    },
    EndTurn,
}


// Clients propose actions through this protocol
enum ProposedAction {
    Move {
        path: Vec<Direction>,
    },
    Attack {
        target: UnitId,
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
