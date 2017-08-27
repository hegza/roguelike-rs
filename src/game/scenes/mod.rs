mod combat_scene;

pub use self::combat_scene::*;

pub enum Scene {
    Combat(CombatScene),
    OffCombat,
}
