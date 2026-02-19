use smallvec::SmallVec;

pub struct AttackId(u16);

struct AttackDefinition {
    base_damage: i32,  // negative for heal
    damage_type: DamageProfile,
    range: AttackRange,
    aoe: AoePattern,
    effects: SmallVec<Effect, 3>,
    target: TargetFilter,
}

enum TargetFilter {
    Enemy,
    Ally,
    Any,
}

struct DamageProfile {
    pierce: u8,
    blunt: u8,
    slash: u8,
}

enum AoePattern {
    Single,
    Sides,  // left and right
    Radius(u8),  // affects everyone except self
    Line(u8),  // affects behind in line
    Cone(u8),
}

struct AttackRange {
    innerRadius: u8,
    outerRadius: u8,
}
