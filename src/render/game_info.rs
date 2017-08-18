use super::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;

pub struct GameInfo<'a> {
    ticks: &'a usize,
}

impl<'a> GameInfo<'a> {
    pub fn new(ticks: &usize) -> GameInfo {
        GameInfo { ticks: ticks }
    }
}

impl<'a> Render for GameInfo<'a> {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, _: &Controller) {
        Paragraph::default().text(&format!("Ticks: {}", self.ticks)).render(t, &area);
    }
}
