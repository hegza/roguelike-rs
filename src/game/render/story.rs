use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use rpglib::*;
use textwrap::fill;
use game::scenes::*;

pub struct Story;
impl GameView for Story {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, scene: &GameScene) {
        let mut focused = None;
        if scene.controller.focused() == "story" {
            focused = Some(scene.controller.selected_idx("story"));
        }

        render_combat(t, area, scene, focused);
    }
}

fn render_combat(t: &mut Terminal<TermionBackend>, area: &Rect,
        scene: &GameScene, focused: Option<usize>) {
    let options: Vec<(String, Style)> = create_options(focused, scene.story.options().as_slice());

    let title: String;
    let paragraph: String;

    match scene.story {
        StoryState::CombatEncounter {ref combat, ref monster} => {
            let log_text = match combat.results {
                Results::Begin { ref log, .. } |
                Results::Round { ref log, .. } |
                Results::End { ref log, .. } => log,
            };

            let max_log_width = (area.width - 8) as usize;
            let log = fill(log_text, max_log_width);
            title = format!("Fight the {}", monster.name());
            paragraph = format!("{}", log);
        },
        StoryState::OpenTreasure{..} => {
            title = "Treasure".to_owned();
            paragraph = String::new();
        },
        StoryState::Final => {
            title = "You win!".to_owned();
            paragraph = String::new();
        }
    }

    // Split in three vertically
    Group::default()
        .direction(Direction::Vertical)
        .margin(2)
        .sizes(&[Size::Percent(34), Size::Percent(33), Size::Percent(33)])
        .render(t, area, |t, chunks| {
            // 1st: status
            Block::default()
                .title(&title)
                .render(t, &chunks[0]);
            // 2nd: paragraph
            Paragraph::default()
                .text(&paragraph).render(t, &chunks[1]);
            // 3rd: options
            List::default()
                .items(&options.iter().map(|&(ref o, ref s)| (o.clone(), s)).collect::<Vec<(String, &Style)>>())
                .render(t, &chunks[2]);
        });

}

fn create_options(selected_idx: Option<usize>, options: &[String]) -> Vec<(String, Style)> {
    let default_style = Style::default().fg(Color::Yellow);
    let hilight_style = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
    let mut styles = vec![default_style; options.len()];
    if let Some(idx) = selected_idx {
        styles[idx] = hilight_style;
    }
    options.iter().map(|o| o.clone()).zip(styles).collect()
}
