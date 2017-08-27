mod handle_input;
mod render;
mod scenes;
mod controller;

use rpglib::*;
use tui::terminal::Terminal;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use self::render::UI;
use self::handle_input::apply;
use self::scenes::*;
use self::controller::*;

pub struct GameState {
    controller: Controller,
    ui: UI,
    character: Character,
    scene: Scene,
    ticks: usize,
}

impl GameState {
    pub fn new() -> GameState {
        let mut attributes = CharacterAttributes::default();
        attributes.set(Attribute::Constitution, 8);
        let mut my_character = CharacterBuilder::new(2, 12, &attributes).named("hegza").build();
        let item_1 = equipment("Long Sword", 4, Slot::Hand, vec![]).damage(2).build();
        let item_2 = equipment("Stone", 3, Slot::Hand, vec![]).build();
        let item_3 = equipment("Short Sword", 2, Slot::Hand, vec![]).damage(1).build();
        let item_4 = consumable("Food Ration", 1, vec![]).build();

        my_character.inventory.put(item_1.into());
        my_character.inventory.put(item_2.into());
        my_character.inventory.put(item_3.into());
        my_character.inventory.put(item_4.into());

        GameState {
            controller: Controller::new(&vec!["story", "inventory", "character"]),
            ui: UI::new(),
            character: my_character,
            scene: Scene::OffCombat,
            ticks: 0,
        }
    }

    pub fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect) {
        self.ui.render(t, area, self);
    }

    pub fn input(&mut self, input: char) -> bool {
        apply(input.into(), self)
    }
}
