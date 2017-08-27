use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::GameState;
use game::scenes::*;
use rpglib::*;
use textwrap::fill;

pub struct Story;
impl GameView for Story {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState) {
        match state.scene {
            Scene::Combat(ref scene) => {
                let mut focused = None;
                if state.controller.focused() == "story" {
                    focused = Some(state.controller.selected_idx("story"));
                }

                render_combat(t, area, state, scene, focused);
            }
            _ => {}
        };
    }
}

fn render_combat(
    t: &mut Terminal<TermionBackend>,
    area: &Rect,
    state: &GameState,
    scene: &CombatScene,
    focused: Option<usize>,
) {
    let ctrl = &state.controller;
    let log_text = match &scene.combat.results {
        &Results::Begin { ref log, .. } |
        &Results::Round { ref log, .. } |
        &Results::End { ref log, .. } => log,
    };
    // TODO: check this from size
    const MAX_LOG_WIDTH: usize = 20;
    let log = fill(log_text, MAX_LOG_WIDTH);
    let combat_options_owned = create_combat_options(focused);
    let combat_options: Vec<(String, &Style)> =
        combat_options_owned.iter().map(|&(ref o, ref s)| (o.clone(), s)).collect();

    Block::default()
        .borders(border::ALL)
        .title(&format!("Fight the {}", scene.monster.name()))
        .render(t, area);
    // Split in three vertically
    Group::default()
        .direction(Direction::Vertical)
        .margin(4)
        .sizes(&[Size::Percent(34), Size::Percent(33), Size::Percent(33)])
        .render(t, area, |t, chunks| {
            // 1st: status
            // 2nd: paragraph
            Paragraph::default().text(&log).render(t, &chunks[1]);
            // 3rd: options
            List::default().items(&combat_options).render(t, &chunks[2]);
        });
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
