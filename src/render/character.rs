use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::super::*;
use rpglib::*;
use super::*;

impl Render for Character {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, controller: &Controller) {
        /*
        for (slot, equip) in self.equipped_items() {}

        Group::default()
            .direction(Direction::Vertical)
            .borders(border::ALL)
            .render(t, area, |t, chunks| {
                List::default()
                    .block(Block::default().borders(border::ALL).title("List"))
                    .items()
                    .render(t, &chunks[1]);
            });
        */
    }
}

fn create_item_list(inventory: &Inventory) -> Vec<String> {
    enum State {
        LargeItem(usize, usize),
        Empty,
    }
    let none_str = "<-     free     ->";

    let mut items = vec![];
    let mut state = State::Empty;
    for pos in 0..inventory.capacity() {
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
    items
}

impl Render for Inventory {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, controller: &Controller) {
        let items = create_item_list(&self);
        let mut focus = false;
        let select_index = match controller.cursor {
            Cursor::Item(idx) => {
                focus = true;
                idx
            }
            _ => 0,
        };

        let highlight_symbol;
        if focus {
            highlight_symbol = ">";
        } else {
            highlight_symbol = "o";
        }
        // TODO: change based on focus
        SelectableList::default()
            .block(Block::default().borders(border::ALL).title("Inventory"))
            .items(&items)
            .select(select_index)
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::Bold))
            .highlight_symbol(highlight_symbol)
            .render(t, area);
    }
}
