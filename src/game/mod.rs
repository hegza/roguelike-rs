pub mod command;
pub mod controller;
pub mod handle_input;

use rpglib::*;
use command::{Command, Direction as UIDirection};
use view::View;
use scenes::*;
pub use self::controller::*;
use self::handle_input::HandleInput;

pub struct Game {
    pub last_key: char,
    pub controller: Controller,
    pub character: Character,
    pub ticks: usize,
    scene: Scene,
}

pub enum Scene {
    Combat(CombatScene),
    OffCombat,
}

pub enum StateChange {
    // Scene should advance by one round
    Advance,
    // Game should changes scenes
    ChangeScene(Scene),
    // Equip an item
    Equip(Equipment),
    // Game state should not change
    Still,
    // Exit game
    Quit,
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
            character: my_character,
            controller: Controller::new(),
            ticks: 0,
            scene: Scene::OffCombat,
        }
    }

    /// Returns true while the game is running
    pub fn input(&mut self, key: char) -> bool {
        let command: Command = key.into();
        let state_change = self.handle_input(command);
        self.last_key = key;
        match state_change {
            StateChange::Quit => false,
            StateChange::Still => true,
            StateChange::Advance => {
                self.advance();
                true
            }
            StateChange::ChangeScene(scene) => {
                self.scene = scene;
                true
            }
            StateChange::Equip(item) => {
                let c = &mut self.character;
                // Equip the item, take previous item out
                if let Some(prev) = c.equip(item) {
                    // Put the previous item back in inventory
                    c.inventory.put(prev.into());
                }
                true
            }
        }
    }

    fn handle_input(&mut self, cmd: Command) -> StateChange {
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
            Command::MoveSelect(_) => {
                let inventory_id = {
                    let c = &self.character;
                    let inventory = &c.inventory;
                    inventory.id()
                };
                let character_id = {
                    let c = &self.character;
                    let ref_c = &c;
                    ref_c.id()
                };
                if self.controller.focus == inventory_id {
                    self.character.inventory.handle_input(cmd, &mut self.controller)
                } else if self.controller.focus == character_id {
                    self.character.handle_input(cmd, &mut self.controller)
                } else {
                    StateChange::Still
                }
            }
            Command::Confirm => {
                let inventory_id = {
                    let c = &self.character;
                    let inventory = &c.inventory;
                    inventory.id()
                };
                let character_id = {
                    let c = &self.character;
                    let ref_c = &c;
                    ref_c.id()
                };
                if self.controller.focus == inventory_id {
                    self.character
                        .inventory
                        .handle_input(cmd, &mut self.controller)
                } else if self.controller.focus == character_id {
                    self.character
                        .handle_input(cmd, &mut self.controller)
                } else {
                    StateChange::Still
                }
            }
            Command::Cheat(cheat) => {
                match cheat {
                    "combat_scene" => {
                        let monster = Monster::new("scene monster", 1, 10, None);
                        let scene = Scene::Combat(CombatScene::new(monster));
                        StateChange::ChangeScene(scene)
                    }
                    _ => StateChange::Still,
                }
            }
            _ => {
                let inventory_id = {
                    let c = &self.character;
                    let inventory = &c.inventory;
                    inventory.id()
                };
                let character_id = {
                    let c = &self.character;
                    let ref_c = &c;
                    ref_c.id()
                };
                let ctrl = &mut self.controller;
                if ctrl.focus == inventory_id {
                    self.character.inventory.handle_input(cmd, ctrl)
                } else if ctrl.focus == character_id {
                    self.character.handle_input(cmd, ctrl)
                } else {
                    StateChange::Still
                }
            }
        }
    }

    fn advance(&mut self) {
        let scene = &mut self.scene;
        match scene {
            &mut Scene::Combat(ref mut scene) => {
                let results = scene.combat
                    .apply_round(&mut self.character, &mut scene.monster);
                for log in results.english_log {
                    println!("{}", log);
                }
            }
            &mut Scene::OffCombat => {}
        };
        self.ticks += 1;
    }
}
