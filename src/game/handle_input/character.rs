use super::*;

impl HandleInput for Character {
    fn handle_input(cmd: Command, state: &mut GameScene) -> bool {
        use super::Command::*;
        match cmd {
            MoveSelect(dir) => {
                let player = &state.player;
                let slot_count = player.slots().len();

                let cur_idx = state.controller.selected_idx("character");
                if dir == Direction::Down {
                    state
                        .controller
                        .set_selected_idx_safe(cur_idx as i32 + 1, slot_count - 1);
                } else if dir == Direction::Up {
                    state
                        .controller
                        .set_selected_idx_safe(cur_idx as i32 - 1, slot_count - 1);
                }
            }
            Confirm => {
                let player = &mut state.player;
                let idx = state.controller.selected_idx("character");

                // Check if there's room in inventory
                if let Some(size_of_current) = {
                    let current_selection = player.equipment().at(idx);
                    match current_selection {
                        Some(item) => Some(item.size()),
                        None => None,
                    }
                } {
                    let inventory_has_space =
                        player.inventory.find_space(size_of_current).is_some();
                    if inventory_has_space {
                        if let Some(unequipped_item) = player.unequip(idx) {
                            player.inventory.put(unequipped_item.into());
                        }
                    }
                }
            }
            _ => {}
        }
        false
    }
}
