use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use render::Render;
use controller::*;
use game::Scene;
use scenes::*;
use rpglib::*;

pub struct Map;

impl Render for Map {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller) {
        Paragraph::default()
            .block(Block::default().borders(border::ALL).title("World Map"))
            .text(&format!("This is supposed to be a map:\n\n{}", "o---------o"))
            .render(t, &area);
    }
}
