use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use rpglib::*;
use textwrap::fill;
use game::scenes::*;
use game::scenes::game_scene::story_option::*;

lazy_static!{
    static ref DEFAULT_STYLE: Style = Style::default().fg(Color::Yellow);
    static ref HILIGHT_STYLE: Style = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
}

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

fn render_combat(
    t: &mut Terminal<TermionBackend>,
    area: &Rect,
    scene: &GameScene,
    focused: Option<usize>,
) {
    let title: String;
    let paragraph: String;
    let options: Vec<String>;

    match scene.story {
        StoryState::CombatEncounter {
            ref combat,
            ref monster,
        } => {
            let log_text = match combat.results {
                Results::Begin { ref log, .. } |
                Results::Round { ref log, .. } |
                Results::End { ref log, .. } => log,
            };

            let max_log_width = (area.width - 8) as usize;
            let log = fill(log_text, max_log_width);
            title = format!("Fight the {}", monster.name());
            paragraph = format!("{}", log);
            options = scene.story.options().iter().map(|x| x.into()).collect();
        }
        StoryState::OpenTreasure { ref items } => {
            title = "Treasure".to_owned();
            paragraph = String::new();
            options = scene
                .story
                .options()
                .iter()
                .map(|o| match *o {
                    StoryOption::PickUp(item_idx) => format!("Pick up {}", items[item_idx].name()),
                    ref s => s.into(),
                })
                .collect();
        }
        StoryState::Final => {
            title = "You win!".to_owned();
            paragraph = String::new();
            options = scene.story.options().iter().map(|x| x.into()).collect();
        }
    }

    let options_list: Vec<(String, &Style)> = create_options_list(focused, &options.as_slice());

    // Split in three vertically
    Group::default()
        .direction(Direction::Vertical)
        .margin(2)
        .sizes(&[Size::Percent(34), Size::Percent(33), Size::Percent(33)])
        .render(t, area, |t, chunks| {
            // 1st: status
            Block::default().title(&title).render(t, &chunks[0]);
            // 2nd: paragraph
            Paragraph::default().text(&paragraph).render(t, &chunks[1]);
            // 3rd: options
            List::default()
                .items(&options_list.as_slice())
                .render(t, &chunks[2]);
        });
}

fn create_options_list(selected_idx: Option<usize>, options: &[String]) -> Vec<(String, &Style)> {
    let mut styles: Vec<&Style> = vec![&DEFAULT_STYLE; options.len()];
    if let Some(idx) = selected_idx {
        styles[idx] = &HILIGHT_STYLE;
    }
    options.iter().map(|o| o.clone()).zip(styles).collect()
}
