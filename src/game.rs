use rpglib::*;
use super::render::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::*;
use super::ui::{Command, View, Direction as UIDirection};
use render::game_info::GameInfo;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Game {
    pub last_key: char,
    character: Rc<RefCell<Character>>,
    pub controller: Controller,
    ticks: usize,
}

pub struct Controller {
    pub inventory: usize,
    pub equipment: ItemSlot,
    pub focus: usize, // Id of view
    // TODO: focus() -> &View
    max_views: usize,
}

enum StateChange {
    // Game should advance by one round
    Advance,
    // Game state should not change
    Still,
    // Exit game
    Quit,
}

impl Controller {
    fn new() -> Controller {
        Controller {
            inventory: 0,
            equipment: ItemSlot::MainHand,
            focus: 0,
            max_views: 2,
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
        let item_4 = Consumable {
            effects: vec![],
            english_name: "Food ration".to_owned(),
            size: 1,
        };
        my_character.inventory.put(item_1.into());
        my_character.inventory.put(item_2.into());
        my_character.inventory.put(item_3.into());
        my_character.inventory.put(item_4.into());

        Game {
            last_key: ' ',
            character: Rc::new(RefCell::new(my_character)),
            controller: Controller::new(),
            ticks: 0,
        }
    }

    fn handle_command(&mut self, cmd: Command) -> StateChange {
        match cmd {
            Command::Quit => StateChange::Quit,
            Command::Nav(dir) => {
                match dir {
                    UIDirection::Right => {
                        if self.controller.focus != self.controller.max_views - 1 {
                            self.controller.focus += 1;
                        }
                    }
                    UIDirection::Left => {
                        if self.controller.focus > 0 {
                            self.controller.focus -= 1;
                        }
                    }
                    _ => {}
                }
                StateChange::Still
            }
            Command::MoveSelect(dir) => {
                if self.controller.focus == self.character.borrow().inventory.id() {
                    let idx = &mut self.controller.inventory;
                    if dir == UIDirection::Down {
                        let inventory = &self.character.borrow().inventory;
                        // Get bounds of item in current position
                        let (start, size) = inventory.bounds(*idx as i32);
                        if start + size != inventory.capacity() {
                            // Move cursor below the current item
                            *idx = start + size;
                        }
                    } else if dir == UIDirection::Up {
                        let inventory = &self.character.borrow().inventory;
                        // Get bounds of item in previous position
                        let (start, _) = inventory.bounds(*idx as i32 - 1);
                        // Move cursor to the start of the item in previous position
                        *idx = start;
                    }
                } else if self.controller.focus == self.character.borrow().id() {
                    let character = self.character.borrow();
                    let mut all_slots: Vec<&ItemSlot> =
                        character.equipped_items().iter().map(|(k, _)| k).collect();
                    all_slots.sort();

                    let cur_idx = all_slots.iter()
                        .position(|x| *x == &self.controller.equipment)
                        .expect("a non-existing slot should not be selected");
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
                StateChange::Still
            }
            Command::Confirm => {
                if self.controller.focus == self.character.borrow().inventory.id() {
                    let idx = &mut self.controller.inventory;
                    let character = &mut self.character.borrow_mut();
                    if let Some(item) = character.inventory.take(*idx as i32) {
                        match item {
                            Item::Equipment(equip) => {
                                // Equip the item, take previous item out
                                if let Some(prev) = character.equip(equip) {
                                    // Put the previous item back in inventory
                                    character.inventory.put(prev.into());
                                }
                            }
                            _ => {
                                // Put the item back into the inventory
                                character.inventory.put_at(item, *idx);
                            }
                        }
                    };
                }
                if self.controller.focus == self.character.borrow().id() {
                    let slot = &self.controller.equipment;
                    let character = &mut self.character.borrow_mut();
                    if let Some(unequipped_item) = character.unequip(slot) {
                        character.inventory.put(unequipped_item.into());
                    };
                }

                StateChange::Still
            }
            Command::Unknown => StateChange::Still,
        }
    }

    /// Returns true while the game is running
    pub fn input(&mut self, key: char) -> bool {
        let command: Command = key.into();
        let state_change = self.handle_command(command);
        self.last_key = key;
        match state_change {
            StateChange::Quit => false,
            StateChange::Still => true,
            StateChange::Advance => {
                self.advance();
                true
            }
        }
    }

    fn advance(&mut self) {
        self.ticks += 1;
    }

    pub fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect) {
        // Create game info view
        let game_info = GameInfo::new(&self.ticks);

        // Split the view in two horizontally
        Group::default()
            .margin(0)
            .direction(Direction::Horizontal)
            .sizes(&[Size::Percent(50), Size::Percent(50)])
            .render(t, &area, |t, chunks| {
                // Split the left view in two vertically
                Group::default()
                    .margin(0)
                    .direction(Direction::Vertical)
                    .sizes(&[Size::Percent(50), Size::Percent(50)])
                    .render(t, &chunks[0], |_, _| {});
                // Split the right view in three vertically
                Group::default()
                    .margin(0)
                    .direction(Direction::Vertical)
                    .sizes(&[Size::Percent(50), Size::Percent(25), Size::Percent(25)])
                    .render(t, &chunks[1], |t, chunks| {
                        self.character.borrow().inventory.render(t, &chunks[0], &self.controller);
                        self.character.borrow().render(t, &chunks[1], &self.controller);
                        game_info.render(t, &chunks[2], &self.controller);
                    });

            });
    }
}
