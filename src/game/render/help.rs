use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::scenes::GameScene;

pub struct Help {}
impl GameView for Help {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, scene: &GameScene) {
        let guides: Vec<(String, String)> = scene.story.active_guides();
        let guide_text: String = guides.iter()
            .map(|&(ref keys, ref description)| format!("{:<8} {:>8}", keys, description)).collect::<Vec<String>>().join("\n");
        Paragraph::default()
            .text(&guide_text)
            .render(t, &area);
    }
}
