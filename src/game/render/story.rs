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
            focused = Some(
                scene
                    .controller
                    .selected_idx_safe("story", scene.story.options().len() - 1),
            );
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
        StoryState::Encounter(ref encounter) => {
            match *encounter {
                Some(ref encounter) => {
                    let log_text = match encounter.combat.results {
                        Results::Begin { ref log, .. } |
                        Results::Round { ref log, .. } |
                        Results::End { ref log, .. } => log,
                    };

                    let max_log_width = (area.width - 8) as usize;
                    let log = fill(log_text, max_log_width);
                    title = format!(
                        "You have encountered {} {}",
                        encounter.monster.indefinite_article(),
                        encounter.monster.name()
                    );
                    paragraph = format!("{}", log);
                }
                None => {
                    title = "Safety".to_owned();
                    paragraph = "".to_owned()
                }
            }
            options = scene.story.options().iter().map(|x| x.into()).collect();
        }
        StoryState::OpenTreasure { ref items } => {
            title = "Safety".to_owned();
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
    // Render a red border if navigation is not free
    let list_border_style = {
        match scene.story.has_free_nav() {
            true => Style::default().fg(Color::Black),
            false => Style::default().fg(Color::Red),
        }
    };

    // Split in three vertically
    Group::default()
        .direction(Direction::Vertical)
        .margin(2)
        .sizes(&[Size::Percent(20), Size::Percent(50), Size::Percent(30)])
        .render(t, area, |t, chunks| {
            // 1st: status
            Block::default()
                .title(&title.to_sentence_case())
                .render(t, &chunks[0]);
            // 2nd: paragraph
            Paragraph::default().text(&paragraph).render(t, &chunks[1]);
            // 3rd: options
            List::default()
                .block(
                    Block::default()
                        .borders(border::ALL)
                        .border_style(list_border_style),
                )
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
