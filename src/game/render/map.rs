use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::scenes::GameScene;

pub struct Map {}
impl GameView for Map {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, _: &GameScene) {
        Paragraph::default()
            .block(Block::default().borders(border::ALL).title("World Map"))
            .text(&format!(
                "This is supposed to be a map:\n\n{}",
                "o---------o"
            ))
            .render(t, &area);
    }
}
