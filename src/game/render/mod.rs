pub mod inventory;
pub mod character;
pub mod game_info;
pub mod map;
pub mod story;

use game::*;
use tui::terminal::Terminal;
use tui::backend::TermionBackend;
use tui::layout::{Direction, Size, Rect, Group};
use self::inventory::InventoryWidget;
use self::character::Character;
use self::game_info::GameInfo;
use self::map::Map;
use self::story::Story;

pub struct UI {
    root: ViewNode,
}

impl UI {
    pub fn new() -> UI {
        use self::ViewNode::*;
        use tui::layout::Direction::*;
        use tui::layout::Size::*;

        let story = Box::new(Display(Box::new(Story {})));
        let inventory = Box::new(Display(Box::new(InventoryWidget {})));
        let character = Box::new(Display(Box::new(Character {})));
        let game_info = Box::new(Display(Box::new(GameInfo {})));
        let map = Box::new(Display(Box::new(Map {})));

        let root = Split {
            direction: Horizontal,
            sizes: vec![Percent(50), Percent(50)],
            nodes: vec![Box::new(Split {
                            direction: Vertical,
                            sizes: vec![Percent(67), Percent(33)],
                            nodes: vec![story, map],
                        }),
                        Box::new(Split {
                            direction: Vertical,
                            sizes: vec![Percent(50), Percent(25), Percent(25)],
                            nodes: vec![inventory, character, game_info],
                        })],
        };
        UI { root: root }
    }
    pub fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState) {
        self.root.render(t, area, state);
    }
}

trait GameView {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState);
}

enum ViewNode {
    Split {
        direction: Direction,
        sizes: Vec<Size>,
        nodes: Vec<Box<ViewNode>>,
    },
    Display(Box<GameView>), 
    // TODO: SwapDisplay(),
}

impl ViewNode {
    fn render(&self, t: &mut Terminal<TermionBackend>, area: &Rect, state: &GameState) {
        use self::ViewNode::*;
        match self {
            &Split { ref direction, ref sizes, ref nodes } => {
                Group::default()
                    .margin(0)
                    .direction(direction.clone())
                    .sizes(&sizes)
                    .render(t, area, |t, chunks| for i in 0..nodes.len() {
                        nodes[i].render(t, &chunks[i], state);
                    });
            }
            &Display(ref view) => {
                view.render(t, area, state);
            }
        }
    }
}
