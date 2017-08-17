use rpglib::*;
use super::render::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::*;
use super::ui::{Command, View};
use super::ui::Direction as UIDirection;

pub struct Game {
    pub last_key: char,
    character: Character,
    pub controller: Controller,
}

pub struct Controller {
    pub inventory: usize,
    pub equipment: ItemSlot,
    pub focus: usize, // Id of view
                      // TODO: focus() -> &View
}

impl Controller {
    fn new() -> Controller {
        Controller {
            inventory: 0,
            equipment: ItemSlot::MainHand,
            focus: 0,
        }
    }
}

impl Game {
    pub fn new() -> Game {
        let attributes = hashmap![Attribute::MaxLife => 8, Attribute::Damage => 1];
        let mut my_character = Character::new("hegza", &attributes, 12);
        let item_1 = BaseItem {
            slot: ItemSlot::MainHand,
            english_name: "Long Sword".to_owned(),
            implicit_effects: vec![],
            size: 4,
        };
        let item_2 = BaseItem {
            slot: ItemSlot::OffHand,
            english_name: "Stone".to_owned(),
            implicit_effects: vec![],
            size: 3,
        };
        let item_3 = BaseItem {
            slot: ItemSlot::MainHand,
            english_name: "Short Sword".to_owned(),
            implicit_effects: vec![],
            size: 2,
        };
        my_character.inventory.put(item_1.into());
        my_character.inventory.put(item_2.into());
        my_character.inventory.put(item_3.into());

        Game {
            last_key: ' ',
            character: my_character,
            controller: Controller::new(),
        }
    }

    fn handle_command(&mut self, cmd: Command) -> bool {
        match cmd {
            Command::Quit => false,
            Command::Nav(dir) => {
                match dir {
                    UIDirection::Right => {
                        self.controller.focus += 1;
                    }
                    UIDirection::Left => {
                        // TODO: check upper bound
                        if self.controller.focus > 0 {
                            self.controller.focus -= 1;
                        }
                    }
                    _ => {}
                }
                true
            }
            Command::MoveSelect(dir) => {
                if self.controller.focus == self.character.inventory.id() {
                    let idx = &mut self.controller.inventory;
                    if dir == UIDirection::Down {
                        let inventory = &self.character.inventory;
                        // Get bounds of item in current position
                        let (start, size) = inventory.bounds(*idx as i32);
                        if start + size != inventory.capacity() {
                            // Move cursor below the current item
                            *idx = start + size;
                        }
                    } else if dir == UIDirection::Up {
                        let inventory = &self.character.inventory;
                        // Get bounds of item in previous position
                        let (start, _) = inventory.bounds(*idx as i32 - 1);
                        // Move cursor to the start of the item in previous position
                        *idx = start;
                    }
                } else if self.controller.focus == self.character.id() {
                    let mut all_slots: Vec<&ItemSlot> =
                        self.character.equipped_items().iter().map(|(k, _)| k).collect();
                    all_slots.sort();

                    let cur_idx =
                        all_slots.iter().position(|x| *x == &self.controller.equipment).unwrap();
                    if dir == UIDirection::Down {
                        if cur_idx != all_slots.len() - 1 {
                            self.controller.equipment = *all_slots[cur_idx + 1];
                        }
                    } else if dir == UIDirection::Up {
                        if cur_idx != 0 {
                            self.controller.equipment = *all_slots[cur_idx - 1];
                        }
                    }
                }
                true
            }
            Command::Confirm => {
                if self.controller.focus == self.character.inventory.id() {
                    let idx = &mut self.controller.inventory;
                    let character = &mut self.character;
                    if let Some(stored_item) = character.inventory.take(*idx as i32) {
                        match stored_item {
                            Item::Equipment(item) => {
                                // Equip item from inventory
                                if let Some(prev) = character.equip(item) {
                                    // Put the previous item back in inventory
                                    character.inventory.put(prev.into());
                                }
                            }
                            _ => {}
                        }
                    };
                }
                if self.controller.focus == self.character.id() {
                    let slot = &self.controller.equipment;
                    let character = &mut self.character;
                    if let Some(unequipped_item) = character.unequip(slot) {
                        character.inventory.put(unequipped_item.into());
                    };
                }

                true
            }
            Command::Unknown => true,
        }
    }

    /// Returns true while the game is running
    pub fn input(&mut self, key: char) -> bool {
        let command: Command = key.into();
        let ret = self.handle_command(command);
        self.last_key = key;
        ret
    }

    pub fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect) {
        Group::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .sizes(&[Size::Percent(50), Size::Percent(50)])
            .render(t, &area, |t, chunks| {
                self.character.render(t, &chunks[0], &self.controller);
                self.character.inventory.render(t, &chunks[1], &self.controller);
            });
    }
}