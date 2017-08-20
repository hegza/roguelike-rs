use command::{Command, Direction};
use super::*;

pub trait HandleInput {
    fn handle_input(&mut self, cmd: Command, controller: &mut Controller) -> StateChange;
}

impl HandleInput for Inventory {
    fn handle_input(&mut self, cmd: Command, ctrl: &mut Controller) -> StateChange {
        use self::Direction::*;
        match cmd {
            Command::MoveSelect(dir) => {
                if dir == Down {
                    let idx = &mut ctrl.inventory;
                    // Get bounds of item in current position
                    let (start, size) = self.bounds(*idx as i32);
                    if start + size != self.capacity() {
                        // Move cursor below the current item
                        *idx = start + size;
                    }
                } else if dir == Up {
                    let idx = &mut ctrl.inventory;
                    // Get bounds of item in previous position
                    let (start, _) = self.bounds(*idx as i32 - 1);
                    // Move cursor to the start of the item in previous position
                    *idx = start;
                }
                StateChange::Still
            }
            Command::Confirm => {
                let idx = &mut ctrl.inventory;
                if let Some(item) = self.take(*idx as i32) {
                    match item {
                        Item::Equipment(equip) => {
                            // Equip the item
                            return StateChange::Equip(equip);
                        }
                        _ => {
                            // Put the item back into the inventory
                            self.put_at(item, *idx);
                        }
                    }
                };

                StateChange::Still
            }
            _ => StateChange::Still,
        }
    }
}

impl HandleInput for Character {
    fn handle_input(&mut self, cmd: Command, ctrl: &mut Controller) -> StateChange {
        match cmd {
            Command::MoveSelect(dir) => {
                let mut all_slots: Vec<&ItemSlot> =
                    self.equipped_items().iter().map(|(k, _)| k).collect();
                all_slots.sort();

                let cur_idx = all_slots.iter()
                    .position(|x| *x == &ctrl.equipment)
                    .expect("a non-existing slot should not be selected");
                if dir == UIDirection::Down {
                    if cur_idx != all_slots.len() - 1 {
                        ctrl.equipment = *all_slots[cur_idx + 1];
                    }
                } else if dir == UIDirection::Up {
                    if cur_idx != 0 {
                        ctrl.equipment = *all_slots[cur_idx - 1];
                    }
                }
                StateChange::Still
            }
            Command::Confirm => {
                let slot = &ctrl.equipment;
                if let Some(unequipped_item) = self.unequip(slot) {
                    self.inventory.put(unequipped_item.into());
                }
                StateChange::Still
            }
            _ => StateChange::Still,
        }
    }
}
