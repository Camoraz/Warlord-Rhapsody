use super::geom::Position;
use super::player::PlayerId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnitClassId(u32);  // Set at runtime with
                              // HashMap<UnitClassId, UnitDefinition>

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]                       
pub struct UnitId(u32);

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
    pub id: u32,
    pub owner: PlayerId,
    pub class: UnitClassId,
    pub health: u32,
    pub actions: Vec<ActionPoint>,
    pub position: Position,
    //pub effects: Vec<Effect>,
}

impl Unit {
    // Compute the current effective speed of the unit,
    // including base speed and active effects/modifiers
    // pub fn speed(&self, registry: &ClassRegistry) -> u8 {
        // Get base speed from the unit definition
        //let base = registry.classes[&self.class].base_speed as f32;

        //let mut additive: f32 = 0.0;
        //let mut multiplier: f32 = 1.0;

        // Apply all effects
        // for effect in &self.effects {
            // additive += effect.speed_additive();
            // multiplier *= effect.speed_multiplier();
        // }

        // let effective = (base + additive) * multiplier;

        // effective.max(1.0).min(255.0) as u8
    //     1.0
    // }
}


pub enum ActionPoint {
    Move,
    Attack,
    Ability,
    MoveOrAttack,
    Wildcard,
    Enclosure(Vec<ActionPoint>)
}

