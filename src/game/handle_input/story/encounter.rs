use rpglib::*;
use game::handle_input::*;
use game::scenes::game_scene::*;
use game::scenes::game_scene::story_option::StoryOption::*;

pub fn handle_input(cmd: Command, scene: &mut GameScene) -> bool {
    if let StoryState::Encounter { .. } = scene.story {
        let option_count = scene.story.options().len();
        let idx = scene
            .controller
            .selected_idx_safe(&"story", option_count - 1);
        match cmd {
            Command::MoveSelect(dir) => match dir {
                Direction::Down => {
                    scene
                        .controller
                        .set_selected_idx_safe(idx as i32 + 1, option_count - 1);
                }
                Direction::Up => {
                    scene
                        .controller
                        .set_selected_idx_safe(idx as i32 - 1, option_count - 1);
                }
                _ => {}
            },
            Command::Confirm => match scene.story.options()[idx] {
                Attack => {
                    return true;
                }
                Search => {
                    scene.story = OpenTreasure {
                        items: vec![equipment("gold", 1, Slot::Hand, vec![]).build().into()],
                    };
                    return true;
                }
                _ => {}
            },
            _ => {}
        }
    } else {
        panic!("encounter::handle_input should not be called while not in encounter mode");
    }
    false
}
