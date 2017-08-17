use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use rpglib::*;
use super::*;
use super::super::ui::View;

fn create_slot_list<'a>(selected: &ItemSlot,
                        slots: &Vec<(&ItemSlot, Option<&'a Equipment>)>)
                        -> (Vec<String>, Vec<Style>) {
    let mut sorted_slots = slots.clone();
    sorted_slots.sort_by_key(|k| k.0);
    let ss: Vec<&(&ItemSlot, Option<&'a Equipment>)> = sorted_slots.iter().collect();

    let mut content = Vec::with_capacity(slots.len());
    let mut styles = Vec::with_capacity(slots.len());

    for &(slot, equip) in ss {
        let slot_name: &str = slot.into();
        let item_name = match equip {
            None => "<- empty ->".to_owned(),
            Some(equip) => equip.english_name(),
        };
        let list_item = format!("{}: {}", slot_name, item_name);
        let mut style = Style::default().fg(Color::Yellow);
        if slot == selected {
            style = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
        }
        content.push(list_item);
        styles.push(style);
    }
    (content, styles)
}

impl View for Character {
    fn id(&self) -> usize {
        0
    }
}

impl Render for Character {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, ctrl: &Controller) {
        let (content, styles) =
            create_slot_list(&ctrl.equipment,
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

fn create_item_list(selected_idx: Option<usize>,
                    inventory: &Inventory)
                    -> (Vec<String>, Vec<Style>) {
    enum State {
        LargeItem(usize, usize),
        Empty,
    }
    let none_str = "<-     free     ->";

    let mut items = vec![];
    let mut state = State::Empty;
    for pos in 0..inventory.capacity() {
        // Collect items
        match state {
            State::Empty => {
                match inventory.get(pos as i32) {
                    None => {
                        items.push(none_str.to_owned());
                    }
                    Some(item) => {
                        // TODO: use letter count
                        let name = item.english_name();
                        items.push(format!("/´{}`\\", name.clone()));
                        if item.size() > 1 {
                            state = State::LargeItem(item.size() - 1, name.chars().count());
                        }
                    }
                }
            }
            State::LargeItem(1, letters) => {
                items.push(format!("\\{}/", (0..letters + 2).map(|_| "_").collect::<String>()));
                state = State::Empty;
            }
            State::LargeItem(ref mut size, letters) => {
                items.push(format!("|{}|", (0..letters + 2).map(|_| " ").collect::<String>()));
                *size -= 1;
            }
        }
    }

    let mut styles = vec![Style::default().fg(Color::Yellow); inventory.capacity()];
    if let Some(idx) = selected_idx {
        // Hilight selected items
        let (sel_start, sel_size) = inventory.bounds(idx as i32);
        for pos in sel_start..sel_start + sel_size {
            styles[pos] = Style::default().fg(Color::Yellow).modifier(Modifier::Bold);
        }
    }

    (items, styles)
}

impl View for Inventory {
    fn id(&self) -> usize {
        1
    }
}

impl Render for Inventory {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, controller: &Controller) {
        let focus = controller.focus == self.id();
        let select_index = controller.inventory;
        let hilight = match focus {
            true => Some(select_index),
            false => None,
        };

        let (content, styles) = create_item_list(hilight, &self);
        let items: Vec<(String, &Style)> =
            content.iter().map(|x| x.clone()).zip(styles.iter().map(|x| x)).collect();

        List::default()
            .block(Block::default().borders(border::ALL).title("Inventory"))
            .items(&items)
            .render(t, area);
    }
}
