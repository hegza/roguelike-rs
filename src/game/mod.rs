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

pub struct GameState {
    ui: UI,
    scene: Scene,
    input_ticks: usize,
}

impl GameState {
    pub fn new() -> GameState {
        let scene;
        if super::RP {
            scene = Scene::game();
        } else {
            scene = Scene::Title;
        }
        GameState {
            ui: UI::new(),
            scene: scene,
            input_ticks: 0,
        }
    }

    pub fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect) {
        self.ui.render(t, area, self);
    }

    pub fn input(&mut self, input: char) -> bool {
        let ret = apply(input.into(), self);
        self.input_ticks += 1;
        ret
    }
}
