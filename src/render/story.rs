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
use view::View;

pub struct Story<'a> {
    pub current_scene: &'a Scene,
}

impl<'a> Render for Story<'a> {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller) {
        match self.current_scene {
            &Scene::Combat(ref scene) => {
                scene.render(t, area, ctrl);
            }
            _ => {}
        };
    }
}

impl Render for CombatScene {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller) {
        let lines: &Vec<String> = &self.combat
            .last_results
            .english_log;
        let log: String = lines.join("\n");
        let mut focused = None;
        if ctrl.focus == self.id() {
            focused = Some(ctrl.story_option);
        }
        let combat_options_owned = create_combat_options(focused);
        let combat_options: Vec<(String, &Style)> = combat_options_owned.iter()
            .map(|&(ref o, ref s)| (o.clone(), s))
            .collect();

        Block::default()
            .borders(border::ALL)
            .title(&format!("Fight the {}", self.monster.english_name()))
            .render(t, area);
        // Split in three vertically
        Group::default()
            .direction(Direction::Vertical)
            .margin(4)
            .sizes(&[Size::Percent(34), Size::Percent(33), Size::Percent(33)])
            .render(t, area, |t, chunks| {
                // 1st: status
                // 2nd: paragraph
                Paragraph::default()
                    .text(&log)
                    .render(t, &chunks[1]);
                // 3rd: options
                List::default()
                    .items(&combat_options)
                    .render(t, &chunks[2]);
            });
    }
}

fn create_combat_options(selected_idx: Option<usize>) -> Vec<(String, Style)> {
    let mut options = vec![];

    options.push("Attack".to_owned());

    let default_style = Style::default().fg(Color::Yellow);
    let hilight_style = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
    let mut styles = vec![default_style; options.len()];
    if let Some(idx) = selected_idx {
        styles[idx] = hilight_style;
    }

    options.iter().map(|o| o.clone()).zip(styles).collect()
}
