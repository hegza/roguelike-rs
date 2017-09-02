use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::scenes::GameScene;

pub struct Help {}
impl GameView for Help {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, _: &GameScene) {
        Paragraph::default()
            .text(&format!(
                "{}\n{}\n{}",
                &format!(
                    "i/j select option"
                ),
                &format!(
                    "e   confirm"
                ),
                &format!(
                    "h/l navigate windows"
                )
            ))
            .render(t, &area);
    }
}
