mod character;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::*;
use super::game::Controller;

pub trait Render {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller);
}
