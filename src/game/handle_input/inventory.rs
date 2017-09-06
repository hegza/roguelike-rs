use super::*;
use game::scenes::GameScene;

impl HandleInput for Inventory {
    fn handle_input(cmd: Command, scene: &mut GameScene) -> bool {
        let idx = scene.controller.selected_idx(&"inventory");

        match cmd {
            Command::MoveSelect(dir) => {
                let inventory = &scene.player.inventory;
                match dir {
                    Direction::Down => {
                        // Get bounds of item in current position
                        let (start, size) = inventory.bounds(idx as i32);
                        // Move cursor below the current item
                        scene
                            .controller
                            .set_selected_idx_safe((start + size) as i32, inventory.capacity() - 1);
                    }
                    Direction::Up => {
                        // Get bounds of item in previous position
                        let (start, _) = inventory.bounds(idx as i32 - 1);
                        // Move cursor to the start of the item in previous position
                        scene
                            .controller
                            .set_selected_idx_safe(start as i32, inventory.capacity() - 1);
                    }
                    _ => {}
                }
            }
            Command::Confirm => {
                // Check if something's selected
                if scene.player.inventory.is_reserved(idx) {
                    let item = scene.player.inventory.take(idx as i32).unwrap();
                    // Check if the item is equipment
                    match item {
                        Item::Equipment(e) => {
                            // Equip it
                            let prev = scene.player.equip(e);
                            if let Some(p) = prev {
                                scene.player.inventory.put(p.into());
                            }
                        }
                        item => {
                            // Put it back into inventory
                            scene.player.inventory.put_at(item, idx);
                        }
                    }
                }
            }
            _ => {}
        }
        false
    }
}
