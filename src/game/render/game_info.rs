use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::GameState;
use rpglib::Attribute;
use rpglib::combat::*;

pub struct GameInfo {}
impl GameView for GameInfo {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState) {
        Paragraph::default()
            .text(&format!("{}\n{}\n{}",
                           &format!("Life:    {:>8}",
                                    format!("{}/{}",
                                            state.character.life(),
                                            state.character.attribute(&Attribute::Constitution))),
                           &format!("Stamina: {:>8}", format!("? ({})",
                                state.character.attribute(&Attribute::Endurance))),
                           &format!("Ticks:   {:>8}", state.ticks)))
            .render(t, &area);
    }
}
