use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::GameState;
use rpglib::item::*;
use rpglib::display::*;

pub struct Character {}
impl GameView for Character {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState) {
        let ctrl = &state.controller;

        let focus = ctrl.focused() == "character";
        let selected = match focus {
            true => Some(ctrl.selected_idx("character")),
            false => None,
        };
        let (content, styles) = create_slot_list(
            selected,
            &state.character.equipment().inner().iter().map(|&(ref k, ref v)| (k, v.as_ref())).collect(),
        );
        let slots: Vec<(String, &Style)> =
            content.iter().map(|x| x.clone()).zip(styles.iter().map(|x| x)).collect();

        Group::default()
            .direction(Direction::Vertical)
            .margin(0)
            .sizes(&[Size::Percent(100)])
            .render(t, area, |t, chunks| {
                List::default()
                    .block(
                        Block::default().borders(border::ALL).title("List of equipped items"),
                    )
                    .items(&slots)
                    .render(t, &chunks[0]);
            });
    }
}

fn create_slot_list<'a>(
    selected: Option<usize>,
    slots: &Vec<(&Slot, Option<&'a Equipment>)>,
) -> (Vec<String>, Vec<Style>) {
    let mut content = Vec::with_capacity(slots.len());

    for &(slot, equip) in slots {
        let slot_name: &str = slot.into();
        let item_name = match equip {
            None => "<- empty ->".to_owned(),
            Some(equip) => equip.name(),
        };
        let list_item = format!("{}: {}", slot_name, item_name);
        content.push(list_item);
    }

    let mut styles = vec![Style::default().fg(Color::Yellow); slots.len()];
    if let Some(idx) = selected {
        styles[idx] = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
    }

    (content, styles)
}
