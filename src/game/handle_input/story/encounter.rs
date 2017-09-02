use rpglib::*;
use game::handle_input::*;
use game::scenes::game_scene::*;
use game::scenes::game_scene::story_option::StoryOption::*;

pub fn handle_input(cmd: Command, scene: &mut GameScene) -> bool {
    if let StoryState::CombatEncounter { .. } = scene.story {
        let idx = scene.controller.selected_idx(&"story");
        let options = scene.story.options();
        match cmd {
            Command::MoveSelect(dir) => match dir {
                Direction::Down => if idx != options.len() - 1 {
                    scene.controller.set_selected_idx(idx + 1);
                },
                Direction::Up => if idx != 0 {
                    scene.controller.set_selected_idx(idx - 1);
                },
                _ => {}
            },
            Command::Confirm => match *&options[idx] {
                Attack => {
                    return true;
                }
                Search => {
                    scene.story = OpenTreasure {
                        items: vec![equipment("Gold", 1, Slot::Hand, vec![]).build().into()],
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
