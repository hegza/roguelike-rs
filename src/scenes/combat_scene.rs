use rpglib::*;

pub struct CombatScene {
    pub combat: Combat,
    pub monster: Monster,
}

impl CombatScene {
    pub fn new(monster: Monster) -> CombatScene {
        CombatScene {
            combat: Combat::new(),
            monster: monster,
        }
    }
}
