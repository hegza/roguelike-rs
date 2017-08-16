use rpglib::*;
use super::render::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use tui::style::*;
use std::cmp::*;

pub struct Game {
    pub last_key: char,
    character: Character,
    controller: Controller,
}

#[derive(PartialEq)]
pub enum Cursor {
    Item(usize),
}

pub struct Controller {
    pub cursor: Cursor,
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
        my_character.inventory.put(Box::new(item_1));
        my_character.inventory.put(Box::new(item_2));
        my_character.inventory.put(Box::new(item_3));

        Game {
            last_key: ' ',
            character: my_character,
            controller: Controller { cursor: Cursor::Item(0) },
        }
    }

    /// Returns true while the game is running
    pub fn input(&mut self, key: char) -> bool {
        match key {
            'q' => {
                self.last_key = key;
                return false;
            }
            'j' => {
                match self.controller.cursor {
                    Cursor::Item(ref mut idx) => {
                        let inventory = &self.character.inventory;
                        // Get bounds of item in next position
                        let (start, size) = inventory.bounds(*idx as i32 + 1);
                        // Move cursor to the end of the item in next position
                        *idx = start + size - 1;
                    }
                }
            }
            'k' => {
                match self.controller.cursor {
                    Cursor::Item(ref mut idx) => {
                        let inventory = &self.character.inventory;
                        // Get bounds of item in previous position
                        let (start, _) = inventory.bounds(*idx as i32 - 1);
                        // Move cursor to the start of the item in previous position
                        *idx = start;
                    }
                }
            }
            _ => {}
        }

        self.last_key = key;
        return true;

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
