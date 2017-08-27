use rpglib::*;

pub struct CombatScene {
    pub combat: Combat,
    pub monster: Monster,
}

impl CombatScene {
    pub fn new(character: &Character, monster: Monster) -> CombatScene {
        CombatScene {
            combat: Combat::new(character, &monster),
            monster: monster,
        }
    }
}
