use super::*;

impl HandleInput for Character {
    fn handle_input(cmd: Command, state: &mut GameScene) -> bool {
        use super::Command::*;
        match cmd {
            MoveSelect(dir) => {
                let player = &state.player;
                let mut all_slots: Vec<&Slot> = player.slots();
                all_slots.sort();

                let cur_idx = state.controller.selected_idx("character");
                if dir == Direction::Down {
                    if cur_idx != all_slots.len() - 1 {
                        state.controller.set_selected_idx(cur_idx + 1);
                    }
                } else if dir == Direction::Up {
                    if cur_idx != 0 {
                        state.controller.set_selected_idx(cur_idx - 1);
                    }
                }
            }
            Confirm => {
                let player = &mut state.player;
                let idx = state.controller.selected_idx("character");

                // Check if there's room in inventory
                let size_of_current = {
                    let current_selection = player.equipment().at(idx);
                    match current_selection {
                        Some(item) => item.size(),
                        None => 0,
                    }
                };
                let inventory_has_space = player.inventory.find_space(size_of_current).is_some();
                if inventory_has_space {
                    if let Some(unequipped_item) = player.unequip(idx) {
                        player.inventory.put(unequipped_item.into());
                    }
                }
            }
            _ => {}
        }
        false
    }
}
