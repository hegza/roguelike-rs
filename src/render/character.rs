use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use rpglib::*;
use super::*;
use view::View;

fn create_slot_list<'a>(selected: Option<&ItemSlot>,
                        slots: &Vec<(&ItemSlot, Option<&'a Equipment>)>)
                        -> (Vec<String>, Vec<Style>) {
    let mut sorted_slots = slots.clone();
    sorted_slots.sort_by_key(|k| k.0);
    let ss: Vec<&(&ItemSlot, Option<&'a Equipment>)> = sorted_slots.iter().collect();

    let mut content = Vec::with_capacity(slots.len());

    for &(slot, equip) in ss {
        let slot_name: &str = slot.into();
        let item_name = match equip {
            None => "<- empty ->".to_owned(),
            Some(equip) => equip.english_name(),
        };
        let list_item = format!("{}: {}", slot_name, item_name);
        content.push(list_item);
    }

    let mut styles = vec![Style::default().fg(Color::Yellow); slots.len()];
    if let Some(sel) = selected {
        // Find position of selected slot
        let idx = sorted_slots.iter()
            .map(|&(k, _)| k)
            .position(|slot| slot == sel)
            .expect("a slot that does not exist should not be selected");
        styles[idx] = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
    }

    (content, styles)
}

impl Render for Character {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller) {
        let focus = ctrl.focus == self.id();
        let selected = match focus {
            true => Some(&ctrl.equipment),
            false => None,
        };
        let (content, styles) =
            create_slot_list(selected,
                             &self.equipped_items().iter().map(|(k, v)| (k, v.as_ref())).collect());
        let slots: Vec<(String, &Style)> =
            content.iter().map(|x| x.clone()).zip(styles.iter().map(|x| x)).collect();

        Group::default()
            .direction(Direction::Vertical)
            .margin(0)
            .sizes(&[Size::Percent(100)])
            .render(t, area, |t, chunks| {
                List::default()
                    .block(Block::default().borders(border::ALL).title("List of equipped items"))
                    .items(&slots)
                    .render(t, &chunks[0]);
            });
    }
}
