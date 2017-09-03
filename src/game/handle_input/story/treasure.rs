use rpglib::*;
use game::handle_input::*;
use game::scenes::game_scene::*;
use game::scenes::game_scene::story_option::StoryOption::*;

pub fn handle_input(cmd: Command, scene: &mut GameScene) -> bool {
    let options = scene.story.options();
    if let StoryState::OpenTreasure { .. } = scene.story {
        let idx = scene.controller.selected_idx(&"story");
        let option_count = options.len();
        match cmd {
            Command::MoveSelect(dir) => match dir {
                Direction::Down => if idx != option_count - 1 {
                    scene.controller.set_selected_idx(idx + 1);
                },
                Direction::Up => if idx != 0 {
                    scene.controller.set_selected_idx(idx - 1);
                },
                _ => {}
            },
            Command::Confirm => match options[idx] {
                PickUp(item_idx) => {
                    // TODO: take the item from the rewards list and add it to player's inventory
                    let inventory = &mut scene.player.inventory;

                    if let StoryState::OpenTreasure { ref mut items } = scene.story {
                        let item_size = items[idx].size();
                        let place = inventory.find_space(item_size);
                        match place {
                            // Check if there's room in inventory
                            Some(at) => {
                                let item = items.swap_remove(idx);
                                inventory.put_at(item.into(), at);
                            }
                            None => {}
                        }
                    }
                }
                GoEast => {
                    scene.enter_adjacent_room(CompassPoint::East);
                }
                _ => {}
            },
            _ => {}
        }
    } else {
        panic!("treasure::handle_input should not be called while not in treasure mode");
    }
    false
}
