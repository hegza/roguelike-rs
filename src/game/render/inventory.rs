use tui::style::*;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::*;
use tui::layout::*;
use super::GameView;
use game::GameState;
use rpglib::*;

pub struct InventoryWidget {}
impl GameView for InventoryWidget {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState) {
        let controller = &state.controller;
        let focus = controller.focused() == "inventory";
        let select_index = controller.selected_idx("inventory");
        let hilight = match focus {
            true => Some(select_index),
            false => None,
        };

        let (content, styles) = create_item_list(hilight, &state.character.inventory);
        let items: Vec<(String, &Style)> =
            content.iter().map(|x| x.clone()).zip(styles.iter().map(|x| x)).collect();

        List::default()
            .block(Block::default().borders(border::ALL).title("Inventory"))
            .items(&items)
            .render(t, area);
    }
}

fn create_item_list(
    selected_idx: Option<usize>,
    inventory: &Inventory,
) -> (Vec<String>, Vec<Style>) {
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
            State::Empty => match inventory.get(pos as i32) {
                None => {
                    items.push(none_str.to_owned());
                }
                Some(item) => {
                    let name = item.name();
                    let display = match item.size() {
                        1 => format!("(:{}:)", name.clone()),
                        _ => {
                            state = State::LargeItem(item.size() - 1, name.chars().count());
                            format!("/´{}`\\", name.clone())
                        }
                    };
                    items.push(display);
                }
            },
            State::LargeItem(1, letters) => {
                items.push(format!(
                    "\\{}/",
                    (0..letters + 2).map(|_| "_").collect::<String>()
                ));
                state = State::Empty;
            }
            State::LargeItem(ref mut size, letters) => {
                items.push(format!(
                    "|{}|",
                    (0..letters + 2).map(|_| " ").collect::<String>()
                ));
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
