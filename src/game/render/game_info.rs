use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use rpglib::{Attribute, Combatant};
use game::scenes::GameScene;

pub struct GameInfo {}
impl GameView for GameInfo {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameScene) {
        Paragraph::default()
            .text(&format!(
                "{}\n{}\n{}",
                &format!(
                    "Life:    {:>8}",
                    format!(
                        "{}/{}",
                        state.player.life(),
                        state.player.attribute(&Attribute::Constitution)
                    )
                ),
                &format!(
                    "Stamina: {:>8}",
                    format!("? ({})", state.player.attribute(&Attribute::Endurance))
                ),
                &format!("Ticks:   {:>8}", &"?")
            ))
            .render(t, &area);
    }
}
