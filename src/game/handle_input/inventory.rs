use super::*;

impl HandleInput for Inventory {
    fn handle_input(cmd: Command, state: &mut GameState) {
        let idx = state.controller.selected_idx(&"inventory");

        match cmd {
            Command::MoveSelect(dir) => {
                let inventory = &state.character.inventory;
                match dir {
                    Direction::Down => {
                        // Get bounds of item in current position
                        let (start, size) = inventory.bounds(idx as i32);
                        if start + size != inventory.capacity() {
                            // Move cursor below the current item
                            state.controller.set_selected_idx(start + size);
                        }
                    }
                    Direction::Up => {
                        // Get bounds of item in previous position
                        let (start, _) = inventory.bounds(idx as i32 - 1);
                        // Move cursor to the start of the item in previous position
                        state.controller.set_selected_idx(start);
                    }
                    _ => {}
                }
            }
            Command::Confirm => {
                // Check if something's selected
                if state.character.inventory.is_reserved(idx) {
                    let item = state.character.inventory.take(idx as i32).unwrap();
                    // Check if the item is equipment
                    match item {
                        Item::Equipment(e) => {
                            // Equip it
                            let prev = state.character.equip(e);
                            if let Some(p) = prev {
                                state.character.inventory.put(p.into());
                            }
                        }
                        i => {
                            state.character.inventory.put_at(i, idx);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
