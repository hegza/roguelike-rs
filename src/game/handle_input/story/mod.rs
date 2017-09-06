mod encounter;
mod treasure;

use super::*;

pub struct Story;
impl HandleInput for Story {
    fn handle_input(cmd: Command, scene: &mut GameScene) -> bool {
        match scene.story {
            StoryState::Encounter { .. } => {
                return encounter::handle_input(cmd, scene);
            }
            StoryState::OpenTreasure { .. } => {
                return treasure::handle_input(cmd, scene);
            }
            StoryState::Final => {}
        }
        false
    }
}
