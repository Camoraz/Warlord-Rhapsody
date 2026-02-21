use tokio::io::unix::AsyncFdTryNewError;

use crate::core::game::state::ProposedAction;

use super::geom::Position;
use super::player::PlayerId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnitClassId(u32);  // Set at runtime with
                              // HashMap<UnitClassId, UnitDefinition>
impl UnitClassId { pub fn new(val: u32) -> Self { UnitClassId(val) }}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]              
pub struct UnitId(pub u32);

impl UnitId {
    pub fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(UnitId)
    }
}

pub struct UnitDefinition {
    pub name: String,
    pub base_health: i32,
    pub defense: f32,
    //pub attacks: Vec<AttackDefinition>,
    pub actions: Vec<ActionPoint>,
    // pub abilities: Vec<Ability>,
    pub base_speed: u8,
}

#[derive(Clone)]
pub struct Unit {
    pub id: UnitId,
    pub owner: PlayerId,
    pub class: UnitClassId,
    pub health: u32,
    pub actions: Vec<ActionPoint>,
    pub position: Position,
    //pub effects: Vec<Effect>,
}

impl Unit {
    pub fn new(class: UnitClassId, owner: PlayerId, pos: Position, id: UnitId) -> Self {
        Unit {
            id: id,
            owner: owner,
            class: class,
            health: u32::default(),  // This needs to be changed
            actions: Vec::new(),
            position: pos,
        }
    }

    pub fn get_pos(&self) -> Position {
        self.position
    }

    pub fn get_id(&self) -> UnitId {
        self.id
    }

    pub fn change_pos(&mut self, new_pos: Position) {
        self.position = new_pos;
    }

    // Compute the current effective speed of the unit,
    // including base speed and active effects/modifiers
    pub fn speed(&self) -> u8 {  // take arg , registry: &ClassRegistry
        // Get base speed from the unit definition
        // let base = registry.classes[&self.class].base_speed as f32;

        let mut additive: f32 = 0.0;
        let mut multiplier: f32 = 1.0;

        // Apply all effects
        // for effect in &self.effects {
            // additive += effect.speed_additive();
            // multiplier *= effect.speed_multiplier();
        // }

        // let effective = (base + additive) * multiplier;

        // effective.max(1.0).min(255.0) as u8
        1
    }
}

#[derive(Debug, Clone)]
pub enum ActionPoint {
    Move,
    Attack,
    Ability,
    MoveOrAttack,
    Wildcard,
    Enclosure(Vec<ActionPoint>)
}

