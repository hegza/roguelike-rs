mod encounter;

use super::*;

pub struct Story;
impl HandleInput for Story {
    fn handle_input(cmd: Command, scene: &mut GameScene) -> bool {
        match scene.story {
            StoryState::CombatEncounter { .. } => {
                return encounter::handle_input(cmd, scene);
            }
            StoryState::OpenTreasure { .. } => {
                let idx = scene.controller.selected_idx(&"story");
                let options = scene.story.options();
                match cmd {
                    Command::MoveSelect(dir) => {
                        match dir {
                            Direction::Down => if idx != options.len() - 1 {
                                scene.controller.set_selected_idx(idx + 1);
                            },
                            Direction::Up => if idx != 0 {
                                scene.controller.set_selected_idx(idx - 1);
                            },
                            _ => {}
                        }
                    }
                    Command::Confirm => {
                    }
                    _ => {}
                }
            }
            StoryState::Final => {}
        }
        false
    }
}
