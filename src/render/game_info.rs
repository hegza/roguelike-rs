use super::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use rpglib::*;

pub struct GameInfo<'a> {
    character: &'a Character,
    ticks: &'a usize,
}

impl<'a> GameInfo<'a> {
    pub fn new(character: &'a Character, ticks: &'a usize) -> GameInfo<'a> {
        GameInfo {
            character: character,
            ticks: ticks,
        }
    }
}

impl<'a> Render for GameInfo<'a> {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, _: &Controller) {
        Paragraph::default()
            .text(&format!("{}\n{}\n{}",
                           &format!("Life:    {:>8}",
                                    format!("{}/{}",
                                            self.character.life(),
                                            self.character.attributes[&Attribute::MaxLife])),
                           &format!("Stamina: {:>8}", "? (?)"),
                           &format!("Ticks:   {:>8}", self.ticks)))
            .render(t, &area);
    }
}
