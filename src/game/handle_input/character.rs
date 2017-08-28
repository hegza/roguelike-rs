use super::*;

impl HandleInput for Character {
    fn handle_input(cmd: Command, state: &mut GameState) {
        use super::Command::*;
        match cmd {
            MoveSelect(dir) => {
                let character = &state.character;
                let mut all_slots: Vec<&Slot> = character.slots();
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
                let character = &mut state.character;
                let idx = state.controller.selected_idx("character");
                if let Some(unequipped_item) = character.unequip(idx) {
                    character.inventory.put(unequipped_item.into());
                }
            }
            _ => {}
        }
    }
}
